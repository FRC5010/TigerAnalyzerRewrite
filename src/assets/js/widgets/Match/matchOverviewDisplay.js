
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
      <div class="label">${team.team_number}</div>
                    <div class="team-name">${(team.tba_data) ? team.tba_data.nickname:"Team"} (${team.match_data.length} ${(team.match_data.length == 1) ? "Entry":"Entries" })</div>
                    <div class="cones-row">
                    <object class="caption-icon" data="./assets/svg/cone.svg" type=""></object>
                      <table class="cone-table">
                        <tr>
                          <th>Low</th>
                          <th>Medium</th>
                          <th>High</th>
                        </tr>
                        <tr>
                          <td>${round2Two(team.summary.avg_cone_low)}</td>
                          <td>${round2Two(team.summary.avg_cone_med)}</td>
                          <td>${round2Two(team.summary.avg_cone_high)}</td>
                        </tr>
                      </table>
                    </div>
                    <div class="cubes-row">
                      <object class="caption-icon" data="./assets/svg/cube.svg" type=""></object>
                      <table class="cube-table">
                        <tr>
                          <th>Low</th>
                          <th>Medium</th>
                          <th>High</th>
                        </tr>
                        <tr>
                          <td>${round2Two(team.summary.avg_cube_low)}</td>
                          <td>${round2Two(team.summary.avg_cube_med)}</td>
                          <td>${round2Two(team.summary.avg_cube_high)}</td>
                        </tr>
                      </table>
                    </div>
  
                <table class="station-table">
                  <tr>
                    <th class="station-text">Balance</th>
                    <th class="station-text">Dock %</th>
                    <th class="station-text">Engage %</th>
                  </tr>
                  <tr>
                    <td style="text-align:center;">${team.summary.can_balance.toString().toUpperCase()}</td>
                    <td><div class="percentage-bar ldBar label-center"></div></td>
                    <td><div class="percentage-bar ldBar label-center"></div></td>
                  </tr>
                </table>`
                
      let percentageBars = teamCard.getElementsByClassName("percentage-bar");
      new ldBar(percentageBars[0], { "value": Math.round(team.summary.balance_percentage*100), "stroke": "white", "preset": "circle"});
      new ldBar(percentageBars[1], { "value": Math.round(team.summary.dock_percentage*100), "stroke": "white", "preset": "circle"});
  
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