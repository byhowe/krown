use crate::x::window::window::Window;
use crate::x::x::X;
use x11::xlib;

pub struct WindowDecorations
{
  pub border_width: u32,
  pub border_color: u64,
  pub background_color: u64,
}

pub struct KrownConfig
{
  pub decorations: WindowDecorations,
}

pub(crate) struct Krown
{
  pub(crate) x: X,
}

impl Krown
{
  pub(crate) fn new(config: KrownConfig) -> Self
  {
    log::info!("connecting to the X server.");
    let x = X::connect("").unwrap_or_else(|| {
      log::error!("can't connect to the X server!");
      std::process::exit(1);
    });

    Self { x }
  }

  pub(crate) fn run(&self) -> bool
  {
    self.install_error_handlers();
    self.initialize_wm();
    self.x.grab(|| self.scan());

    true
  }

  fn initialize_wm(&self)
  {
    self
      .x
      .get_default_root()
      .select_input(xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask);
    self.x.sync(false);
  }

  fn scan(&self)
  {
    log::info!("scanning for windows.");
    if let Some(wins) = self.x.get_default_root().query_tree().map(|wins| {
      wins
        .into_iter()
        .filter_map(|w| {
          w.get_attributes()
            .filter(|wa| wa.override_redirect == 0 && wa.map_state == xlib::IsViewable)
            .map(|wa| (w, wa))
        })
        .collect::<Vec<(Window, xlib::XWindowAttributes)>>()
    }) {
      log::debug!("{} viewable window(s) detected.", wins.len());
      log::debug!("iterating non-transient windows.");
      for (w, wa) in &wins {
        if w.get_transient_for_hint().is_none() {
          self.manage(&w, &wa);
        }
      }

      log::debug!("iterating transient windows.");
      for (w, wa) in &wins {
        if w.get_transient_for_hint().is_some() {
          self.manage(&w, &wa);
        }
      }
    }
  }

  fn manage(&self, w: &Window, wa: &xlib::XWindowAttributes)
  {
    log::debug!(
      "managing a window with title: {}",
      w.get_name().unwrap_or(String::from("None"))
    );
  }
}
