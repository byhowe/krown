use crate::x;
use crate::x::atom::atom::Atom;
use crate::x::window::window::Window;
use num_integer::Integer;
use x11::xlib;

struct Property<T>
{
  type_atom: Atom,
  format: i32,
  n_items: usize,
  data: *mut T,
}

impl<T> Property<T>
{
  fn drop_data(&self)
  {
    unsafe { xlib::XFree(self.data as *mut libc::c_void) };
  }
}

impl Window<'_>
{
  fn get_property<T: Integer>(
    &self,
    property: Atom,
    long_length: i64,
    req_type: Atom,
  ) -> Option<Property<T>>
  {
    let mut actual_type_return: xlib::Atom = 0;
    let mut actual_format_return: i32 = 0;
    let mut nitems_return: u64 = 0;
    let mut bytes_after_return: u64 = 0;
    let mut prop_return: *mut u8 = unsafe { std::mem::zeroed() };
    if unsafe {
      xlib::XGetWindowProperty(
        self.x.dpy,
        self.w,
        self.x.atoms.to_xatom(property),
        0,
        long_length,
        false as i32,
        self.x.atoms.to_xatom(req_type),
        &mut actual_type_return,
        &mut actual_format_return,
        &mut nitems_return,
        &mut bytes_after_return,
        &mut prop_return,
      )
    } == x::SUCCESS
      && nitems_return != 0
      && !prop_return.is_null()
    {
      Some(Property {
        type_atom: self
          .x
          .atoms
          .to_atom(actual_type_return)
          .unwrap_or(Atom::All),
        format: actual_format_return,
        n_items: nitems_return as usize,
        data: prop_return as *mut T,
      })
    } else {
      unsafe { xlib::XFree(prop_return as *mut libc::c_void) };
      None
    }
  }

  pub fn get_property_text(&self, property: Atom) -> Option<String>
  {
    if let Some(prop) = self.get_property::<i8>(property, i64::MAX, Atom::All) {
      if prop.type_atom == Atom::String
      /* iso_8859-1 */
      {
        let mut result = String::with_capacity(prop.n_items);
        for c in unsafe { std::slice::from_raw_parts(prop.data, prop.n_items) } {
          let ch: u8 = *c as u8;
          if ch < 0x80 {
            result.push(ch as char);
          } else {
            result.push((0xc0 | ch >> 6) as char);
            result.push((0x80 | (ch & 0x3f)) as char);
          }
        }
        Some(result)
      } else if prop.type_atom == Atom::Utf8String
      /* utf-8 */
      {
        Some(unsafe { String::from_raw_parts(prop.data as *mut u8, prop.n_items, prop.n_items) })
      } else {
        prop.drop_data();
        None
      }
    } else {
      None
    }
  }

  pub fn get_property_atom(&self, property: Atom) -> Vec<Atom>
  {
    self
      .get_property::<xlib::Atom>(property, i64::MAX, Atom::Atom)
      .and_then(|v| {
        if v.format == 32 && v.type_atom == Atom::Atom {
          Some(
            unsafe { Vec::from_raw_parts(v.data, v.n_items, v.n_items) }
              .iter()
              .map(|a| self.x.atoms.to_atom(*a).unwrap())
              .collect(),
          )
        } else {
          None
        }
      })
      .unwrap_or(Vec::with_capacity(0))
  }

  pub fn get_property_cardinal(&self, property: Atom) -> Vec<i64>
  {
    self
      .get_property(property, i64::MAX, Atom::Cardinal)
      .and_then(|v| {
        if v.format == 32 && v.type_atom == Atom::Cardinal {
          Some(unsafe { Vec::from_raw_parts(v.data, v.n_items, v.n_items) })
        } else {
          None
        }
      })
      .unwrap_or(Vec::with_capacity(0))
  }
}
