use std::{collections::HashMap, cmp};
use std::str::FromStr;
use reqwest::{header, Response, Error};
use serde::{Serialize, Deserialize, de};




/* Definition of MatchEntry record.  Based upon CSV file headers. */
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntry {
    
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
    #[serde(deserialize_with = "from_climbtime_string")]
    pub climbtime: u64,
    #[serde(deserialize_with = "from_scorepoints_string")]
    pub amplifications: u64,
    //Added "#[serde..." to amlifications to try and remove errors
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote1: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote2: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote3: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote4: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote5: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote6: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote7: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
    pub autonote8: u64,
    //#[serde(deserialize_with = "from_scorepoints_string")]
}

impl MatchEntry {
/*
    pub fn constrain_values(&mut self) -> MatchEntry {
        
        self.autoamp = self.autoamp.clamp(0, 5);
        self.autospeaker = self.autospeaker.clamp(0, 5);
        self.teleopamp = self.teleopamp.clamp(0, 20);
        self.teleopspeaker = self.teleopspeaker.clamp(0, 20);
        self.teleoptrap = self.teleoptrap.clamp(0, 3);
        self.to_owned()
    }
*/
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
        _ => Ok(u64::from_str(s).unwrap_or_default())
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
        _ => Ok(u64::from_str(s).unwrap_or_default())
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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum ClimbState {
    #[default]
    OffPlatform,
    OnPlatform,
    OnDocked,
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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum MatchType {
    #[default]
    Friendly,
    Quarter,
    Semi,
    Final,
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
    pub teleopspeaker: f64,
    pub teleopspeaker_avg: f64,
    pub teleopamp: f64,
    pub teleopamp_avg: f64,
    pub autoamp: f64,
    pub autoamp_avg: f64,
    pub autospeaker: f64,
    pub autospeaker_avg: f64,
    pub points_trap: f64,
    pub climb_count: f64,
    pub climb_percentage: f64,
    pub amplifications: f64,
    pub autonote1: f64,
    pub autonote2: f64,
    pub autonote3: f64,
    pub autonote4: f64,
    pub autonote5: f64,
    pub autonote6: f64,
    pub autonote7: f64,
    pub autonote8: f64, 
}

/* Team Summary Averages.  Populated based upon summarization of team MatchEntry items. **/
struct  TeamSummaryAvgCounter {
    teleopspeaker: Vec<u64>,
    teleopspeaker_avg: Vec<u64>,
    teleopamp: Vec<u64>,
    teleopamp_avg: Vec<u64>,
    autoamp: Vec<u64>,
    autoamp_avg: Vec<u64>,
    autospeaker: Vec<u64>,
    autospeaker_avg: Vec<u64>,
    points_trap: Vec<u64>,
    climb_count: Vec<u64>,
    climb_percentage: Vec<f64>,
    amplifications: Vec<u64>,
    autonote1: Vec<u64>,
    autonote2: Vec<u64>,
    autonote3: Vec<u64>,
    autonote4: Vec<u64>,
    autonote5: Vec<u64>,
    autonote6: Vec<u64>,
    autonote7: Vec<u64>,
    autonote8: Vec<u64>,
}

impl TeamSummaryAvgCounter {
    pub fn new() -> TeamSummaryAvgCounter {
        TeamSummaryAvgCounter { teleopspeaker: Vec::new(), teleopspeaker_avg: Vec::new(), 
            teleopamp: Vec::new(), teleopamp_avg: Vec::new(), autoamp: Vec::new(), autoamp_avg: Vec::new(),
            autospeaker: Vec::new(), autospeaker_avg: Vec::new(), points_trap: Vec::new(), 
            climb_count: Vec::new(), climb_percentage: Vec::new(), amplifications: Vec::new(), autonote1: Vec::new(),
            autonote2: Vec::new(), autonote3: Vec::new(), autonote4: Vec::new(), autonote5: Vec::new(), autonote6: Vec::new(), 
            autonote7: Vec::new(), autonote8: Vec::new(),
        }
    }
}


impl TeamSummary {
    pub fn new(team: &FrcTeam) -> TeamSummary {
        let mut avg_count = TeamSummaryAvgCounter::new();
        let mut climb_count = 0;
        for match_entry in &team.match_data {
            avg_count.autoamp.push(match_entry.autoamp);
            avg_count.autoamp_avg.push(match_entry.autoamp);
            avg_count.autospeaker.push(match_entry.autospeaker);
            avg_count.autospeaker_avg.push(match_entry.autospeaker);
            avg_count.teleopspeaker.push(match_entry.teleopspeaker);
            avg_count.teleopspeaker_avg.push(match_entry.teleopspeaker);
            avg_count.teleopamp.push(match_entry.teleopamp);
            avg_count.teleopamp_avg.push(match_entry.teleopamp);
            avg_count.points_trap.push(match_entry.teleoptrap);
            avg_count.amplifications.push(match_entry.amplifications);
            avg_count.autonote1.push(match_entry.autonote1);
            avg_count.autonote2.push(match_entry.autonote2);
            avg_count.autonote3.push(match_entry.autonote3);
            avg_count.autonote4.push(match_entry.autonote4);
            avg_count.autonote5.push(match_entry.autonote5);
            avg_count.autonote6.push(match_entry.autonote6);
            avg_count.autonote7.push(match_entry.autonote7);
            avg_count.autonote8.push(match_entry.autonote8);
            if match_entry.climbtime > 0 { 
                avg_count.climb_count.push(1);
            } else {
                avg_count.climb_count.push(0);
            }
        }

        // TeamSummary object initializer.  Divide TeamSummaryAvgCounter fields by length of each fields array.
        TeamSummary { 
            teamNumber: team.teamNumber, 
            teleopspeaker: avg_count.teleopspeaker.iter().copied().sum::<u64>() as f64, 
            teleopspeaker_avg: avg_count.teleopspeaker_avg.iter().copied().sum::<u64>() as f64 / avg_count.teleopspeaker_avg.len() as f64, 
            teleopamp: avg_count.teleopamp.iter().copied().sum::<u64>() as f64, 
            teleopamp_avg: avg_count.teleopamp_avg.iter().copied().sum::<u64>() as f64 / avg_count.teleopamp_avg.len() as f64, 
            autoamp: avg_count.autoamp.iter().copied().sum::<u64>() as f64,
            autoamp_avg: avg_count.autoamp.iter().copied().sum::<u64>() as f64 / avg_count.autoamp_avg.len() as f64,
            autospeaker: avg_count.autospeaker.iter().copied().sum::<u64>() as f64,
            autospeaker_avg: avg_count.autospeaker.iter().copied().sum::<u64>() as f64 / (avg_count.autospeaker.len()-8) as f64,
            points_trap: avg_count.points_trap.iter().copied().sum::<u64>() as f64,  
            climb_count: avg_count.climb_count.iter().copied().sum::<u64>() as f64,
            climb_percentage: (avg_count.climb_count.iter().copied().sum::<u64>() as f64 / avg_count.climb_count.len() as f64),
            amplifications: (avg_count.amplifications.iter().copied().sum::<u64>() as f64),
            autonote1: avg_count.autonote1.iter().copied().sum::<u64>() as f64,
            autonote2: avg_count.autonote2.iter().copied().sum::<u64>() as f64,
            autonote3: avg_count.autonote3.iter().copied().sum::<u64>() as f64,
            autonote4: avg_count.autonote4.iter().copied().sum::<u64>() as f64,
            autonote5: avg_count.autonote5.iter().copied().sum::<u64>() as f64,
            autonote6: avg_count.autonote6.iter().copied().sum::<u64>() as f64,
            autonote7: avg_count.autonote7.iter().copied().sum::<u64>() as f64,
            autonote8: avg_count.autonote8.iter().copied().sum::<u64>() as f64,
        }
    }

    // Creates a combination of two teams into one summary.
    fn combine_teams(team1: &TeamSummary, team2: &TeamSummary) -> TeamSummary {
        TeamSummary {
            teamNumber: team1.teamNumber,
            teleopspeaker: (team1.teleopspeaker + team2.teleopspeaker),
            teleopspeaker_avg: (team1.teleopspeaker_avg + team2.teleopspeaker_avg),
            teleopamp: (team1.teleopamp + team2.teleopamp),
            teleopamp_avg: (team1.teleopamp_avg + team2.teleopamp_avg),
            autoamp: (team1.autoamp + team2.autoamp),
            autoamp_avg: (team1.autoamp_avg + team2.autoamp_avg),
            autospeaker: (team1.autospeaker + team2.autospeaker),
            autospeaker_avg: (team1.autospeaker_avg + team2.autospeaker_avg),
            points_trap: (team1.points_trap + team2.points_trap),
            climb_count: team1.climb_count + team2.climb_count,
            climb_percentage: f64::max(team1.climb_percentage, team2.climb_percentage),            
            amplifications: (team1.amplifications + team2.amplifications),
            autonote1: (team1.autonote1 + team2.autonote1),
            autonote2: (team1.autonote2 + team2.autonote2),
            autonote3: (team1.autonote3 + team2.autonote3),
            autonote4: (team1.autonote4 + team2.autonote4),
            autonote5: (team1.autonote5 + team2.autonote5),
            autonote6: (team1.autonote6 + team2.autonote6),
            autonote7: (team1.autonote7 + team2.autonote7),
            autonote8: (team1.autonote8 + team2.autonote8),
        }
    }
/*    
    pub fn constrain_values(&mut self) -> Self {
        self.teleopspeaker = self.teleopspeaker.clamp(0.0, 9.0);
        self.teleopamp = self.teleopamp.clamp(0.0, 9.0);
        self.points_trap = self.points_trap.clamp(0.0, 3.0);
        self.climb_count = self.climb_count.clamp(0.0, 1.0);
        self.to_owned()
    }
*/
}

#[derive(Debug, Default, Clone, Serialize)]
//tracks the max value of each of the following items for each combined team so we can guage how each 
//combined team sits wrt the max value
pub struct RankMaxCount {
    pub autoamp: f64,
    pub autospeaker: f64,
    pub teleopamp: f64,
    pub teleopspeaker: f64,
    pub teleoptrap: f64,
    pub climbcount: f64,
    pub amplifications: f64,
    pub autonote1: f64,
    pub autonote2: f64,
    pub autonote3: f64,
    pub autonote4: f64,
    pub autonote5: f64,
    pub autonote6: f64,
    pub autonote7: f64,
    pub autonote8: f64,
}

//pub struct PointValues {
//    pub low: f64,
//    pub medium: f64,
//    pub high: f64,
//    pub climb: f64,
//}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RankOptions {
    pub comparison_team: Option<FrcTeam>,
    pub sort_order: Option<String>
}

// TODO update for 2024 game
#[derive(Debug, Default, Clone, Serialize)]
pub struct TeamRanking {
    pub teamNumber: u64,
    pub overall_rating: f64,
    pub autoamp_rating: f64,
    pub autospeaker_rating: f64,
    pub teleopamp_rating: f64,
    pub teleopspeaker_rating: f64,
    pub teleoptrap_rating: f64,
    pub climbcount_rating: f64,
    pub amplification_rating: f64,
    pub data_reliability_rating: f64,
    pub autonote1_rating: f64,
    pub autonote2_rating: f64,
    pub autonote3_rating: f64,
    pub autonote4_rating: f64,
    pub autonote5_rating: f64,
    pub autonote6_rating: f64,
    pub autonote7_rating: f64,
    pub autonote8_rating: f64,
}


impl TeamRanking 
{
    //CLJ: rank by:
    //      Sort by teams that AMP up
    //      Then sort by teams that have the highest speaker shots
    //      Then sort by teams that have the highest amp shots?
    //      OR 
    //      Sort by the teams that have highest climb numbers?
    pub fn generate_rankings(teams: HashMap<u64, FrcTeam>, options: RankOptions) -> Vec<TeamRanking> {
        let mut max_rank = RankMaxCount::default();
        let mut rankings = Vec::new();
        let mut comparison_team: FrcTeam;
        // Comparison Team is the team that is being added to each team to get the rating as if two teams were together
        if options.comparison_team.is_none() { 
            comparison_team = FrcTeam::default();
        } else {
            comparison_team = options.comparison_team.unwrap_or_default();
        }

//        // TODO: Make these better to configure
//        let point_values = PointValues {
//            low: 2.0,
//            medium: 3.0,
//            high: 5.0,
//            climb: 6.0, // This is the remainder of Dock so its not in the total_points
//        };

        // TODO: Optimize to not iterate through all teams twice
        for mut team in teams.values() {
            if (comparison_team.teamNumber == team.teamNumber) {
                continue;
            }
            if (comparison_team.get_summary().is_none()) {
                comparison_team.summary = Some(TeamSummary::default());
            }  // Stupid hack to make sure comparison team has a summary.

            //CLJ: why constrain only the comparison team?  why constrain at all?
            //let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap()).constrain_values();
            let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap());
           
            max_rank.autoamp = if (team_summary.autoamp > max_rank.autoamp) {team_summary.autoamp} else {max_rank.autoamp};
            max_rank.autospeaker = if (team_summary.autospeaker > max_rank.autospeaker) {team_summary.autospeaker} else {max_rank.autospeaker};
            max_rank.teleopamp = if (team_summary.teleopamp > max_rank.teleopamp) {team_summary.teleopamp} else {max_rank.teleopamp};
            max_rank.teleopspeaker = if (team_summary.teleopspeaker > max_rank.teleopspeaker) {team_summary.teleopspeaker} else {max_rank.teleopspeaker};
            max_rank.teleoptrap = if (team_summary.points_trap > max_rank.teleoptrap) {team_summary.points_trap} else {max_rank.teleoptrap};
            max_rank.climbcount = if (team_summary.climb_count > max_rank.climbcount) {team_summary.climb_count} else {max_rank.climbcount};
            max_rank.amplifications = if (team_summary.amplifications > max_rank.amplifications) {team_summary.amplifications} else {max_rank.amplifications};
            max_rank.autonote1 = if (team_summary.autonote1 > max_rank.autonote1) {team_summary.autonote1} else {max_rank.autonote1};
            max_rank.autonote2 = if (team_summary.autonote2 > max_rank.autonote2) {team_summary.autonote2} else {max_rank.autonote2};
            max_rank.autonote3 = if (team_summary.autonote3 > max_rank.autonote3) {team_summary.autonote3} else {max_rank.autonote3};
            max_rank.autonote4 = if (team_summary.autonote4 > max_rank.autonote4) {team_summary.autonote4} else {max_rank.autonote4};
            max_rank.autonote5 = if (team_summary.autonote5 > max_rank.autonote5) {team_summary.autonote5} else {max_rank.autonote5};
            max_rank.autonote6 = if (team_summary.autonote6 > max_rank.autonote6) {team_summary.autonote6} else {max_rank.autonote6};
            max_rank.autonote7 = if (team_summary.autonote7 > max_rank.autonote7) {team_summary.autonote7} else {max_rank.autonote7};
            max_rank.autonote8 = if (team_summary.autonote8 > max_rank.autonote8) {team_summary.autonote8} else {max_rank.autonote8};
            
        };

