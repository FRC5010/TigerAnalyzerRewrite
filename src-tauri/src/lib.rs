use std::{error::Error, collections::HashMap};
use data::FrcTeam;
use tauri::{LogicalSize, Size, Manager, Window};
use tauri::App;

mod data;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

fn show_window(handle: &tauri::AppHandle, window_label: &str) {
    handle.get_window(window_label).unwrap().show();
}


fn close_window(handle: &tauri::AppHandle, window_label: &str) {
    handle.get_window(window_label).unwrap().close();
}


#[derive(Clone, Debug, serde::Serialize)]
enum ScoutDataField {
    String(String),
    Number(f64)
}

fn read_scout_data(data_path: &str) -> Result<HashMap<String, HashMap<String, Vec<ScoutDataField>>>, Box<dyn Error>> {
    let mut TeamsHashMap = HashMap::new();
    let mut reader = csv::Reader::from_path(data_path)?;

    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result.expect("Probably not good");
        for (i, value) in record.iter().enumerate() {
            let val = match value.parse::<f64>() {
                Ok(val) => ScoutDataField::Number(val),
                Err(_y) => ScoutDataField::String(String::from(value))
            };
            TeamsHashMap
                .entry(String::from(&record[1]))
                .or_insert_with(HashMap::new)
                .entry(String::from(&headers[i]))
                .or_insert_with(Vec::new)
                .push(val);

        }
    }
    Ok(TeamsHashMap)
}

fn average_scout_data(raw_data: HashMap<String, HashMap<String, Vec<ScoutDataField>>>) -> Result<HashMap<String, HashMap<String, f64>>, Box<dyn Error>> {
    let mut TeamsProcessedHashMap = HashMap::new();
    for (team_number, team_data) in raw_data {
        let mut processedTeamData = TeamsProcessedHashMap
            .entry(team_number)
            .or_insert_with(HashMap::new);
        
        'data_groups: for (data_group, data_list) in team_data {
            let mut total = 0.0;
            let count = data_list.len();
            for entry in data_list {
                match entry {
                    ScoutDataField::Number(val) => {
                        total += val;
                    }
                    ScoutDataField::String(_val) => {
                        continue 'data_groups;
                    }
                }
            }
            processedTeamData
                .insert(data_group, total/count as f64);
        }

    }
    Ok(TeamsProcessedHashMap)
}


#[tauri::command]
fn submit_data(handle: tauri::AppHandle, data_path: &str) {
    show_window(&handle, "results");
    close_window(&handle, "main");

    let raw_data = read_scout_data(data_path).expect("Couldn't Get Data");
    let averaged_data = average_scout_data(raw_data.clone());

    handle.emit_all("data-loaded", raw_data);

    

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