let teamOptionsBox = document.querySelector(".team-select-grid");



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
    })
}

function initialize(event) {
    let scout_data = event.detail.scout_data;

    populateOptions(scout_data);
}

document.addEventListener("data_loaded", initialize)
teamOptionsBox.querySelector(".select-none").addEventListener("click", unselectAllOptions);
teamOptionsBox.querySelector(".select-all").addEventListener("click", selectAllOptions);
