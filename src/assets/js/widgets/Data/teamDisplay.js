// TODO: Add Search Feature
// TODO: Team Comparison Feature
// TODO: Dot menu for teams to access more info

function round2Two(number) {
  return +(Math.round(number + "e+2")  + "e-2");
}

function fillRawTeamData(event) {
    let data = event.detail.scout_data;
    let teams = Object.values(data);
    
    let teamEntryList = document.querySelector(".data-team-raw-display");
    teams.forEach(team => {
      let teamEntry = document.createElement("div");
      teamEntry.classList.add("team-entry")
      // May be unwise to use innerHtml as it will execute any html put in the variables...but its probably fine...
      teamEntry.innerHTML = `
              <div class="label">${team.team_number}</div>
              <div class="team-name">${team.tba_data.nickname} (${team.match_data.length} ${(team.match_data.length == 1) ? "Entry":"Entries" })</div>
              <table class="cone-table">
                <caption><object class="caption-icon" data="./assets/svg/cone.svg" type=""></object></caption>
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
              <table class="cube-table">
                <caption><object class="caption-icon" data="./assets/svg/cube.svg" type=""></object></caption>
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
              <table class="station-table">
                <caption>Charge Station</caption>
                <tr>
                  <th>Can Balance</th>
                  <th>Docked %</th>
                  <th>Engaged %</th>
                </tr>
                <tr>
                  <td>${team.summary.can_balance}</td>
                  <td>${round2Two(team.summary.dock_percentage*100)}</td>
                  <td>${round2Two(team.summary.balance_percentage*100)}</td>
                </tr>
              </table>
      `

      teamEntryList.appendChild(teamEntry);

    });
}
  
document.addEventListener("data_loaded", fillRawTeamData);
  