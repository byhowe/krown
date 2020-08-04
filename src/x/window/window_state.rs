use crate::x::atom::atom::Atom;
use crate::x::window::window::Window;

pub enum WindowState
{
  Above,
  Below,
  DemandsAttention,
  Focused,
  Fullscreen,
  Hidden,
  MaximizedHorz,
  MaximizedVert,
  Modal,
  SkipPager,
  SkipTaskbar,
  StaysOnTop,
  Sticky,
}

impl Window<'_>
{
  pub fn get_states(&self) -> Vec<WindowState>
  {
    self
      .get_property_atom(Atom::NetWMState)
      .iter()
      .filter_map(|prop| match prop {
        Atom::NetWMStateAbove => Some(WindowState::Above),
        Atom::NetWMStateBelow => Some(WindowState::Below),
        Atom::NetWMStateDemandsAttention => Some(WindowState::DemandsAttention),
        Atom::NetWMStateFocused => Some(WindowState::Focused),
        Atom::NetWMStateFullscreen => Some(WindowState::Fullscreen),
        Atom::NetWMStateHidden => Some(WindowState::Hidden),
        Atom::NetWMStateMaximizedHorz => Some(WindowState::MaximizedHorz),
        Atom::NetWMStateMaximizedVert => Some(WindowState::MaximizedVert),
        Atom::NetWMStateModal => Some(WindowState::Modal),
        Atom::NetWMStateSkipPager => Some(WindowState::SkipPager),
        Atom::NetWMStateSkipTaskbar => Some(WindowState::SkipTaskbar),
        Atom::NetWMStateStaysOnTop => Some(WindowState::StaysOnTop),
        Atom::NetWMStateSticky => Some(WindowState::Sticky),
        _ => None,
      })
      .collect()
  }
}

impl std::fmt::Display for WindowState
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Above => "Above",
        Self::Below => "Below",
        Self::DemandsAttention => "DemandsAttention",
        Self::Focused => "Focused",
        Self::Fullscreen => "Fullscreen",
        Self::Hidden => "Hidden",
        Self::MaximizedHorz => "MaximizedHorz",
        Self::MaximizedVert => "MaximizedVert",
        Self::Modal => "Modal",
        Self::SkipPager => "SkipPager",
        Self::SkipTaskbar => "SkipTaskbar",
        Self::StaysOnTop => "StaysOnTop",
        Self::Sticky => "Sticky",
      }
    )
  }
}
