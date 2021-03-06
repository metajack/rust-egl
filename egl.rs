/* automatically generated by rust-bindgen */

#[feature(globs)];

use std::libc::*;
use std::str::raw;
use std::ptr::to_unsafe_ptr;

pub type khronos_int32_t = int32_t;
pub type khronos_uint32_t = uint32_t;
pub type khronos_int64_t = int64_t;
pub type khronos_uint64_t = uint64_t;
pub type khronos_int8_t = c_schar;
pub type khronos_uint8_t = c_uchar;
pub type khronos_int16_t = c_short;
pub type khronos_uint16_t = c_ushort;
pub type khronos_intptr_t = c_long;
pub type khronos_uintptr_t = c_ulong;
pub type khronos_ssize_t = c_long;
pub type khronos_usize_t = c_ulong;
pub type khronos_float_t = c_float;
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_stime_nanoseconds_t = khronos_int64_t;
pub type ANativeWindow = c_void;
pub type egl_native_pixmap_t = c_void;
pub type EGLNativeWindowType = *ANativeWindow;
pub type EGLNativePixmapType = *egl_native_pixmap_t;
pub type EGLNativeDisplayType = *c_void;
pub type NativeDisplayType = EGLNativeDisplayType;
pub type NativePixmapType = EGLNativePixmapType;
pub type NativeWindowType = EGLNativeWindowType;
pub type EGLint = khronos_int32_t;
pub type EGLBoolean = c_uint;
pub type EGLenum = c_uint;
pub type EGLConfig = *c_void;
pub type EGLContext = *c_void;
pub type EGLDisplay = *c_void;
pub type EGLSurface = *c_void;
pub type EGLClientBuffer = *c_void;
pub type __eglMustCastToProperFunctionPointerType = *u8;
pub type status_t = int32_t;

pub static EGL_CONTEXT_CLIENT_VERSION: c_uint = 0x3098;
pub static EGL_NO_CONTEXT: c_uint = 0;
pub static EGL_DEFAULT_DISPLAY: c_uint = 0;
pub static EGL_NONE: c_uint = 0x3038;  // Attrib list terminator
pub static EGL_NO_DISPLAY: c_uint = 0;
pub static EGL_TRUE: c_uint = 1;
pub static EGL_FALSE: c_uint = 0;
pub static EGL_NO_SURFACE: c_uint = 0;
pub static EGL_SURFACE_TYPE: c_uint = 0x3033;
pub static EGL_WINDOW_BIT: c_uint = 0x0004;     // EGL_SURFACE_TYPE mask bits
pub static EGL_RENDERABLE_TYPE: c_uint = 0x3040;
pub static EGL_OPENGL_ES2_BIT: c_uint = 0x0004; // EGL_RENDERABLE_TYPE mask bits
pub static EGL_HEIGHT: c_uint = 0x3056;
pub static EGL_WIDTH: c_uint = 0x3057;

pub static EGL_BUFFER_SIZE: c_uint = 0x3020;
pub static EGL_DEPTH_SIZE: c_uint = 0x3025;
pub static EGL_ALPHA_SIZE: c_uint = 0x3021;
pub static EGL_BLUE_SIZE: c_uint = 0x3022;
pub static EGL_GREEN_SIZE: c_uint = 0x3023;
pub static EGL_RED_SIZE: c_uint = 0x3024;

pub static EGL_DRAW: EGLint = 0x3059;

