// APP VERSION
const VERSION = "v1.0.1";

// Imports
const {
  app,
  BrowserWindow,
  ipcMain,
  screen,
  dialog,
  net,
  shell,
} = require("electron");
const path = require("path");
const ytdl = require("ytdl-core");
const Store = require("electron-store");

const store = new Store({
  clearInvalidConfig: true,
});

let currRelease;

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require("electron-squirrel-startup")) {
  app.quit();
}

const createWindow = async () => {
  const primaryDisplay = screen.getPrimaryDisplay();
  const { width, height } = primaryDisplay.workAreaSize;

  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: Math.floor(width * 0.6),
    height: Math.floor(height * 0.75),
    minHeight: 300,
    minWidth: 300,
    webPreferences: {
      contextIsolation: true,
      preload: path.join(__dirname, "preload.js"),
    },
    frame: false,
  });
  const contents = mainWindow.webContents;

  // and load the index.html of the app.
  await mainWindow.loadFile(path.join(__dirname, "index.html"));
  contents.send("xel-dark-mode", store.get("darkMode", true));

  // mainWindow.webContents.openDevTools();

  ipcMain.on("window-close-request", function () {
    mainWindow.close();
  });
  ipcMain.on("xel-dark-mode", function (event, data) {
    store.set("darkMode", data);
  });
  ipcMain.on("open-release-request", function () {
    const regex =
      /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&\/\/=]*)/;
    if (net.online && regex.test(currRelease.url)) {
      shell.openExternal(currRelease.url);
      app.quit();
    }
  });

  contents.send("current-version", VERSION);

  if (net.online) {
    currRelease = await fetchLatestRelease();
    if (currRelease.name !== VERSION) {
      contents.send("version-outdated", VERSION, currRelease.name);
    }
  } else {
  }
};

app.on("ready", createWindow);

app.on("window-all-closed", () => {
  app.quit();
});

async function fetchLatestRelease() {
  const request = net.request({
    method: "GET",
    url: "https://ytconvertreleases.vercel.app",
  });
  let result;
  let resolveFunc;
  request.on("response", (response) => {
    let body = "";
    response.on("data", (chunk) => {
      body += chunk.toString();
    });
    response.on("end", () => {
      result = JSON.parse(body);
      resolveFunc();
    });
    response.on("error", (error) => {
      result = new Error("Ran into error fetching update");
      resolveFunc();
    });
  });
  request.end();
  await new Promise((resolve, reject) => {
    resolveFunc = resolve;
  });
  return result;
}

// const { dialog } = require("electron");
//   console.log(dialog.showOpenDialog({ properties: ["openDirectory"] }));

async function fetchThumbnail() {
  try {
    let thumbnails = (
      await ytdl.getInfo("https://www.youtube.com/watch?v=LpGotn0m_zs")
    ).videoDetails.thumbnails;
    return thumbnails[thumbnails.length - 1].url;
  } catch (error) {
    return "https://placehold.co/1920x1080";
  }
}

// // Create an listener for the event "A"
// ipcMain.on("A", (event, args) => {
//   console.log("A recieved");
//   // Send result back to renderer process
//   mainWindow.webContents.send("D", { success: true });
// });
