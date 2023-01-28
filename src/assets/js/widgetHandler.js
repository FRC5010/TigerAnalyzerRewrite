const fs = window.__TAURI__.fs;
var Data;

function initializeWidgetScripts(data) {
    Data = data;
    
    console.log(fs.readDir("./widgets"));
}