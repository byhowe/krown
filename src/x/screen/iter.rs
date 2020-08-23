use super::Screen;
use ::xcb::ffi::xproto;

pub struct ScreenIterator<'a>
{
  ptr: *mut xproto::xcb_screen_iterator_t<'a>,
}

impl<'a> Iterator for ScreenIterator<'a>
{
  type Item = Screen;

  fn next(&mut self) -> Option<Self::Item>
  {
    if unsafe { *self.ptr }.rem == 0 {
      None
    } else {
      let data = unsafe { *self.ptr }.data;
      unsafe { xproto::xcb_screen_next() }
      Some(Screen {})
    }
  }
}
