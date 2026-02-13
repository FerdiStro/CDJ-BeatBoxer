mod app;
mod dev_state;

use crate::app::app::App;
use crate::dev_state::{DevState, DevStatus};
use color_eyre::Result;
use dotenvy::dotenv;






fn main() -> Result<()> {
    dotenv().expect(".ENV can't load");

    color_eyre::install()?;

    let mut dev_state = DevState::new();
    
    match dev_state.run_selection_window()? {
        DevStatus::App => App::new(),
        DevStatus::DevSel => App::new(),
        DevStatus::Dev => dev_state.run_dev(),
        _ => Ok(()),
    }
}
