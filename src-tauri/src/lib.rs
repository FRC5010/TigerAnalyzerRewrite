use std::{error::Error, collections::HashMap};
use data::{FrcTeam, MatchEntry};
use tauri::{Manager};
use tauri::App;

extern crate csv;
extern crate serde;

mod data;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

pub static TBA_AUTH_KEY: &str = "YpCes0r7kuXhw0S5fubKU27qoU4cAwDft0NBjhD3DdUKa9taHKhC3zGR0mqK76zA";



fn show_window(handle: &tauri::AppHandle, window_label: &str) {
    handle.get_window(window_label).unwrap().show().expect("Couldn't find window");
}


fn close_window(handle: &tauri::AppHandle, window_label: &str) {
    handle.get_window(window_label).unwrap().close().expect("Couldn't find window");
}


fn read_scout_data(data_path: &str) -> Result<HashMap<u64, FrcTeam>, Box<dyn Error>> {
    let mut team_list: HashMap<u64, FrcTeam> = HashMap::new();
    let mut csv_data = csv::Reader::from_path(data_path)?;
    for entry in csv_data.deserialize() {
        let match_entry: MatchEntry = entry?;
        if !team_list.contains_key(&match_entry.team_number) {
            team_list.insert(match_entry.team_number, FrcTeam::new(match_entry.team_number));
        }
        team_list.get_mut(&match_entry.team_number).unwrap().add_match_entry(match_entry);
    }
    for team in team_list.values_mut() {
      team.generate_summary();
    }

    Ok(team_list)
}


#[tauri::command]
fn submit_data(handle: tauri::AppHandle, data_path: &str) {
    show_window(&handle, "results");
    close_window(&handle, "main");
    let data: HashMap<u64, FrcTeam> = read_scout_data(data_path).unwrap();
    handle.emit_to("results", "got_data", data).expect("Unable to Emit Data");

}

fn get_data(handle: &tauri::AppHandle) {
  
  // Return data to Tauri Frontend
}


#[derive(Default)]
pub struct AppBuilder {
  setup: Option<SetupHook>,
}

impl AppBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  #[must_use]
  pub fn setup<F>(mut self, setup: F) -> Self
  where
    F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
  {
    self.setup.replace(Box::new(setup));
    self
  }

  pub fn run(self) {
    let setup = self.setup;
    tauri::Builder::default()
      .setup(move |app| {
        if let Some(setup) = setup {
          (setup)(app)?;
        }
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![submit_data])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }
}


mod tests {

}