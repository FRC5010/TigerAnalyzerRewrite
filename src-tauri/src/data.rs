use std::{collections::HashMap, cmp};
use std::str::FromStr;
use reqwest::{header, Response, Error};
use serde::{Serialize, Deserialize, de};


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum ClimbState {
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

/* Definition of MatchEntry record.  Based upon CSV file headers. */
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntry {
   // #[serde(deserialize_with = "from_match_type_string")]
    //match_type: MatchType,
    //#[serde(default = "empty_tba_data")]
    //pub tba_match_data: Option<String>,

    // Game(2024) settings.
    pub teamNumber: u64,
    pub alliance: String,
    pub startTime: String,
    #[serde(deserialize_with = "from_scorepoints_string")]
    pub autoamp: u64,
    #[serde(deserialize_with = "from_scorepoints_string")]
    pub autospeaker: u64,
    #[serde(deserialize_with = "from_scorepoints_string")]
    pub teleopamp: u64,
    #[serde(deserialize_with = "from_scorepoints_string")]
    pub teleopspeaker: u64,
    #[serde(deserialize_with = "from_scorepoints_string")]
    pub teleoptrap: u64,
    #[serde(deserialize_with = "from_scorepoints_string")]
    pub subwoofer: u64,
    #[serde(deserialize_with = "from_climbtime_string")]
    pub climbtime: u64

}

impl MatchEntry {
    pub fn constrain_values(&mut self) -> MatchEntry {
        // Game(2024) settings.
        self.autoamp = self.autoamp.clamp(0, 5);
        self.autospeaker = self.autospeaker.clamp(0, 5);
        self.teleopamp = self.teleopamp.clamp(0, 20);
        self.teleopspeaker = self.teleopspeaker.clamp(0, 20);
        self.teleoptrap = self.teleoptrap.clamp(0, 3);
        self.subwoofer = self.subwoofer.clamp(0, 20);
        self.to_owned()
    }
}

fn from_scorepoints_string<'de, D>(
    deserializer: D,
) -> Result<u64, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer).unwrap_or("0");
    
    match s {
        "undefined" => Ok(0),
        _ => Ok(u64::from_str(s).unwrap())
    }
}

fn from_climbtime_string<'de, D>(
    deserializer: D,
) -> Result<u64, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer).unwrap_or("0");
    
    match s {
        "undefined" => Ok(0),
        _ => Ok(u64::from_str(s).unwrap())
    }
}

/*** 2023 Game remnants.  Leaving for coding examples.
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
) -> Result<ClimbState, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let num: &str = 
        de::Deserialize::deserialize(deserializer)?;
    match num {
        "0" => Ok(ClimbState::OffPlatform),
        "OffPlatform" => Ok(ClimbState::OffPlatform),
        "1" => Ok(ClimbState::OnPlatform),
        "OnPlatform" => Ok(ClimbState::OnPlatform),
        "2" => Ok(ClimbState::OnDocked),
        "OnDocked" => Ok(ClimbState::OnDocked),
        _ => Err(de::Error::custom("Not a valid Climb Status"))
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
***/

/* Team Summary.  Based upon summarization of team MatchEntry items. */
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TeamSummary {
    pub teamNumber: u64,
    pub total_speaker: f64,
    pub total_speaker_avg: f64,
    pub speaker_amplified: f64,
    pub speaker_unamplified: f64,
    pub total_amp: f64,
    pub total_amp_avg: f64,
    pub amp_amplified: f64,
    pub amp_unamplified: f64,
    pub auton_amp: f64,
    pub auton_amp_avg: f64,
    pub auton_speaker: f64,
    pub auton_speaker_avg: f64,
    pub points_trap: f64,
    pub climb_count: f64,
    pub climb_percentage: f64,
    pub dock_percentage: f64
}

