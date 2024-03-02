/*
<div class="entry">
                  <p>123</p>
                  <p>5010</p>
                </div>
                <div class="entry"></div>
*/
function fillRawMatchData(event) {
  let data = event.detail.scout_data;
  let teams = Object.values(data);

  teams.forEach(team => {
    let label = document.createElement("div");
    label.classList.add("label");
    label.textContent = team.teamNumber;
    document.querySelector(".data-match-raw-display").appendChild(label);
    team.match_data.forEach(match => {
      let entry = document.createElement("div");
      entry.classList.add("entry");

      Object.values(match).forEach(value => {
        let dataPoint = document.createElement("p");
        dataPoint.textContent = value;
        entry.appendChild(dataPoint);
      })

      document.querySelector(".data-match-raw-display").appendChild(entry);
    })
  });
}


document.addEventListener("data_loaded", fillRawMatchData)
