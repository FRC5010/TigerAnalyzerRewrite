use std::collections::HashMap;

use reqwest::{header, Response, Error};
use serde::{Serialize, Deserialize, de};


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum BalanceState {
    #[default]
    OffPlatform,
    OnPlatform,
    OnDocked,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum MatchType {
    #[default]
    Friendly,
    Quarter,
    Semi,
    Final,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntry {
    pub date: u64,
    #[serde(deserialize_with = "from_match_type_string")]
    match_type: MatchType,
    pub team_number: u64,
    #[serde(default = "empty_tba_data")]
    pub tba_match_data: Option<String>,
    pub alliance: String,
    pub cone_low_count: u64,
    pub cone_med_count: u64,
    pub cone_high_count: u64,
    pub cube_low_count: u64,
    pub cube_med_count: u64,
    pub cube_high_count: u64,
    #[serde(deserialize_with = "from_bool_string")]
    pub floor_pickup: bool,
    #[serde(deserialize_with = "from_charge_station_int")]
    auton_balance: BalanceState,
    #[serde(deserialize_with = "from_charge_station_int")]
    end_game_balance: BalanceState
}

fn empty_tba_data() -> Option<String> {
    None
}

fn from_bool_string<'de, D>(
    deserializer: D,
) -> Result<bool, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let s: &str =
        de::Deserialize::deserialize(deserializer).unwrap_or("false");
    
    match s {
        "true"|"TRUE" => Ok(true),
        "false"|"FALSE" => Ok(false),
        _ => Err(de::Error::custom("Not a valid boolean"))
        
    }
}

fn from_charge_station_int<'de, D>(
    deserializer: D,
) -> Result<BalanceState, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let num: &str = 
        de::Deserialize::deserialize(deserializer)?;
    match num {
        "0" => Ok(BalanceState::OffPlatform),
        "OffPlatform" => Ok(BalanceState::OffPlatform),
        "1" => Ok(BalanceState::OnPlatform),
        "OnPlatform" => Ok(BalanceState::OnPlatform),
        "2" => Ok(BalanceState::OnDocked),
        "OnDocked" => Ok(BalanceState::OnDocked),
        _ => Err(de::Error::custom("Not a valid Balance Status"))
    }
}

