mod client;
mod console;
mod error_handler;
mod krown;
mod x;

pub use krown::KrownConfig;
pub use krown::WindowDecorations;

pub fn launch(config: KrownConfig)
{
  let status = console::Console::new().perform_actions();

  if !status.success {
    std::process::exit(1);
  }

  if status.start {
    std::process::exit(!krown::Krown::new(config).run() as i32);
  }
}
