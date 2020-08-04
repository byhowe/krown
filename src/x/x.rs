use crate::x::atom::atom::Atoms;
use crate::x::window::window::Window;
use x11::xinerama;
use x11::xlib;

pub struct X
{
  pub dpy: *mut xlib::Display,
  pub atoms: Atoms,
}

impl X
{
  pub fn connect(display_name: &str) -> Option<Self>
  {
    let display_name: *const i8 = if display_name.is_empty() {
      std::ptr::null()
    } else {
      std::ffi::CString::new(display_name)
        .map(|v| v.as_ptr())
        .unwrap_or(std::ptr::null())
    };
    let dpy: *mut xlib::Display = unsafe { xlib::XOpenDisplay(display_name) };

    if dpy.is_null() {
      None
    } else {
      Some(Self {
        dpy,
        atoms: Atoms::new(dpy),
      })
    }
  }

  pub fn get_default_root(&self) -> Window
  {
    Window::new(self, unsafe { xlib::XDefaultRootWindow(self.dpy) })
  }

  pub fn xinerama_is_active(&self) -> bool
  {
    (unsafe { xinerama::XineramaIsActive(self.dpy) } != 0)
  }

  pub fn sync(&self, discard: bool)
  {
    unsafe { xlib::XSync(self.dpy, discard as i32) };
  }

  pub fn grab<F>(&self, while_grabbed: F)
  where
    F: FnOnce(),
  {
    log::debug!("grabbing the server.");
    unsafe { xlib::XGrabServer(self.dpy) };
    while_grabbed();
    log::debug!("ungrabbing the server.");
    unsafe { xlib::XUngrabServer(self.dpy) };
  }

  pub fn set_error_handler(
    handler: extern "C" fn(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> i32,
  )
  {
    unsafe { xlib::XSetErrorHandler(Some(handler)) };
  }
}