fn from_match_type_string<'de, D>(
    deserializer: D,
) -> Result<MatchType, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let string: &str =
        de::Deserialize::deserialize(deserializer)?;
    
    match string {
        _ => Ok(MatchType::Friendly),
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TeamSummary {
    pub team_number: u64,
    pub avg_cone_low: f64,
    pub avg_cone_med: f64,
    pub avg_cone_high: f64,
    pub avg_cube_low: f64,
    pub avg_cube_med: f64,
    pub avg_cube_high: f64,
    pub avg_low: f64,
    pub avg_med: f64,
    pub avg_high: f64,
    pub can_balance: bool,
    pub balance_percentage: f64,
    pub dock_percentage: f64
}

// UNSURE OF IMPLEMENTATION FOR AVERAGING
struct  TeamSummaryAvgCounter {
    avg_cone_low: Vec<u64>,
    avg_cone_med: Vec<u64>,
    avg_cone_high: Vec<u64>,
    avg_cube_low: Vec<u64>,
    avg_cube_med: Vec<u64>,
    avg_cube_high: Vec<u64>,
    balance_count: Vec<u64>,
    dock_count: Vec<u64>
}
impl TeamSummaryAvgCounter {
    pub fn new() -> TeamSummaryAvgCounter {
        TeamSummaryAvgCounter { avg_cone_low: Vec::new(), avg_cone_med: Vec::new(), avg_cone_high: Vec::new(), avg_cube_low: Vec::new(), avg_cube_med: Vec::new(), avg_cube_high: Vec::new(), balance_count: Vec::new(), dock_count: Vec::new() }
    }
}


impl TeamSummary {
    pub fn new(team: &FrcTeam) -> TeamSummary {
        let mut avg_count = TeamSummaryAvgCounter::new();
        let mut balance_flag = false;
        for match_entry in &team.match_data {
            avg_count.avg_cone_low.push(match_entry.cone_low_count);
            avg_count.avg_cone_med.push(match_entry.cone_med_count);
            avg_count.avg_cone_high.push(match_entry.cone_high_count);
            avg_count.avg_cube_low.push(match_entry.cube_low_count);
            avg_count.avg_cube_med.push(match_entry.cube_med_count);
            avg_count.avg_cube_high.push(match_entry.cube_high_count);
            match match_entry.end_game_balance {
                BalanceState::OffPlatform => {
                    avg_count.balance_count.push(0);
                    avg_count.dock_count.push(0);
                }
                
                BalanceState::OnDocked => {
                    avg_count.balance_count.push(0);
                    avg_count.dock_count.push(1);
                    balance_flag = true;
                }
                
                BalanceState::OnPlatform => {
                    avg_count.balance_count.push(1);
                    avg_count.dock_count.push(0);
                    balance_flag = true;
                }
            }

        }

        TeamSummary { 
            team_number: team.team_number, 
            avg_cone_low: avg_count.avg_cone_low.iter().copied().sum::<u64>() as f64 / avg_count.avg_cone_low.len() as f64, 
            avg_cone_med: avg_count.avg_cone_med.iter().copied().sum::<u64>() as f64 /avg_count.avg_cone_med.len() as f64, 
            avg_cone_high: avg_count.avg_cone_high.iter().copied().sum::<u64>() as f64 /avg_count.avg_cone_high.len() as f64, 
            avg_cube_low: avg_count.avg_cube_low.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_low.len() as f64, 
            avg_cube_med: avg_count.avg_cube_med.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_med.len() as f64,  
            avg_cube_high: avg_count.avg_cube_high.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_high.len() as f64,
            avg_low: avg_count.avg_cone_low.iter().copied().sum::<u64>() as f64 / avg_count.avg_cone_low.len() as f64 + avg_count.avg_cube_low.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_low.len() as f64,
            can_balance: balance_flag,
            avg_med: avg_count.avg_cone_med.iter().copied().sum::<u64>() as f64 /avg_count.avg_cone_med.len() as f64 + avg_count.avg_cube_med.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_med.len() as f64,
            avg_high: avg_count.avg_cone_high.iter().copied().sum::<u64>() as f64 /avg_count.avg_cone_high.len() as f64 + avg_count.avg_cube_high.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_high.len() as f64,
            balance_percentage: 0.0 + avg_count.balance_count.iter().copied().sum::<u64>() as f64 /avg_count.balance_count.len() as f64, 
            dock_percentage: avg_count.dock_count.iter().copied().sum::<u64>() as f64 /avg_count.dock_count.len() as f64 }


    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct RankMaxCount {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub balance: f64,
    pub dock: f64
}

pub struct PointValues {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub balance: f64,
    pub dock: f64
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RankOptions {
    pub comparison_teams: Option<Vec<FrcTeam>>
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct TeamRanking {
    pub team_number: u64,
    pub overall_rating: f64,
    pub low_rating: f64,
    pub medium_rating: f64,
    pub high_rating: f64,
    pub balance_rating: f64,
    pub dock_rating: f64,
    pub data_reliability_rating: f64
}



const MAX_SCORE_COLUMNS: f64 = 9.0;


impl TeamRanking {
    // TODO: ADD COMBINING TEAMS TO RANKING
    pub fn generate_rankings(teams: HashMap<u64, FrcTeam>, options: RankOptions) -> Vec<TeamRanking> {
        let mut maxCount = RankMaxCount::default();
        let mut rankings = Vec::new();

        // TODO: Make these better to configure
        let point_values = PointValues {
            low: 2.0,
            medium: 3.0,
            high: 5.0,
            balance: 6.0, // This is the remainder of Dock so its not in the total_points
            dock: 10.0
        };

        // TODO: Optimize to not iterate through all teams twice
        for mut team in teams.values() {
            let team_summary = team.get_summary().as_ref().unwrap();
            if team_summary.avg_low > maxCount.low {
                maxCount.low = team_summary.avg_low;
            }
            if team_summary.avg_med > maxCount.medium {
                maxCount.medium = team_summary.avg_med;
            }
            if team_summary.avg_high > maxCount.high {
                maxCount.high = team_summary.avg_high;
            }
            if team_summary.balance_percentage > maxCount.balance {
                maxCount.balance = team_summary.balance_percentage;
            }
            if team_summary.dock_percentage > maxCount.dock {
                maxCount.dock = team_summary.dock_percentage;
            }
        };

        let totalPoints = (maxCount.low*point_values.low + maxCount.medium*point_values.medium + maxCount.high*point_values.high + maxCount.balance*point_values.balance + maxCount.dock*point_values.dock);

        for team in teams.values() {
            let team_summary = team.get_summary().as_ref().unwrap();
            let mut ranking = TeamRanking::default();
            ranking.team_number = team.team_number;
            ranking.low_rating = team_summary.avg_low / maxCount.low;
            ranking.medium_rating = team_summary.avg_med / maxCount.medium;
            ranking.high_rating = team_summary.avg_high / maxCount.high;
            ranking.balance_rating = team_summary.balance_percentage / maxCount.balance;
            ranking.dock_rating = team_summary.dock_percentage / maxCount.dock;
            ranking.data_reliability_rating = 1.0;
            ranking.overall_rating = (team_summary.avg_low*point_values.low + team_summary.avg_med*point_values.medium + team_summary.avg_high*point_values.high + team_summary.balance_percentage*point_values.balance + team_summary.dock_percentage*point_values.dock)/totalPoints;
            rankings.push(ranking);
        };
        rankings

    }
        
}

#[derive(Debug, Default, Clone , Serialize, Deserialize)]
pub struct FrcTeam {
    version_id: u64,
    pub team_number: u64,
    match_data: Vec<MatchEntry>,
    pub summary: Option<TeamSummary>,
    tba_data: Option<HashMap<String, serde_json::Value>>
}

impl FrcTeam {
    pub fn new(team_number: u64) -> FrcTeam {
        FrcTeam { version_id: 1, team_number: team_number, match_data: Vec::new(), summary: None, tba_data: None} 
    }

    pub fn generate_summary(&mut self) {
        self.summary = Some(TeamSummary::new(&self));
    }

    pub fn get_summary(&self) -> &Option<TeamSummary> {
        &self.summary
    }

    pub fn get_mut_summary(&mut self) -> &Option<TeamSummary> {
        &mut self.summary
    }

    pub fn query_tba_data(&mut self, auth_key: &str) {
        self.tba_data = match get_tba_data(auth_key, &("/team/frc".to_owned()+&self.team_number.to_string())) {
            Ok(data) => Some(data.json::<HashMap<String, serde_json::Value>>().unwrap()),
            Err(err) => None
        };
    }

    pub fn add_match_entry(&mut self, entry: MatchEntry) {
        self.match_data.push(entry);
    }
}


fn get_tba_data(auth_key:&str, query:&str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let request_url = "https://www.thebluealliance.com/api/v3".to_string()+query;
    let client = reqwest::blocking::Client::new();
    let response =  client.get(request_url)
        .header("X-TBA-Auth-Key", auth_key)
        .send();
    response    
}

#[cfg(test)]
mod tests {
}