/* Team Summary Averages.  Populated based upon summarization of team MatchEntry items. **/
struct  TeamSummaryAvgCounter {
    total_speaker: Vec<u64>,
    total_speaker_avg: Vec<u64>,
    speaker_amplified: Vec<u64>,
    speaker_unamplified: Vec<u64>,
    total_amp: Vec<u64>,
    total_amp_avg: Vec<u64>,
    amp_amplified: Vec<u64>,
    amp_unamplified: Vec<u64>,
    auton_amp: Vec<u64>,
    auton_amp_avg: Vec<u64>,
    auton_speaker: Vec<u64>,
    auton_speaker_avg: Vec<u64>,
    points_trap: Vec<u64>,
    climb_count: Vec<u64>,
    climb_percentage: Vec<f64>,
    dock_count: Vec<u64>
}

impl TeamSummaryAvgCounter {
    pub fn new() -> TeamSummaryAvgCounter {
        TeamSummaryAvgCounter { total_speaker: Vec::new(), total_speaker_avg: Vec::new(), speaker_amplified: Vec::new(), speaker_unamplified: Vec::new(),  
            total_amp: Vec::new(), total_amp_avg: Vec::new(), amp_amplified: Vec::new(), amp_unamplified: Vec::new(),
            auton_amp: Vec::new(), auton_amp_avg: Vec::new(), auton_speaker: Vec::new(), auton_speaker_avg: Vec::new(), 
            points_trap: Vec::new(), climb_count: Vec::new(), climb_percentage: Vec::new(), dock_count: Vec::new()
        }
    }
}


impl TeamSummary {
    pub fn new(team: &FrcTeam) -> TeamSummary {
        let mut avg_count = TeamSummaryAvgCounter::new();
        let mut climb_count = 0;
        for match_entry in &team.match_data {
            avg_count.auton_amp.push(match_entry.autoamp);
            avg_count.auton_amp_avg.push(match_entry.autoamp);
            avg_count.auton_speaker.push(match_entry.autospeaker);
            avg_count.auton_speaker_avg.push(match_entry.autospeaker);
            avg_count.total_speaker.push(match_entry.teleopspeaker);
            avg_count.total_speaker_avg.push(match_entry.teleopspeaker);
            avg_count.speaker_amplified.push(match_entry.teleopspeaker);
            avg_count.speaker_unamplified.push(match_entry.teleopspeaker);
            avg_count.total_amp.push(match_entry.teleopamp);
            avg_count.total_amp_avg.push(match_entry.teleopamp);
            avg_count.points_trap.push(match_entry.teleoptrap);
            if match_entry.climbtime > 0 { 
                avg_count.climb_count.push(1);
            } else {
                avg_count.climb_count.push(0);
            }
            
        }

        // TeamSummary object initializer.  Divide TeamSummaryAvgCounter fields by length of each fields array.
        TeamSummary { 
            teamNumber: team.teamNumber, 
            total_speaker: avg_count.total_speaker.iter().copied().sum::<u64>() as f64, 
            total_speaker_avg: avg_count.total_speaker_avg.iter().copied().sum::<u64>() as f64 / avg_count.total_speaker_avg.len() as f64, 
            speaker_amplified: avg_count.speaker_amplified.iter().copied().sum::<u64>() as f64 / avg_count.speaker_amplified.len() as f64, 
            speaker_unamplified: avg_count.speaker_unamplified.iter().copied().sum::<u64>() as f64 / avg_count.speaker_unamplified.len() as f64, 
            total_amp: avg_count.total_amp.iter().copied().sum::<u64>() as f64, 
            total_amp_avg: avg_count.total_amp_avg.iter().copied().sum::<u64>() as f64 / avg_count.total_amp_avg.len() as f64, 
            amp_amplified: avg_count.amp_amplified.iter().copied().sum::<u64>() as f64 / avg_count.total_amp.len() as f64,
            amp_unamplified: avg_count.amp_unamplified.iter().copied().sum::<u64>() as f64 / avg_count.total_amp.len() as f64,
            auton_amp: avg_count.auton_amp.iter().copied().sum::<u64>() as f64,
            auton_amp_avg: avg_count.auton_amp.iter().copied().sum::<u64>() as f64 / avg_count.auton_amp_avg.len() as f64,
            auton_speaker: avg_count.auton_speaker.iter().copied().sum::<u64>() as f64,
            auton_speaker_avg: avg_count.auton_speaker.iter().copied().sum::<u64>() as f64 / avg_count.auton_speaker_avg.len() as f64,
            points_trap: avg_count.points_trap.iter().copied().sum::<u64>() as f64 / avg_count.points_trap.len() as f64,  
            climb_count: avg_count.climb_count.iter().copied().sum::<u64>() as f64,
            climb_percentage: (avg_count.climb_count.iter().copied().sum::<u64>() as f64 / avg_count.climb_count.len() as f64),
            dock_percentage: avg_count.dock_count.iter().copied().sum::<u64>() as f64 / avg_count.dock_count.len() as f64,
        }
    }

