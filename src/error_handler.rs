use crate::krown::Krown;
use crate::x::x::X;
use x11::xlib;

const X_CONFIGURE_WINDOW: u8 = 12;
const X_COPY_AREA: u8 = 62;
const X_GRAB_BUTTON: u8 = 28;
const X_GRAB_KEY: u8 = 33;
const X_POLY_FILL_RECTANGLE: u8 = 70;
const X_POLY_SEGMENT: u8 = 66;
const X_POLY_TEXT_8: u8 = 74;
const X_SET_INPUT_FOCUS: u8 = 42;

fn x_error_event_to_string(e: &xlib::XErrorEvent) -> String
{
  format!(
    "  XErrorEvent:\n    type: {}\n    resource_id: {}\n    serial: {}\n    error_code: {}\n    \
     request_code: {}\n    minor_code: {}",
    e.type_, e.resourceid, e.serial, e.error_code, e.request_code, e.minor_code
  )
}

pub extern "C" fn check_other_wm(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> i32
{
  let ee = unsafe { *e };
  if ee.error_code == xlib::BadAccess {
    log::error!("another window manager is already running!");
  } else {
    log::error!(
      "check_other_wm detected error code other than BadAccess:\n\n{}\n",
      x_error_event_to_string(&ee)
    );
  }

  std::process::exit(1);
}

pub extern "C" fn x_error_handler(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> i32
{
  let ee: xlib::XErrorEvent = unsafe { *e };
  if ee.error_code == xlib::BadAtom {
    return 0;
  }
  if ee.error_code == xlib::BadWindow
    || (ee.request_code == X_CONFIGURE_WINDOW && ee.error_code == xlib::BadMatch)
    || (ee.request_code == X_COPY_AREA && ee.error_code == xlib::BadDrawable)
    || (ee.request_code == X_GRAB_BUTTON && ee.error_code == xlib::BadAccess)
    || (ee.request_code == X_GRAB_KEY && ee.error_code == xlib::BadAccess)
    || (ee.request_code == X_POLY_FILL_RECTANGLE && ee.error_code == xlib::BadDrawable)
    || (ee.request_code == X_POLY_SEGMENT && ee.error_code == xlib::BadDrawable)
    || (ee.request_code == X_POLY_TEXT_8 && ee.error_code == xlib::BadDrawable)
    || (ee.request_code == X_SET_INPUT_FOCUS && ee.error_code == xlib::BadMatch)
  {
    log::debug!(
      "x_error_handler caught exception\n\n{}\n",
      x_error_event_to_string(&ee)
    );
    0
  } else {
    log::error!("fatal X11 error:\n\n{}\n", x_error_event_to_string(&ee));
    1
  }
}

impl Krown
{
  pub fn install_error_handlers(&self)
  {
    log::info!("installing error handlers.");
    X::set_error_handler(check_other_wm);
    self
      .x
      .get_default_root()
      .select_input(xlib::SubstructureRedirectMask);
    self.x.sync(false);
    X::set_error_handler(x_error_handler);
    self.x.sync(false);
  }
}
