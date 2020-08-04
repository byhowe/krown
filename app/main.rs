fn main()
{
  pretty_env_logger::init_custom_env("KROWN_LOG");

  let config = krown::KrownConfig {
    decorations: krown::WindowDecorations {
      border_width: 3,
      border_color: 0x0000ff,
      background_color: 0xffff00,
    },
  };
  krown::launch(config);
}
