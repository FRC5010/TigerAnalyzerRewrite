// TODO: Add Search Feature
// TODO: Team Comparison Feature
// TODO: Dot menu for teams to access more info
//
//climb_count is a copy of the parameters balance_count.
//The variable "climb" is a copy of the variable balance.
//Edit the Docked, engaged, etc. variables

function round2Two(number) {
    return +(Math.round(number + "e+2")  + "e-2");
  }
  
  function toggleCardOptions(event) {
    let teamCard = event.target.parentElement;
    let optionsMenu = teamCard.querySelector(".card-options-menu");
    console.log(event);
    if (optionsMenu.classList.contains("visible")) {
      console.log("visible")
      optionsMenu.classList.remove("visible")
    } else {
      console.log("not visible")
      optionsMenu.classList.add("visible");
    }
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
                <div class="label">${team.teamNumber}</div>
                <div class="team-name">${(team.tba_data) ? team.tba_data.nickname:"Team"} (${team.match_data.length} ${(team.match_data.length == 1) ? "Entry":"Entries" })</div>
                <table class="cone-table">
                  <caption>Amplifications</caption>
                  <tr>
                    <td>${round2Two(team.summary.amplifications)}</td>
                  </tr>

                </table>
                <table class="cone-table">
                  <caption>Autonomous</caption>
                  <tr>
                    <th>Total&nbsp;/&nbsp;Avg Amp</th>
                    <th>Total&nbsp;/&nbsp;Avg Speaker</th>
                  </tr>
                  <tr> 
                    <td>${round2Two(team.summary.auton_amp)}&nbsp;/&nbsp;${round2Two(team.summary.auton_amp_avg.toFixed(3))}</td> <!-- auto amp, perviously auton_amp --->
                    <td>${round2Two(team.summary.auton_speaker)}&nbsp;/&nbsp;${round2Two(team.summary.auton_speaker_avg.toFixed(3))}</td> <!-- auto speaker, previously auton_speaker -->
                  </tr>
                </table>
                <table class="cone-table"> 
                  <caption>Teleop</caption>
                  <tr>
                    <th>Total&nbsp;/&nbsp;Avg Amp</th>
                    <th>Total&nbsp;/&nbsp;Avg Speaker</th>
                  </tr>
                  <tr>
                    <td>${round2Two(team.summary.teleopamp)} / ${round2Two(team.summary.teleopamp_avg.toFixed(3))}</td> 
                    <td>${round2Two(team.summary.total_speaker)} / ${round2Two(team.summary.total_speaker_avg.toFixed(3))}</td> 
                  </tr>
                </table>
                <table class="station-table">
                  <caption>Endgame</caption>
                  <tr>
                    <th>Points: Trap</th>
                    <th>Climb Count</th>
                    <th>Climb %</th>
                  </tr>
                  <tr> 
                    <td>${round2Two(team.summary.points_trap)}</td> <!-- points_trap --->
                    <td>${round2Two(team.summary.climb_count)}</td>
                    <td>${round2Two(team.summary.climb_percentage)}</td> <!--- climb_percentage*100 --->
                  </tr>
                </table>
        `
        teamEntryList.appendChild(teamEntry);
  
        
  
      });
      Array.from(teamEntryList.getElementsByClassName("dot-menu")).forEach(menu => {
        menu.addEventListener("click", toggleCardOptions);
        console.log("added event listener");
      })
  }
    
  document.addEventListener("data_loaded", fillRawTeamData);
    
  
  /*                  3rd Section of 1st part: <th>Points (Unamplified Speaker)</th>
                      <td>${round2Two(team.summary.points_unamplified_speaker)}</td>
                                          <th>Points: Amplified Speaker</th>
                                          (Cone Image upload code) <caption>omous/Teleoperated<object class="caption-icon" data="./assets/svg/cone.svg" type=""></object></caption>
                                          (Cube Image upload code) <caption><object class="caption-icon" data="./assets/svg/cube.svg" type=""></object></caption>
                                          */
  