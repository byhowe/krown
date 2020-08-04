use x11::xlib;

pub struct XA {}

impl XA
{
  pub const ALL: xlib::Atom = 0;
  pub const PRIMARY: xlib::Atom = 1;
  pub const SECONDARY: xlib::Atom = 2;
  pub const ARC: xlib::Atom = 3;
  pub const ATOM: xlib::Atom = 4;
  pub const BITMAP: xlib::Atom = 5;
  pub const CARDINAL: xlib::Atom = 6;
  pub const COLORMAP: xlib::Atom = 7;
  pub const CURSOR: xlib::Atom = 8;
  pub const CUT_BUFFER0: xlib::Atom = 9;
  pub const CUT_BUFFER1: xlib::Atom = 10;
  pub const CUT_BUFFER2: xlib::Atom = 11;
  pub const CUT_BUFFER3: xlib::Atom = 12;
  pub const CUT_BUFFER4: xlib::Atom = 13;
  pub const CUT_BUFFER5: xlib::Atom = 14;
  pub const CUT_BUFFER6: xlib::Atom = 15;
  pub const CUT_BUFFER7: xlib::Atom = 16;
  pub const DRAWABLE: xlib::Atom = 17;
  pub const FONT: xlib::Atom = 18;
  pub const INTEGER: xlib::Atom = 19;
  pub const PIXMAP: xlib::Atom = 20;
  pub const POINT: xlib::Atom = 21;
  pub const RECTANGLE: xlib::Atom = 22;
  pub const RESOURCE_MANAGER: xlib::Atom = 23;
  pub const RGB_COLOR_MAP: xlib::Atom = 24;
  pub const RGB_BEST_MAP: xlib::Atom = 25;
  pub const RGB_BLUE_MAP: xlib::Atom = 26;
  pub const RGB_DEFAULT_MAP: xlib::Atom = 27;
  pub const RGB_GRAY_MAP: xlib::Atom = 28;
  pub const RGB_GREEN_MAP: xlib::Atom = 29;
  pub const RGB_RED_MAP: xlib::Atom = 30;
  pub const STRING: xlib::Atom = 31;
  pub const VISUALID: xlib::Atom = 32;
  pub const WINDOW: xlib::Atom = 33;
  pub const WM_COMMAND: xlib::Atom = 34;
  pub const WM_HINTS: xlib::Atom = 35;
  pub const WM_CLIENT_MACHINE: xlib::Atom = 36;
  pub const WM_ICON_NAME: xlib::Atom = 37;
  pub const WM_ICON_SIZE: xlib::Atom = 38;
  pub const WM_NAME: xlib::Atom = 39;
  pub const WM_NORMAL_HINTS: xlib::Atom = 40;
  pub const WM_SIZE_HINTS: xlib::Atom = 41;
  pub const WM_ZOOM_HINTS: xlib::Atom = 42;
  pub const MIN_SPACE: xlib::Atom = 43;
  pub const NORM_SPACE: xlib::Atom = 44;
  pub const MAX_SPACE: xlib::Atom = 45;
  pub const END_SPACE: xlib::Atom = 46;
  pub const SUPERSCRIPT_X: xlib::Atom = 47;
  pub const SUPERSCRIPT_Y: xlib::Atom = 48;
  pub const SUBSCRIPT_X: xlib::Atom = 49;
  pub const SUBSCRIPT_Y: xlib::Atom = 50;
  pub const UNDERLINE_POSITION: xlib::Atom = 51;
  pub const UNDERLINE_THICKNESS: xlib::Atom = 52;
  pub const STRIKEOUT_ASCENT: xlib::Atom = 53;
  pub const STRIKEOUT_DESCENT: xlib::Atom = 54;
  pub const ITALIC_ANGLE: xlib::Atom = 55;
  pub const X_HEIGHT: xlib::Atom = 56;
  pub const QUAD_WIDTH: xlib::Atom = 57;
  pub const WEIGHT: xlib::Atom = 58;
  pub const POINT_SIZE: xlib::Atom = 59;
  pub const RESOLUTION: xlib::Atom = 60;
  pub const COPYRIGHT: xlib::Atom = 61;
  pub const NOTICE: xlib::Atom = 62;
  pub const FONT_NAME: xlib::Atom = 63;
  pub const FAMILY_NAME: xlib::Atom = 64;
  pub const FULL_NAME: xlib::Atom = 65;
  pub const CAP_HEIGHT: xlib::Atom = 66;
  pub const WM_CLASS: xlib::Atom = 67;
  pub const WM_TRANSIENT_FOR: xlib::Atom = 68;
}