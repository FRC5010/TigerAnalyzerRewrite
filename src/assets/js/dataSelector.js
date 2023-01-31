const { invoke } = window.__TAURI__.tauri;
const dialog = window.__TAURI__.dialog;
var filePath = null;

async function populate_results(data) {
  initializeWidgetScripts(data);
  Promise.resolve();
} 

async function open_results() {
  if (!filePath) return

  invoke('submit_data', {'dataPath':filePath}).then(async (scout_data) => {
    console.log(scout_data);
    await populate_results(scout_data).then(() => {document.querySelector('.navbar').scrollIntoView(true)});
  }, (reason) => { console.error(reason);});
}

async function get_filePath() {
  filePath = await dialog.open({
    multiple: false,
    
  })
}



window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector(".submitBtn")
    .addEventListener("click", () => open_results());
  document
    .querySelector(".dataSelectBtn")
    .addEventListener("click", () => get_filePath());
});


