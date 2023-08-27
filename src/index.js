const { app, BrowserWindow, ipcMain, screen, dialog } = require("electron");
const path = require("path");
const ytdl = require("ytdl-core");

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require("electron-squirrel-startup")) {
  app.quit();
}

const createWindow = () => {
  const primaryDisplay = screen.getPrimaryDisplay();
  const { width, height } = primaryDisplay.workAreaSize;

  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: Math.floor(width * 0.75),
    height: Math.floor(height * 0.75),
    webPreferences: {
      contextIsolation: true,
      preload: path.join(__dirname, "preload.js"),
    },
    frame: false,
  });

  // and load the index.html of the app.
  mainWindow.loadFile(path.join(__dirname, "index.html"));

  // mainWindow.webContents.openDevTools();

  ipcMain.on("window-close-request", function () {
    mainWindow.close();
  });
};

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on("ready", createWindow);

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  // On OS X it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and import them here.

// async function fetchLatestRelease() {
//   let xmlRequest = new XMLHttpRequest();
//   let result;
//   let resolveFunc;
//   xmlRequest.onreadystatechange = function() {
//       if(this.readyState == 4) {
//           if(this.status == 200){
//               result = JSON.parse(this.responseText);
//           } else {
//               result = new Error("Ran into error fetching update");
//           }
//           resolveFunc();
//       }
//   }
//   xmlRequest.open("GET", "https://api.github.com/repos/TurtleY0da/ytconvertv2/releases/latest");
//   xmlRequest.send();
//   await new Promise((resolve, reject)=>{resolveFunc = resolve});
//   return result;
// }

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
