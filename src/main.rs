// mod components;
mod app;
mod layout;
mod pages;
mod provider;
use app::App;

fn main() {
  yew::start_app::<App>();
  wasm_logger::init(wasm_logger::Config::default());
}
