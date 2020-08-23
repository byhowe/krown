use ::xcb::ffi::base as xcb;

pub enum ConnectionError
{
  ConnectionError,
  ExtNotSupported,
  MemInsufficient,
  ReqLenExceed,
  ParseError,
  InvalidScreen,
}

impl std::fmt::Display for ConnectionError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::ConnectionError => "socket, pipe or other stream errors.",
        Self::ExtNotSupported => "extension not supported.",
        Self::MemInsufficient => "memory not available.",
        Self::ReqLenExceed => "exceeding request length that server accepts.",
        Self::ParseError => "error during parsing display string.",
        Self::InvalidScreen => "the server does not have a screen matching the display.",
      }
    )
  }
}

impl ConnectionError
{
  /// Test wheter a connection has shut down due to a fatal or unexpected error.
  #[inline(always)]
  pub fn has_error(c: *mut xcb::xcb_connection_t) -> Option<ConnectionError>
  {
    //! https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#ga70a6bade94bd2824db552abcf5fbdbe3
    match unsafe { xcb::xcb_connection_has_error(c) } {
      0 => None,
      xcb::XCB_CONN_ERROR => Some(ConnectionError::ConnectionError),
      xcb::XCB_CONN_CLOSED_EXT_NOTSUPPORTED => Some(ConnectionError::ExtNotSupported),
      xcb::XCB_CONN_CLOSED_MEM_INSUFFICIENT => Some(ConnectionError::MemInsufficient),
      xcb::XCB_CONN_CLOSED_REQ_LEN_EXCEED => Some(ConnectionError::ReqLenExceed),
      xcb::XCB_CONN_CLOSED_PARSE_ERR => Some(ConnectionError::ParseError),
      xcb::XCB_CONN_CLOSED_INVALID_SCREEN => Some(ConnectionError::InvalidScreen),
      _ => Some(ConnectionError::ConnectionError),
    }
  }
}
