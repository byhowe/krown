use crate::x::atom::atom::Atom;
use crate::x::window::window::Window;

pub enum WindowType
{
  Combo,
  Desktop,
  Dialog,
  Dnd,
  Dock,
  DropdownMenu,
  Menu,
  Normal,
  Notification,
  PopupMenu,
  Splash,
  Toolbar,
  Tooltip,
  Utility,
}

impl Window<'_>
{
  pub fn get_types(&self) -> Vec<WindowType>
  {
    self
      .get_property_atom(Atom::NetWMWindowType)
      .iter()
      .filter_map(|prop| match prop {
        Atom::NetWMWindowTypeCombo => Some(WindowType::Combo),
        Atom::NetWMWindowTypeDesktop => Some(WindowType::Desktop),
        Atom::NetWMWindowTypeDialog => Some(WindowType::Dialog),
        Atom::NetWMWindowTypeDnd => Some(WindowType::Dnd),
        Atom::NetWMWindowTypeDock => Some(WindowType::Dock),
        Atom::NetWMWindowTypeDropdownMenu => Some(WindowType::DropdownMenu),
        Atom::NetWMWindowTypeMenu => Some(WindowType::Menu),
        Atom::NetWMWindowTypeNormal => Some(WindowType::Normal),
        Atom::NetWMWindowTypeNotification => Some(WindowType::Notification),
        Atom::NetWMWindowTypePopupMenu => Some(WindowType::PopupMenu),
        Atom::NetWMWindowTypeSplash => Some(WindowType::Splash),
        Atom::NetWMWindowTypeToolbar => Some(WindowType::Toolbar),
        Atom::NetWMWindowTypeTooltip => Some(WindowType::Tooltip),
        Atom::NetWMWindowTypeUtility => Some(WindowType::Utility),
        _ => None,
      })
      .collect()
  }
}

impl std::fmt::Display for WindowType
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Combo => "Combo",
        Self::Desktop => "Desktop",
        Self::Dialog => "Dialog",
        Self::Dnd => "Dnd",
        Self::Dock => "Dock",
        Self::DropdownMenu => "DropdownMenu",
        Self::Menu => "Menu",
        Self::Normal => "Normal",
        Self::Notification => "Notification",
        Self::PopupMenu => "PopupMenu",
        Self::Splash => "Splash",
        Self::Toolbar => "Toolbar",
        Self::Tooltip => "Tooltip",
        Self::Utility => "Utility",
      }
    )
  }
}
