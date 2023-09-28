extern crate xlib;

use std::ffi::c_int;

pub type GLXWindow = xlib::XID;
pub type GLXDrawable = xlib::XID;

#[repr(C)]
pub struct GLXFBConfig {
    opaque: [u8; 0],
}

#[repr(C)]
pub struct GLXContext {
    opaque: [u8; 0],
}

#[link(name = "GLX")]
extern "system" {
    /// Use XFree to free the data returned
    pub fn glXChooseFBConfig(
        dpy: *mut xlib::cdef::Display,
        screen: c_int,
        attrib_list: *const c_int,
        nitems: *mut c_int,
    ) -> *mut *mut GLXFBConfig;
    pub fn glXGetFBConfigAttrib(
        dpy: *mut xlib::cdef::Display,
        config: *mut GLXFBConfig,
        attribute: c_int,
        value: *mut c_int,
    ) -> c_int;
    pub fn glXCreateNewContext(
        dpy: *mut xlib::cdef::Display,
        config: *mut GLXFBConfig,
        render_type: c_int,
        share_list: *mut GLXContext,
        direct: c_int,
    ) -> *mut GLXContext;
    pub fn glXCreateWindow(
        dpy: *mut xlib::cdef::Display,
        config: *mut GLXFBConfig,
        win: xlib::Window,
        attrib_list: *const c_int,
    ) -> GLXWindow;
    pub fn glXMakeContextCurrent(
        dpy: *mut xlib::cdef::Display,
        draw: GLXDrawable,
        read: GLXDrawable,
        ctx: *mut GLXContext,
    ) -> c_int;
    pub fn glXSwapBuffers(dpy: *mut xlib::cdef::Display, drawable: GLXDrawable);
    pub fn glXGetCurrentDrawable() -> GLXDrawable;
}
