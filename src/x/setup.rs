use ::xcb::ffi::xproto;

pub struct Setup
{
  ptr: *const xproto::xcb_setup_t,
}

impl Setup
{
  pub fn roots(&self) {}
}

impl From<*const xproto::xcb_setup_t> for Setup
{
  fn from(ptr: *const xproto::xcb_setup_t) -> Self
  {
    Self { ptr }
  }
}
