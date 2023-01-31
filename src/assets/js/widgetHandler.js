const fs = window.__TAURI__.fs;
const path = window.__TAURI__.path;

var Data;

async function initializeWidgetScripts(data) {
    const widget_data_loaded = new CustomEvent("data_loaded", {
        detail: {"scout_data": data},
        bubbles: true,
        cancelable: false,
        composed: true,
      });
    document.querySelector(".grids").dispatchEvent(widget_data_loaded);
    const resourceDirPath = await path.resourceDir();
    console.log(resourceDirPath)
    console.log(fs.readDir(path.join(resourceDirPath, "/widgets")));
}