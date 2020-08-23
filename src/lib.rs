mod console;
mod krown;

pub fn launch()
{
  let status = console::Console::new().perform_actions();

  if !status.success {
    std::process::exit(1);
  }

  if status.start {
    std::process::exit(!krown::Krown::new().run() as i32);
  }
}