    // Creates a combination of two teams into one summary.
    fn combine_teams(team1: &TeamSummary, team2: &TeamSummary) -> TeamSummary {
        TeamSummary {
            teamNumber: team1.teamNumber,
            total_speaker: (team1.total_speaker + team2.total_speaker),
            total_speaker_avg: (team1.total_speaker_avg + team2.total_speaker_avg),
            speaker_amplified: (team1.speaker_amplified + team2.speaker_amplified),
            speaker_unamplified: (team1.speaker_unamplified + team2.speaker_unamplified),
            total_amp: (team1.total_amp + team2.total_amp),
            total_amp_avg: (team1.total_amp_avg + team2.total_amp_avg),
            amp_amplified: (team1.amp_amplified + team2.amp_amplified),
            amp_unamplified: (team1.amp_unamplified + team2.amp_unamplified),
            auton_amp: (team1.auton_amp + team2.auton_amp),
            auton_amp_avg: (team1.auton_amp_avg + team2.auton_amp_avg),
            auton_speaker: (team1.auton_speaker + team2.auton_speaker),
            auton_speaker_avg: (team1.auton_speaker_avg + team2.auton_speaker_avg),
            points_trap: (team1.points_trap + team2.points_trap),
            climb_count: team1.climb_count + team2.climb_count,
            climb_percentage: f64::max(team1.climb_percentage, team2.climb_percentage),
            dock_percentage: f64::max(team1.dock_percentage, team2.dock_percentage)
        }
    }
    pub fn constrain_values(&mut self) -> Self {
        self.total_speaker = self.total_speaker.clamp(0.0, 9.0);
        self.speaker_amplified = self.speaker_amplified.clamp(0.0, 6.0);
        self.speaker_unamplified = self.speaker_unamplified.clamp(0.0, 6.0);
        self.total_amp = self.total_amp.clamp(0.0, 9.0);
        self.points_trap = self.points_trap.clamp(0.0, 3.0);
        self.climb_count = self.climb_count.clamp(0.0, 1.0);
        self.dock_percentage = self.dock_percentage.clamp(0.0, 1.0);
        self.to_owned()
    }
}

#[derive(Debug, Default, Clone, Serialize)]
//TODO update for 2024 game
pub struct RankMaxCount {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub climb: f64,
    pub dock: f64
}

