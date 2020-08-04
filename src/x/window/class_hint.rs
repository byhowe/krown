use crate::x::window::window::Window;
use std::ffi::CString;
use x11::xlib;

pub struct ClassHint
{
  name: String,
  class: String,
}

impl Window<'_>
{
  pub fn get_class_hint(&self) -> ClassHint
  {
    let mut hint = ClassHint::default();
    let mut class_hints_return: xlib::XClassHint = unsafe { std::mem::zeroed() };
    if unsafe { xlib::XGetClassHint(self.x.dpy, self.w, &mut class_hints_return) } != 0 {
      if !class_hints_return.res_name.is_null() {
        hint.name = unsafe { CString::from_raw(class_hints_return.res_name) }
          .into_string()
          .unwrap_or_default();
      }
      if !class_hints_return.res_class.is_null() {
        hint.class = unsafe { CString::from_raw(class_hints_return.res_class) }
          .into_string()
          .unwrap_or_default();
      }
    }
    hint
  }
}

impl Default for ClassHint
{
  fn default() -> Self
  {
    Self {
      name: String::default(),
      class: String::default(),
    }
  }
}

impl std::fmt::Display for ClassHint
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "(\"{}\" \"{}\")", self.name, self.class)
  }
}
