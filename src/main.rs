// mod components;
mod layout;
mod pages;
mod app;
mod provider;
use app::App;

fn main() {
  yew::start_app::<App>();
  wasm_logger::init(wasm_logger::Config::default());
}