// TODO update for 2024 game
pub struct PointValues {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub climb: f64,
    pub dock: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RankOptions {
    pub comparison_team: Option<FrcTeam>
}

// TODO update for 2024 game
#[derive(Debug, Default, Clone, Serialize)]
pub struct TeamRanking {
    pub teamNumber: u64,
    pub overall_rating: f64,
    pub low_rating: f64,
    pub medium_rating: f64,
    pub high_rating: f64,
    pub climb_rating: f64,
    pub dock_rating: f64,
    pub data_reliability_rating: f64
}


impl TeamRanking {
    pub fn generate_rankings(teams: HashMap<u64, FrcTeam>, options: RankOptions) -> Vec<TeamRanking> {
        let mut max_rank_count = RankMaxCount::default();
        let mut rankings = Vec::new();
        let mut comparison_team: FrcTeam;
        if options.comparison_team.is_none() { // Comparison Team is the team that is being added to each team to get the rating as if two teams were together
            comparison_team = FrcTeam::default();
        } else {
            comparison_team = options.comparison_team.unwrap();
        }

        // TODO: Make these better to configure
        let point_values = PointValues {
            low: 2.0,
            medium: 3.0,
            high: 5.0,
            climb: 6.0, // This is the remainder of Dock so its not in the total_points
            dock: 10.0
        };

        // TODO: Optimize to not iterate through all teams twice
        for mut team in teams.values() {
            if (comparison_team.teamNumber == team.teamNumber) {
                continue;
            }
            if (comparison_team.get_summary().is_none()) {
                comparison_team.summary = Some(TeamSummary::default());
            }  // Stupid hack to make sure comparison team has a summary
            let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap()).constrain_values();
            
            //if team_summary.avg_low > max_rank_count.low {
            //    max_rank_count.low = team_summary.avg_low;
            //}
            //if team_summary.avg_med > max_rank_count.medium {
            //    max_rank_count.medium = team_summary.avg_med;
            //}
            //if team_summary.avg_high > max_rank_count.high {
            //    max_rank_count.high = team_summary.avg_high;
            //}
            //if team_summary.balance_percentage > max_rank_count.balance {
            //    max_rank_count.balance = team_summary.balance_percentage;
            //}
            if team_summary.dock_percentage > max_rank_count.dock {
                max_rank_count.dock = team_summary.dock_percentage;
            }
        };

        let total_points_scored = (max_rank_count.low*point_values.low + max_rank_count.medium*point_values.medium + max_rank_count.high*point_values.high + max_rank_count.climb*point_values.climb + max_rank_count.dock*point_values.dock);

        for team in teams.values() {
            if comparison_team.teamNumber == team.teamNumber {
                continue;
            }
            let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap()).constrain_values();
            let mut ranking = TeamRanking::default();
            ranking.teamNumber = team.teamNumber;
            //ranking.low_rating = team_summary.avg_low / max_rank_count.low;
            //ranking.medium_rating = team_summary.avg_med / max_rank_count.medium;
            //ranking.high_rating = team_summary.avg_high / max_rank_count.high;
            //ranking.balance_rating = team_summary.balance_percentage / max_rank_count.balance;
            ranking.dock_rating = team_summary.dock_percentage / max_rank_count.dock;
            ranking.data_reliability_rating = 1.0;
            //ranking.overall_rating = (team_summary.avg_low*point_values.low + team_summary.avg_med*point_values.medium + team_summary.avg_high*point_values.high + team_summary.balance_percentage*point_values.balance + team_summary.dock_percentage*point_values.dock)/total_points_scored;
            rankings.push(ranking);
        };
        rankings

    }
        
}

#[derive(Debug, Default, Clone , Serialize, Deserialize)]
pub struct FrcTeam {
    version_id: u64,
    pub teamNumber: u64,
    match_data: Vec<MatchEntry>,
    pub summary: Option<TeamSummary>,
    tba_data: Option<HashMap<String, serde_json::Value>>
}

impl FrcTeam {
    pub fn new(teamNumber: u64) -> FrcTeam {
        FrcTeam { version_id: 1, teamNumber: teamNumber, match_data: Vec::new(), summary: None, tba_data: None} 
    }

    pub fn generate_summary(&mut self) {
        self.summary = Some(TeamSummary::new(&self).constrain_values());
    }

    pub fn get_summary(&self) -> &Option<TeamSummary> {
        &self.summary
    }

    /** 
    pub fn get_mut_summary(&mut self) -> &Option<TeamSummary> {
        &mut self.summary
    }
    */

    pub fn query_tba_data(&mut self, auth_key: &str) {
        self.tba_data = match get_tba_data(auth_key, &("/team/frc".to_owned()+&self.teamNumber.to_string())) {
            Ok(data) => Some(data.json::<HashMap<String, serde_json::Value>>().unwrap()),
            Err(err) => None
        };
    }

    pub fn add_match_entry(&mut self, mut entry: MatchEntry) {
        self.match_data.push(entry.constrain_values());
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
