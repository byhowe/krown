use super::ConnectionError;
use super::Setup;
use ::xcb::ffi::base as xcb;
use ::xcb::ffi::xproto;
use std::ffi::CString;

/// Represents a connection to the X server.
///
/// Uses the XCB library to communicate with the X server.
///
/// Connection is automatically disconnected when it is dropped.
pub struct X
{
  c: *mut xcb::xcb_connection_t,
}

impl X
{
  /// Attempts to connect to the X server running on the display specified by
  /// the `display_name` variable. If the `display_name` is empty, then the
  /// `DISPLAY` environment variable is used to connect.
  pub fn connect(display_name: &str) -> Result<Self, ConnectionError>
  {
    let display_name: *const i8 = if display_name.is_empty() {
      std::ptr::null()
    } else {
      CString::new(display_name).map(|v| v.as_ptr()).unwrap()
    };
    let c: *mut xcb::xcb_connection_t =
      unsafe { xcb::xcb_connect(display_name, std::ptr::null_mut()) };
    ConnectionError::has_error(c)
      .map(|err| Err(err))
      .unwrap_or(Ok(Self { c }))
  }

  pub fn get_setup(&self) -> Setup
  {
    Setup::from(unsafe { xcb::xcb_get_setup(self.c) })
  }

  #[inline(always)]
  pub fn has_error(&self) -> Option<ConnectionError>
  {
    ConnectionError::has_error(self.c)
  }

  /// Forces any buffered output to be written to the server. Blocks until the
  /// write is complete.
  ///
  /// Returns `true` on success, `false` otherwise.
  #[inline(always)]
  pub fn flush(&self) -> bool
  {
    (unsafe { xcb::xcb_flush(self.c) } > 0)
  }
}

impl From<*mut xcb::xcb_connection_t> for X
{
  fn from(c: *mut xcb::xcb_connection_t) -> Self
  {
    Self { c }
  }
}

impl Drop for X
{
  fn drop(&mut self)
  {
    unsafe { xcb::xcb_disconnect(self.c) };
  }
}
