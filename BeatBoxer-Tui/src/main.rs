mod app;

use crate::app::app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    App::new()
}
