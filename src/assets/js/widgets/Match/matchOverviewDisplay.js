
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
                  <div class="team-name">${(team.tba_data.nickname) ? team.tba_data.nickname:"Team"} (${team.match_data.length} ${(team.match_data.length == 1) ? "Entry":"Entries" })</div>
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
              <!-- NOT IMPLEMENTED
              <table class="station-table">
                <tr>
                  <th>Balance</th>
                  <th>Docked %</th>
                  <th>Engaged %</th>
                </tr>
                <tr>
                  <td>None</td>
                  <td>None</td>
                  <td>None</td>
                </tr>
              </table> -->
    `

 }

function initializeMatchOverview(event) {
    let data = event.detail.scout_data;
    let teamNumbers = Object.keys(data);

    let teamOptionsHtml = "<select class='team-select'><option value='' selected disabled hidden>Team</option></select>"
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