        //let total_points_scored = (max_rank_count.low*point_values.low + max_rank_count.medium*point_values.medium + max_rank_count.high*point_values.high + max_rank_count.climb*point_values.climb);
        let total_points_scored = 1;
    
        for team in teams.values() {
            if comparison_team.teamNumber == team.teamNumber {
                continue;
            }
            //let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap()).constrain_values();
            let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap());
            let mut ranking = TeamRanking::default();
            ranking.teamNumber = team.teamNumber;
            ranking.autoamp_rating = team_summary.autoamp / max_rank.autoamp;
            ranking.autospeaker_rating = team_summary.autospeaker / max_rank.autospeaker;
            ranking.teleopamp_rating = team_summary.teleopamp / max_rank.teleopamp;
            ranking.teleopspeaker_rating = team_summary.teleopspeaker / max_rank.teleopspeaker;
            ranking.teleoptrap_rating = team_summary.points_trap / max_rank.teleoptrap;
            ranking.climbcount_rating = team_summary.climb_count / max_rank.climbcount;
            ranking.amplification_rating = team_summary.amplifications / max_rank.amplifications;
            ranking.data_reliability_rating = 1.0;
            ranking.autonote1_rating = team_summary.autonote1 / max_rank.autonote1;
            ranking.autonote2_rating = team_summary.autonote2 / max_rank.autonote2;
            ranking.autonote3_rating = team_summary.autonote3 / max_rank.autonote3;
            ranking.autonote4_rating = team_summary.autonote4 / max_rank.autonote4;
            ranking.autonote5_rating = team_summary.autonote5 / max_rank.autonote5;
            ranking.autonote6_rating = team_summary.autonote6 / max_rank.autonote6;
            ranking.autonote7_rating = team_summary.autonote7 / max_rank.autonote7;
            ranking.autonote8_rating = team_summary.autonote8 / max_rank.autonote8;

            //println!("teamNumber:{:?}  ampl:{} = totalAmps:{} / maxAmps:{} ",  ranking.teamNumber, ranking.amplification_rating, team_summary.amplifications, max_rank.amplifications);
            //ranking.overall_rating = (team_summary.avg_low*point_values.low + team_summary.avg_med*point_values.medium + team_summary.avg_high*point_values.high + team_summary.balance_percentage*point_values.balance + team_summary.dock_percentage*point_values.dock)/total_points_scored;
            ranking.overall_rating=1.0;
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
        //self.summary = Some(TeamSummary::new(&self).constrain_values());
        self.summary = Some(TeamSummary::new(&self));
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
        //self.match_data.push(entry.constrain_values());
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
