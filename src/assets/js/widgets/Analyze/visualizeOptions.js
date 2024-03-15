function round2Two(number) {
    return +(Math.round(number + "e+2")  + "e-2");
}

function resetRankings(data) {
    document.querySelector(".rankings").innerHTML = "";
}

function getValueStroke(value) {
    if (value < 30) return "red";
    else if (value < 60) return "orange";
    else if (value < 90) return "lightgreen";
    else return "limegreen";
}


function populate_rankings(data, options) {
    resetRankings(data);
    if (options["comparison_team"] == "none") options["comparison_team"] = null;
    invoke('get_team_rankings', {'teamData':data, 'options':options}).then((team_rankings) => {
        let rankings = document.querySelector(".rankings");


        for (let i = 0; i < team_rankings.length; i++) {
            let team = team_rankings[i];
            div = document.createElement("div");
            div.classList.add("analysis-rank-entry");
            div.innerHTML = `
            <div class="id-data">
                <div class="rank">#${i+1}</div>
                <div class="team-number">${team.teamNumber}</div>
            </div>  
            <div class="rate-data">
                <div class="rating-group" >
                    <div class="rating-label">amplifications</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group">
                    <div class="rating-label">autoamp</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group">
                    <div class="rating-label">autospeaker</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group">
                    <div class="rating-label">teleopamp</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group">
                    <div class="rating-label">teleopspeaker</div> 
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group" >
                    <div class="rating-label">teleoptrap</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group" >
                    <div class="rating-label">climbcount</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                
                <!--<div class="overall-rating rating-bar ldBar label-center"></div>
                </div> -->
              `;

            let ratings = div.getElementsByClassName("rating-bar");
            function makeRating(element, rating) {
                let display_rating = Math.round(rating*100);
                new ldBar(element, { "value": display_rating, "stroke": getValueStroke(display_rating), "preset": "circle"});
            }

            //Where are these variables declared?
            let piece_rating_average = Math.round(((team.low_rating+team.medium_rating+team.high_rating)/3)*100);
            makeRating(ratings[0], team.amplification_rating);
            makeRating(ratings[1], team.autoamp_rating);
            makeRating(ratings[2], team.autospeaker_rating);
            makeRating(ratings[3], team.teleopamp_rating);
            makeRating(ratings[4], team.teleopspeaker_rating);
            makeRating(ratings[5], team.teleoptrap_rating);
            makeRating(ratings[6], team.climbcount_rating);
            
            //makeRating(ratings[7], team.overall_rating);
            rankings.appendChild(div);

        }
    }, (reason) => { console.error(reason);});
}

function populateOptions(data) {
    let team_keys = Object.keys(data);

    let teamOptionsHtml = "<option value='none' selected>Individual</option>"
    team_keys.forEach(teamNumber => {
        teamOptionsHtml += `<option value=${teamNumber}>${teamNumber}</option>`
    });
    document.querySelector(".comparison-team-select").innerHTML = teamOptionsHtml;

    // Adds an event listener to the team select dropdown
    document.querySelector(".comparison-team-select").addEventListener("change", (event) => {
        let teamNumber = event.target.value;
        let team = data[teamNumber];

        let sortorder = document.querySelector(".sortorder-select").value;
        populate_rankings(data, {"comparison_team": team, "sort_order": sortorder})        
    });
    
    // Adds an event listener to the team select dropdown
    document.querySelector(".sortorder-select").addEventListener("change", (event) => {
        let teamNumber = document.querySelector(".comparison-team-select").value;
        let team = data[teamNumber];

        let sortorder = event.target.value;
        populate_rankings(data, {"comparison_team": team, "sort_order": sortorder})        
    });
    

    populate_rankings(data, {});

    
}

function initialize(event) {
    let scout_data = event.detail.scout_data;
    populateOptions(scout_data);
}

document.addEventListener("data_loaded", initialize)

