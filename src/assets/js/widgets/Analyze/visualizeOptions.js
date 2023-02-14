let teamOptionsBox = document.querySelector(".team-select-grid");

function round2Two(number) {
    return +(Math.round(number + "e+2")  + "e-2");
}

function unselectAllOptions() {
    let options = document.querySelector(".team-select-grid").getElementsByClassName("team-option");
    for (let option of options) {
        option.classList.remove("active");
    }
}

function selectAllOptions() {
    let options = document.querySelector(".team-select-grid").getElementsByClassName("team-option");
    for (let option of options) {
        option.classList.add("active");
    }
}

function toggleOption(event) {
    let option = event.srcElement;
    if (option.classList.contains("active")) option.classList.remove("active");
    else option.classList.add("active");
}

function getValueStroke(value) {
    if (value < 30) return "red";
    else if (value < 60) return "orange";
    else if (value < 90) return "lightgreen";
    else return "limegreen";
}

function populateOptions(data) {
    let teams = Object.values(data);
    let options = document.querySelector(".team-select-grid");
    teams.forEach(team => {
        let option = document.createElement("div");
        option.classList.add("team-option");
        option.textContent = team.team_number;
        // ADD Event Listener for handling click/unclick
        option.addEventListener("click", toggleOption)
        options.appendChild(option);
    });

    invoke('get_team_rankings', {'teamData':data}).then((team_rankings) => {
        console.log(team_rankings);
        let rankings = document.querySelector(".analysis-ranking-board");

        /*
        Make a new div for each team in the rankings in this format using string literals and innerHTML:
        <div class="analysis-rank-entry">
                <div class="id-data">
                  <div class="team-number">5010</div>
                </div>
                <div class="rate-data">
                  <div class="sub-rating rating-bar ldBar label-center" data-preset="circle" data-value="20"></div>
                  <div class="sub-rating rating-bar ldBar label-center" data-preset="circle" data-value="90"></div>
                  <div class="sub-rating rating-bar ldBar label-center" data-preset="circle" data-value="40"></div>
                  <div class="overall-rating rating-bar ldBar label-center" data-preset="circle" data-value="50"></div>
                </div>
              </div>
        */
        team_rankings.forEach(team => {
            div = document.createElement("div");
            div.classList.add("analysis-rank-entry");
            div.innerHTML = `
            <div class="id-data">
                <div class="team-number">${team.team_number}</div>
            </div>  
            <div class="rate-data">
                  <div class="sub-rating rating-bar ldBar label-center"></div>
                  <div class="sub-rating rating-bar ldBar label-center"></div>
                  <div class="sub-rating rating-bar ldBar label-center"></div>
                  <div class="overall-rating rating-bar ldBar label-center"></div>
                </div>
              `;

            let ratings = div.getElementsByClassName("rating-bar");
            let piece_rating_average = Math.round(((team.low_rating+team.medium_rating+team.high_rating)/3)*100);
            new ldBar(ratings[0], { "value": piece_rating_average, "stroke": getValueStroke(piece_rating_average), "preset": "circle"});
            new ldBar(ratings[1], { "value": Math.round(team.dock_rating*100), "stroke": getValueStroke(Math.round(team.dock_rating*100)), "preset": "circle"});
            new ldBar(ratings[2], { "value":Math.round(team.data_reliability_rating*100), "stroke": getValueStroke(Math.round(team.data_reliability_rating*100)), "preset": "circle"});
            new ldBar(ratings[3], { "value": Math.round(team.overall_rating*100), "stroke": getValueStroke(Math.round(team.overall_rating*100)), "stroke-width":5, "preset": "circle"});
            rankings.appendChild(div);

        });

    }, (reason) => { console.error(reason);});
}

function initialize(event) {
    let scout_data = event.detail.scout_data;

    populateOptions(scout_data);
}

document.addEventListener("data_loaded", initialize)
teamOptionsBox.querySelector(".select-none").addEventListener("click", unselectAllOptions);
teamOptionsBox.querySelector(".select-all").addEventListener("click", selectAllOptions);

