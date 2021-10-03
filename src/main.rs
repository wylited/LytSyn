//imports
pub mod app;
pub mod config;
pub mod playtree;
pub mod drpc;
pub mod renderer;

extern crate tokio;

use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut _args: Vec<String> = std::env::args().collect();

    let mut rust_mu: App = App::new();
    rust_mu.run()
}
