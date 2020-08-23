use xcb_rs::x::X;
use xcb_rs::xcb;

const ROOT_EVENT_MASK: u32 = 0
  | xcb::event_mask_t::SUBSTRUCTURE_NOTIFY as u32
  | xcb::event_mask_t::SUBSTRUCTURE_REDIRECT as u32;

pub(crate) struct Krown
{
  x: X,
  screen: xcb::screen_t,
}

impl Krown
{
  pub(crate) fn new() -> Self
  {
    let x = X::connect("").unwrap_or_else(|err| {
      log::error!("cannot connect to the X server: {}", err);
      std::process::exit(1);
    });
    log::info!("connected to the X server.");

    let screen: xcb::screen_t = x
      .get_setup()
      .roots()
      .nth(x.default_screen as usize)
      .unwrap_or_else(|| {
        log::error!("cannot acquire the default screen.");
        std::process::exit(1);
      });

    Self { x, screen }
  }

  pub(crate) fn run(&self) -> bool
  {
    self.setup();

    true
  }

  fn setup(&self)
  {
    self.register_events();
  }

  fn register_events(&self)
  {
    let e: *mut xcb::generic_error_t = unsafe {
      xcb::request_check(
        self.x.c,
        xcb::change_window_attributes_checked(
          self.x.c,
          self.screen.root,
          xcb::cw_t::EVENT_MASK as u32,
          [ROOT_EVENT_MASK].as_ptr() as *const std::ffi::c_void,
        ),
      )
    };
    self.x.flush();
    if !e.is_null() {
      unsafe { libc::free(e as *mut std::ffi::c_void) };
      log::error!("another window manager is running.");
      std::process::exit(1);
    }
  }
}
