use std::default;

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
enum BalanceState {
    #[default]
    off_platform,
    on_platform,
    on_docked,
}

#[derive(Default, Debug, Serialize, Deserialize)]
enum MatchType {
    #[default]
    Friendly,
    Quarter,
    Semi,
    Final,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MatchEntry {
    date: u64,
    match_type: MatchType,
    team_number: u64,
    tba_match_data: String,
    alliance: String,
    cone_low_count: u64,
    cone_med_count: u64,
    cone_high_count: u64,
    cube_low_count: u64,
    cube_med_count: u64,
    cube_high_count: u64,
    floor_pickup: bool,
    auton_balance: BalanceState,
    end_game_balance: BalanceState
}

#[derive(Debug, Default)]
pub struct TeamSummary {
    team_number: u64,
    avg_cone_low: u64,
    avg_cone_med: u64,
    avg_cone_high: u64,
    avg_cube_low: u64,
    avg_cube_med: u64,
    avg_cube_high: u64,

}

#[derive(Debug, Default)]
pub struct FrcTeam {
    version_id: u64,
    team_number: u64,
    match_data: Vec<MatchEntry>,
    summary: TeamSummary,
    tba_data: String
}

impl FrcTeam {
    fn generate_summary(&self) {
        println!("generating summary");
    }
    fn query_tba_data(&self) {
        println!("querying TBA data");
    }

    pub fn add_match_entry(&mut self, entry: MatchEntry) {
        self.match_data.push(entry);
    }
}