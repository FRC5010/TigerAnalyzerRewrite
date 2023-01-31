function fillRawTeamData(event) {
    let data = event.detail.scout_data;
    let teams = Object.values(data);

    let teamEntryList = document.querySelector(".data-team-raw-display");
    teams.forEach(team => {
      let teamEntry = document.createElement("div");
      teamEntry.classList.add("team-entry");
      
      let label = document.createElement("div");
      label.classList.add("label");
      label.textContent = team.team_number;
      teamEntry.appendChild(label);

      let name = document.createElement("div");
      name.classList.add("name")
      if (team.tba_data.nickname) {name.textContent = team.tba_data.nickname};
      teamEntry.appendChild(name);

      if (team.summary) {
        let coneTable = document.createElement("table");
        coneTable.classList.add("cone-table");
        
        coneTable.innerHTML = `
        <caption><object class="caption-icon" data="./assets/svg/cone.svg" type=""></object></caption>
        <tr>
          <th>Low</th>
          <th>Medium</th>
          <th>High</th>
        </tr>
        <tr>
          <td>${team.summary.avg_cone_low}</td>
          <td>${team.summary.avg_cone_med}</td>
          <td>${team.summary.avg_cone_high}</td>
        </tr>
        `

        teamEntry.appendChild(coneTable);
        
        // May or may not be able to just inject any html code by setting team summary variables...probably ok...
        let cubeTable = document.createElement("table");
        cubeTable.classList.add("cube-table");
        cubeTable.innerHTML = `
        <caption><object class="caption-icon" data="./assets/svg/cube.svg" type=""></object></caption>
        <tr>
          <th>Low</th>
          <th>Medium</th>
          <th>High</th>
        </tr>
        <tr>
          <td>${team.summary.avg_cube_low}</td>
          <td>${team.summary.avg_cube_med}</td>
          <td>${team.summary.avg_cube_high}</td>
        </tr>`

        teamEntry.appendChild(cubeTable);


        let stationTable = document.createElement("table");
        stationTable.classList.add("station-table");
        stationTable.innerHTML = `
        <caption>Charge Station</caption>
        <tr>
          <th>Can Balance</th>
          <th>Docked %</th>
          <th>Engaged %</th>
        </tr>
        <tr>
          <td>None</td>
          <td>None</td>
          <td>None</td>
        </tr>
        `

        teamEntry.appendChild(stationTable);

      }

      teamEntryList.appendChild(teamEntry);

    });
}
  
document.addEventListener("data_loaded", fillRawTeamData);
  