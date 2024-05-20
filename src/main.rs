mod app;
mod rename_tool;
mod util;



use app::App;
use clap::Parser;

fn main() {
  let app = App::parse();
  app.action();
  println!("{}","");
}
