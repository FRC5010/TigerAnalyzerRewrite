
function getValueStroke(value) {
    if (value < 30) return "red";
    else if (value < 60) return "orange";
    else if (value < 90) return "lightgreen";
    else return "limegreen";
  }
  
  function updateTeamCard(event, data) {
      let teamCard = document.getElementById(event.srcElement.dataset.cardId);
      
      let teamNumber = event.srcElement.value;
      if (!teamNumber) return;

      let team = data[teamNumber];
      // TODO: Implement Charge Station Stats
      // TODO: Add Progress/Circle Percentage Meters for Charge Station Stats
      // TODO: Add Export to Pretty Image/Banner of the Match
      
      teamCard.innerHTML = `
      <div class="label">${team.teamNumber}</div>
                    <div class="team-name">${(team.tba_data) ? team.tba_data.nickname:"Team"} (${team.match_data.length} ${(team.match_data.length == 1) ? "Entry":"Entries" })</div>
                    <div class="cones-row">
                    <object class="caption-icon" data="" type=""></object>
                      <table class="cone-table">
                        <tr>
                          <th>Auton Amp Avg</th>
                          <th>Auto Speaker Avg</th>
                        </tr>
                        <tr>
                          <td>${round2Two(team.summary.auton_amp_avg)}</td>
                          <td>${round2Two(team.summary.auton_speaker_avg)}</td>
                        </tr>
                      </table>
                    </div>
                    <div class="cubes-row">
                      <object class="caption-icon" data="" type=""></object>
                      <table class="cube-table">
                        <tr>
                          <th>Amp Avg</th>
                          <th>Speaker Avg</th>
                        </tr>
                        <tr>
                          <td>${round2Two(team.summary.total_amp_avg)}</td>
                          <td>${round2Two(team.summary.total_speaker_avg)}</td>
                        </tr>
                      </table>
                    </div>
  
                <table class="station-table">
                  <tr>
                    <th class="station-text">Endgame Trap</th>
                    <th class="station-text">Endgame Climb</th>
                  </tr>
                  <tr>
                    <td style="text-align:center;">${team.summary.points_trap}</td>
                    <td style="text-align:center;">${team.summary.climb_count}</td>
                  </tr>
                </table>`
                
//      let percentageBars = teamCard.getElementsByClassName("percentage-bar");
//      new ldBar(percentageBars[0], { "value": Math.round(team.summary.balance_percentage*100), "stroke": "white", "preset": "circle"});
//      new ldBar(percentageBars[1], { "value": Math.round(team.summary.dock_percentage*100), "stroke": "white", "preset": "circle"});
  
   }
  
  function initializeMatchOverview(event) {
      let data = event.detail.scout_data;
      let teamNumbers = Object.keys(data);
  
      let teamOptionsHtml = "<option value='' selected disabled hidden>Team</option>"
      teamNumbers.forEach(teamNumber => {
          
          teamOptionsHtml += `<option value=${teamNumber}>${teamNumber}</option>`
          
      })
  
      let teamSelects = document.getElementsByClassName("team-select");
      for (const select of teamSelects) {
          select.innerHTML = teamOptionsHtml;
          select.addEventListener("input", (event) => {updateTeamCard(event, data)});
      }
  
  }
  
  document.addEventListener("data_loaded", initializeMatchOverview);