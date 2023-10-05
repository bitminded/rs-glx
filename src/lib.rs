extern crate xlib;

pub mod cdef;
pub mod constants;

pub type GLXWindow = cdef::GLXWindow;
pub type GLXDrawable = cdef::GLXDrawable;

pub struct DoNotFree<T> {
    data: *mut T,
}

impl<T> std::ops::Deref for DoNotFree<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<T> std::ops::DerefMut for DoNotFree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.data }
    }
}

pub struct GLXFBConfig<'a> {
    data: *mut cdef::GLXFBConfig,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

// A smart pointer for a list of pointers to GLXFBConfig.
pub struct GLXFBConfigList<'a> {
    data: *mut *mut cdef::GLXFBConfig,
    transformed: Vec<GLXFBConfig<'a>>,
}

impl<'a> GLXFBConfigList<'a> {
    fn new(data: *mut *mut cdef::GLXFBConfig, length: usize) -> Self {
        let mut configs = Vec::new();
        let slice = unsafe { std::slice::from_raw_parts(data, length) };
        for config in slice {
            configs.push(GLXFBConfig::<'a> {
                data: *config,
                _lifetime: std::marker::PhantomData,
            });
        }

        Self {
            data: data,
            transformed: configs,
        }
    }

    pub fn get(&self, index: usize) -> &'a GLXFBConfig {
        &self.transformed[index]
    }

    pub fn len(&self) -> usize {
        self.transformed.len()
    }
}

impl<'a> Drop for GLXFBConfigList<'a> {
    fn drop(&mut self) {
        unsafe {
            xlib::cdef::XFree(std::mem::transmute::<
                *mut *mut cdef::GLXFBConfig,
                *mut std::ffi::c_void,
            >(self.data));
        }
    }
}

pub struct GLXContext {
    data: *mut cdef::GLXContext,
}

pub fn gl_x_choose_fb_config<'a>(
    display: &'a xlib::DoNotFree<xlib::cdef::Display>,
    screen: i32,
    attrib_list: &'_ [i32],
) -> GLXFBConfigList<'a> {
    unsafe {
        let display = &**display as *const xlib::cdef::Display as *mut xlib::cdef::Display;
        let display = display as *mut xlib::cdef::Display;
        let attribs = attrib_list.as_ptr();
        let mut length: i32 = 0;
        let configs = cdef::glXChooseFBConfig(display, screen, attribs, &mut length);
        GLXFBConfigList::new(std::mem::transmute(configs), length as usize)
    }
}

pub fn gl_x_get_fb_config_attrib(
    display: &xlib::DoNotFree<xlib::cdef::Display>,
    config: &GLXFBConfig,
    attribute: i32,
    value: &mut i32,
) -> i32 {
    unsafe {
        let display = &**display as *const xlib::cdef::Display as *mut xlib::cdef::Display;
        cdef::glXGetFBConfigAttrib(display, config.data, attribute, value)
    }
}

pub fn gl_x_create_new_context(
    display: &xlib::DoNotFree<xlib::cdef::Display>,
    config: &GLXFBConfig,
    render_type: i32,
    share_list: Option<&GLXContext>,
    direct: bool,
) -> GLXContext {
    unsafe {
        let display = &**display as *const xlib::cdef::Display as *mut xlib::cdef::Display;
        let direct = if direct { 1 } else { 0 };
        let context = match share_list {
            Some(share_list) => {
                let share_list = share_list.data;
                cdef::glXCreateNewContext(display, config.data, render_type, share_list, direct)
            }
            None => cdef::glXCreateNewContext(
                display,
                config.data,
                render_type,
                std::ptr::null_mut(),
                direct,
            ),
        };
        GLXContext { data: context }
    }
}

// pub fn gl_x_doestroy_context()

pub fn gl_x_create_window(
    display: &xlib::DoNotFree<xlib::cdef::Display>,
    config: &GLXFBConfig,
    window: xlib::Window,
    attrib_list: Option<&[i32]>,
) -> GLXWindow {
    unsafe {
        let display = &**display as *const xlib::cdef::Display as *mut xlib::cdef::Display;
        let attrib_list = match attrib_list {
            Some(attrib_list) => attrib_list.as_ptr(),
            None => std::ptr::null(),
        };
        cdef::glXCreateWindow(display, config.data, window, attrib_list)
    }
}

// pub fn gl_x_destroy_window()

pub fn gl_x_make_context_current(
    display: &xlib::DoNotFree<xlib::cdef::Display>,
    draw: GLXDrawable,
    read: GLXDrawable,
    context: &GLXContext,
) -> i32 {
    unsafe {
        let display = &**display as *const xlib::cdef::Display as *mut xlib::cdef::Display;
        cdef::glXMakeContextCurrent(display, draw, read, context.data)
    }
}

pub fn gl_x_swap_buffers(display: &xlib::DoNotFree<xlib::cdef::Display>, drawable: GLXDrawable) {
    unsafe {
        let display = &**display as *const xlib::cdef::Display as *mut xlib::cdef::Display;
        cdef::glXSwapBuffers(display, drawable)
    }
}

pub fn gl_x_get_current_drawable() -> GLXDrawable {
    unsafe { cdef::glXGetCurrentDrawable() }
}