#[nolink]
#[link_args = "-lEGL"]
#[cfg(target_os = "android")]
extern {}
#[fixed_stack_segment]
pub fn GetError() -> EGLint {
    unsafe {
        return eglGetError();
    }
}
#[fixed_stack_segment]
pub fn GetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay {
    unsafe {
        return eglGetDisplay(display_id);
    }
}
#[fixed_stack_segment]
pub fn Initialize(dpy: EGLDisplay, major: &mut EGLint, minor: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglInitialize(dpy, to_unsafe_ptr(major), to_unsafe_ptr(minor));
    }
}
#[fixed_stack_segment]
pub fn Terminate(dpy: EGLDisplay) -> EGLBoolean {
    unsafe {
        return eglTerminate(dpy);
    }
}
#[fixed_stack_segment]
pub fn QueryString(dpy: EGLDisplay, name: EGLint) -> ~str {
    unsafe {
        return raw::from_c_str(eglQueryString(dpy, name));
    }
}
#[fixed_stack_segment]
pub fn GetConfigs(dpy: EGLDisplay, configs: &mut EGLConfig, config_size: EGLint, num_config: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglGetConfigs(dpy, to_unsafe_ptr(configs), config_size, to_unsafe_ptr(num_config));
    }
}
#[fixed_stack_segment]
pub fn ChooseConfig(dpy: EGLDisplay, attrib_list: *EGLint, configs: &mut EGLConfig, config_size: EGLint, num_config: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglChooseConfig(dpy, attrib_list, to_unsafe_ptr(configs), config_size, to_unsafe_ptr(num_config));
    }
}
#[fixed_stack_segment]
pub fn GetConfigAttrib(dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglGetConfigAttrib(dpy, config, attribute, to_unsafe_ptr(value));
    }
}
#[fixed_stack_segment]
pub fn CreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *EGLint) -> EGLSurface {
    unsafe {
        return eglCreateWindowSurface(dpy, config, win, attrib_list);
    }
}
#[fixed_stack_segment]
pub fn CreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig, attrib_list: &mut EGLint) -> EGLSurface {
    unsafe {
        return eglCreatePbufferSurface(dpy, config, to_unsafe_ptr(attrib_list));
    }
}
#[fixed_stack_segment]
pub fn CreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: &mut EGLint) -> EGLSurface {
    unsafe {
        return eglCreatePixmapSurface(dpy, config, pixmap, to_unsafe_ptr(attrib_list));
    }
}
#[fixed_stack_segment]
pub fn DestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    unsafe {
        return eglDestroySurface(dpy, surface);
    }
}
#[fixed_stack_segment]
pub fn QuerySurface(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglQuerySurface(dpy, surface, attribute, to_unsafe_ptr(value));
    }
}
#[fixed_stack_segment]
pub fn BindAPI(api: EGLenum) -> EGLBoolean {
    unsafe {
        return eglBindAPI(api);
    }
}
#[fixed_stack_segment]
pub fn QueryAPI() -> EGLenum {
    unsafe {
        return eglQueryAPI();
    }
}
#[fixed_stack_segment]
pub fn WaitClient() -> EGLBoolean {
    unsafe {
        return eglWaitClient();
    }
}
#[fixed_stack_segment]
pub fn ReleaseThread() -> EGLBoolean {
    unsafe {
        return eglReleaseThread();
    }
}
#[fixed_stack_segment]
pub fn CreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: &mut EGLint) -> EGLSurface {
    unsafe {
        return eglCreatePbufferFromClientBuffer(dpy, buftype, buffer, config, to_unsafe_ptr(attrib_list));
    }
}
#[fixed_stack_segment]
pub fn SurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint) -> EGLBoolean {
    unsafe {
        return eglSurfaceAttrib(dpy, surface, attribute, value);
    }
}
#[fixed_stack_segment]
pub fn BindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean {
    unsafe {
        return eglBindTexImage(dpy, surface, buffer);
    }
}
#[fixed_stack_segment]
pub fn ReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean {
    unsafe {
        return eglReleaseTexImage(dpy, surface, buffer);
    }
}
#[fixed_stack_segment]
pub fn SwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean {
    unsafe {
        return eglSwapInterval(dpy, interval);
    }
}
#[fixed_stack_segment]
pub fn CreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *EGLint) -> EGLContext {
    unsafe {
        return eglCreateContext(dpy, config, share_context, attrib_list);
    }
}
#[fixed_stack_segment]
pub fn DestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean {
    unsafe {
        return eglDestroyContext(dpy, ctx);
    }
}
#[fixed_stack_segment]
pub fn MakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean {
    unsafe {
        return eglMakeCurrent(dpy, draw, read, ctx);
    }
}
#[fixed_stack_segment]
pub fn GetCurrentContext() -> EGLContext {
    unsafe {
        return eglGetCurrentContext();
    }
}
#[fixed_stack_segment]
pub fn GetCurrentSurface(readdraw: EGLint) -> EGLSurface {
    unsafe {
        return eglGetCurrentSurface(readdraw);
    }
}
#[fixed_stack_segment]
pub fn GetCurrentDisplay() -> EGLDisplay {
    unsafe {
        return eglGetCurrentDisplay();
    }
}
#[fixed_stack_segment]
pub fn QueryContext(dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: &mut EGLint) -> EGLBoolean {
    unsafe {
        return eglQueryContext(dpy, ctx, attribute, to_unsafe_ptr(value));
    }
}
#[fixed_stack_segment]
pub fn WaitGL() -> EGLBoolean {
    unsafe {
        return eglWaitGL();
    }
}
#[fixed_stack_segment]
pub fn WaitNative(engine: EGLint) -> EGLBoolean {
    unsafe {
        return eglWaitNative(engine);
    }
}
#[fixed_stack_segment]
pub fn SwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    unsafe {
        return eglSwapBuffers(dpy, surface);
    }
}
#[fixed_stack_segment]
pub fn CopyBuffers(dpy: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType) -> EGLBoolean {
    unsafe {
        return eglCopyBuffers(dpy, surface, target);
    }
}

#[nolink]
extern {
    fn eglGetError() -> EGLint;
    fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;
    fn eglInitialize(dpy: EGLDisplay, major: *EGLint, minor: *EGLint) -> EGLBoolean;
    fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
    fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *c_schar;
    fn eglGetConfigs(dpy: EGLDisplay, configs: *EGLConfig, config_size: EGLint, num_config: *EGLint) -> EGLBoolean;
    fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *EGLint, configs: *EGLConfig, config_size: EGLint, num_config: *EGLint) -> EGLBoolean;
    fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: *EGLint) -> EGLBoolean;
    fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *EGLint) -> EGLSurface;
    fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig, attrib_list: *EGLint) -> EGLSurface;
    fn eglCreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: *EGLint) -> EGLSurface;
    fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *EGLint) -> EGLBoolean;
    fn eglBindAPI(api: EGLenum) -> EGLBoolean;
    fn eglQueryAPI() -> EGLenum;
    fn eglWaitClient() -> EGLBoolean;
    fn eglReleaseThread() -> EGLBoolean;
    fn eglCreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: *EGLint) -> EGLSurface;
    fn eglSurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint) -> EGLBoolean;
    fn eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    fn eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;
    fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;
    fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *EGLint) -> EGLContext;
    fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
    fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext) -> EGLBoolean;
    fn eglGetCurrentContext() -> EGLContext;
    fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;
    fn eglGetCurrentDisplay() -> EGLDisplay;
    fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: *EGLint) -> EGLBoolean;
    fn eglWaitGL() -> EGLBoolean;
    fn eglWaitNative(engine: EGLint) -> EGLBoolean;
    fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    fn eglCopyBuffers(dpy: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType) -> EGLBoolean;
    fn eglGetProcAddress(procname: *c_schar) -> __eglMustCastToProperFunctionPointerType;
}
