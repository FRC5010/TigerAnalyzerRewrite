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
        de::Deserialize::deserialize(deserializer)?;
    
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
    let num: u64 = 
        de::Deserialize::deserialize(deserializer)?;
    match num {
        0 => Ok(BalanceState::OffPlatform),
        1 => Ok(BalanceState::OnPlatform),
        2 => Ok(BalanceState::OnDocked),
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


#[derive(Debug, Default, Clone, Serialize)]
pub struct TeamSummary {
    team_number: u64,
    avg_cone_low: f64,
    avg_cone_med: f64,
    avg_cone_high: f64,
    avg_cube_low: f64,
    avg_cube_med: f64,
    avg_cube_high: f64,
}

// UNSURE OF IMPLEMENTATION FOR AVERAGING
struct  TeamSummaryAvgCounter {
    avg_cone_low: Vec<u64>,
    avg_cone_med: Vec<u64>,
    avg_cone_high: Vec<u64>,
    avg_cube_low: Vec<u64>,
    avg_cube_med: Vec<u64>,
    avg_cube_high: Vec<u64>,
}
impl TeamSummaryAvgCounter {
    pub fn new() -> TeamSummaryAvgCounter {
        TeamSummaryAvgCounter { avg_cone_low: Vec::new(), avg_cone_med: Vec::new(), avg_cone_high: Vec::new(), avg_cube_low: Vec::new(), avg_cube_med: Vec::new(), avg_cube_high: Vec::new() }
    }
}


impl TeamSummary {
    pub fn new(team: &FrcTeam) -> TeamSummary {
        let mut avg_count = TeamSummaryAvgCounter::new();
        for match_entry in &team.match_data {
            avg_count.avg_cone_low.push(match_entry.cone_low_count);
            avg_count.avg_cone_med.push(match_entry.cone_med_count);
            avg_count.avg_cone_high.push(match_entry.cone_high_count);
            avg_count.avg_cube_low.push(match_entry.cube_low_count);
            avg_count.avg_cube_med.push(match_entry.cube_med_count);
            avg_count.avg_cube_high.push(match_entry.cube_high_count);
        }

        TeamSummary { 
            team_number: team.team_number, 
            avg_cone_low: avg_count.avg_cone_low.iter().copied().sum::<u64>() as f64 / avg_count.avg_cone_low.len() as f64, 
            avg_cone_med: avg_count.avg_cone_med.iter().copied().sum::<u64>() as f64 /avg_count.avg_cone_med.len() as f64, 
            avg_cone_high: avg_count.avg_cone_high.iter().copied().sum::<u64>() as f64 /avg_count.avg_cone_high.len() as f64, 
            avg_cube_low: avg_count.avg_cube_low.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_low.len() as f64, 
            avg_cube_med: avg_count.avg_cube_med.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_med.len() as f64,  
            avg_cube_high: avg_count.avg_cube_high.iter().copied().sum::<u64>() as f64 /avg_count.avg_cube_high.len() as f64, }


    }
}

#[derive(Debug, Default, Clone , Serialize)]
pub struct FrcTeam {
    version_id: u64,
    team_number: u64,
    match_data: Vec<MatchEntry>,
    summary: Option<TeamSummary>,
    tba_data: Option<String>
}

impl FrcTeam {
    pub fn new(team_number: u64) -> FrcTeam {
        FrcTeam { version_id: 1, team_number: team_number, match_data: Vec::new(), summary: None, tba_data: None }
    }

    pub fn generate_summary(&mut self) {
        self.summary = Some(TeamSummary::new(&self));
    }

    pub fn get_summary(&self) -> &Option<TeamSummary> {
        &self.summary
    }

    pub fn query_tba_data(&self, auth_key: &str) {
        println!("querying TBA data");
    }

    pub fn add_match_entry(&mut self, entry: MatchEntry) {
        self.match_data.push(entry);
    }
}

#[cfg(test)]
mod tests {
}