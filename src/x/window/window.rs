use crate::x::atom::atom::Atom;
use crate::x::x::X;
use x11::xlib;

pub struct Window<'a>
{
  pub x: &'a X,
  pub w: xlib::Window,
}

impl<'a> Window<'a>
{
  pub(crate) fn new(x: &'a X, w: xlib::Window) -> Self
  {
    Self { x, w }
  }

  pub fn select_input(&self, event_mask: i64)
  {
    unsafe { xlib::XSelectInput(self.x.dpy, self.w, event_mask) };
  }

  pub fn get_attributes(&self) -> Option<xlib::XWindowAttributes>
  {
    let mut window_attributes_return: xlib::XWindowAttributes = unsafe { std::mem::zeroed() };
    if unsafe { xlib::XGetWindowAttributes(self.x.dpy, self.w, &mut window_attributes_return) } != 0
    {
      Some(window_attributes_return)
    } else {
      None
    }
  }

  pub fn get_transient_for_hint(&self) -> Option<xlib::Window>
  {
    let mut prop_window_return: xlib::Window = 0;
    if unsafe { xlib::XGetTransientForHint(self.x.dpy, self.w, &mut prop_window_return) } != 0 {
      Some(prop_window_return)
    } else {
      None
    }
  }

  pub fn get_name(&self) -> Option<String>
  {
    self
      .get_property_text(Atom::NetWMName)
      .and_then(|v| if v.is_empty() { None } else { Some(v) })
      .or(self.get_property_text(Atom::WMName))
      .and_then(|v| if v.is_empty() { None } else { Some(v) })
  }

  pub fn query_tree(&self) -> Option<Vec<Self>>
  {
    let mut root_return: xlib::Window = 0;
    let mut parent_return: xlib::Window = 0;
    let mut children_return: *mut xlib::Window = unsafe { std::mem::zeroed() };
    let mut nchildren_return: u32 = 0;
    let status: bool = unsafe {
      xlib::XQueryTree(
        self.x.dpy,
        self.w,
        &mut root_return,
        &mut parent_return,
        &mut children_return,
        &mut nchildren_return,
      )
    } != 0;

    let result = if status {
      Some(
        unsafe { std::slice::from_raw_parts(children_return, nchildren_return as usize) }
          .to_vec()
          .iter()
          .map(|v| Self::new(self.x, *v))
          .collect(),
      )
    } else {
      None
    };
    unsafe { xlib::XFree(children_return as *mut libc::c_void) };
    result
  }
}

impl std::fmt::Display for Window<'_>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", self.get_name().unwrap_or(String::from("None")),)
  }
}
