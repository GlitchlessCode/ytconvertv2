// APP VERSION
const VERSION = "v1.0.2";

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
const { createMachine, interpret } = require("xstate");

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
  ipcMain.on("xstate-event", function (event, data) {
    const snapshot = StateService.send(data);
    contents.send(
      "xstate-transitioned",
      generateInterpretation(snapshot.value)
    );
  });

  contents.send("current-version", VERSION);

  StateService.start();
  const snapshot = StateService.send("INIT");
  contents.send("xstate-transitioned", generateInterpretation(snapshot.value));

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

// * Functions

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

function parseStateTree(obj) {
  const result = {};
  for (const [key, value] of Object.entries(obj)) {
    result[key] = {};
    switch (typeof value) {
      case "string":
        result[key][value] = true;
        break;
      case "object":
        result[key] = parseStateTree(value);
        break;
    }
  }
  return result;
}

function generateInterpretation(obj) {
  const parsedTree = parseStateTree(obj);
  const result = {
    LocationSection: !!parsedTree.LocationSelector.enabled,
    SingleSection: !!parsedTree.SingleVideoFetcher.enabled,
    MultiSection: !!parsedTree.MultiVideoFetcher.enabled,
    SingleFetchBtn:
      !!parsedTree.SingleVideoFetcher.enabled &&
      (!!parsedTree.SingleVideoFetcher.enabled.not_ready ||
        !!parsedTree.SingleVideoFetcher.enabled.ready),
    SingleExtractBtn:
      !!parsedTree.SingleVideoFetcher.enabled &&
      !!parsedTree.SingleVideoFetcher.enabled.ready,
    SingleCancelBtn:
      !!parsedTree.SingleVideoFetcher.enabled &&
      !!parsedTree.SingleVideoFetcher.enabled.extracting,
    MultiFetchBtn:
      !!parsedTree.MultiVideoFetcher.enabled &&
      (!!parsedTree.MultiVideoFetcher.enabled.not_ready ||
        !!parsedTree.MultiVideoFetcher.enabled.ready),
    MultiExtractBtn:
      !!parsedTree.MultiVideoFetcher.enabled &&
      !!parsedTree.MultiVideoFetcher.enabled.ready,
    MultiCancelBtn:
      !!parsedTree.MultiVideoFetcher.enabled &&
      !!parsedTree.MultiVideoFetcher.enabled.extracting,
    FileLocationValid: true,
  };
  return result;
}

// * State Machine
const StateManager = createMachine(
  {
    type: "parallel",
    id: "StateManager",
    predictableActionArguments: true,
    states: {
      LocationSelector: {
        initial: "init",
        states: {
          enabled: {
            on: {
              ENTER_URL: [
                {
                  target: [
                    "#StateManager.SingleVideoFetcher.enabled.hist",
                    "#StateManager.MultiVideoFetcher.enabled.hist",
                  ],
                  cond: "validLocation",
                },
                {
                  target: [
                    "#StateManager.SingleVideoFetcher.disabled",
                    "#StateManager.MultiVideoFetcher.disabled",
                  ],
                },
              ],
            },
          },
          disabled: {},
          init: {
            on: {
              INIT: {
                target: "enabled",
              },
            },
          },
        },
      },
      SingleVideoFetcher: {
        initial: "init",
        states: {
          init: {
            on: {
              INIT: [
                {
                  target: "enabled.hist",
                  cond: "validLocation",
                },
                { target: "disabled" },
              ],
            },
          },
          enabled: {
            initial: "not_ready",
            states: {
              not_ready: {
                on: {
                  FETCH_VIDEO_URL: [
                    {
                      target: "ready",
                      cond: "validVideoUrl",
                    },
                  ],
                },
              },
              ready: {
                on: {
                  VIDEO_URL_CHANGE: {
                    target: "not_ready",
                  },
                  START_VIDEO_EXTRACT: {
                    target: [
                      "extracting",
                      "#StateManager.LocationSelector.disabled",
                      "#StateManager.MultiVideoFetcher.disabled",
                    ],
                  },
                },
              },
              extracting: {
                on: {
                  CANCEL_VIDEO_EXTRACT: {
                    target: [
                      "ready",
                      "#StateManager.LocationSelector.enabled",
                      "#StateManager.MultiVideoFetcher.enabled.hist",
                    ],
                  },
                  FINISH_VIDEO_EXTRACT: {
                    target: [
                      "not_ready",
                      "#StateManager.LocationSelector.enabled",
                      "#StateManager.MultiVideoFetcher.enabled.hist",
                    ],
                  },
                },
              },

              hist: {
                type: "history",
                history: "shallow",
              },
            },
          },
          disabled: {},
        },
      },
      MultiVideoFetcher: {
        initial: "init",
        states: {
          init: {
            on: {
              INIT: [
                {
                  target: "enabled.hist",
                  cond: "validLocation",
                },
                { target: "disabled" },
              ],
            },
          },
          enabled: {
            initial: "not_ready",
            states: {
              not_ready: {
                on: {
                  FETCH_PLAYLIST_URL: [
                    {
                      target: "ready",
                      cond: "validVideoUrl",
                    },
                  ],
                },
              },
              ready: {
                on: {
                  PLAYLIST_URL_CHANGE: {
                    target: "not_ready",
                  },
                  START_PLAYLIST_EXTRACT: {
                    target: [
                      "extracting",
                      "#StateManager.LocationSelector.disabled",
                      "#StateManager.SingleVideoFetcher.disabled",
                    ],
                  },
                },
              },
              extracting: {
                on: {
                  CANCEL_PLAYLIST_EXTRACT: {
                    target: [
                      "ready",
                      "#StateManager.LocationSelector.enabled",
                      "#StateManager.SingleVideoFetcher.enabled.hist",
                    ],
                  },
                  FINISH_PLAYLIST_EXTRACT: {
                    target: [
                      "not_ready",
                      "#StateManager.LocationSelector.enabled",
                      "#StateManager.SingleVideoFetcher.enabled.hist",
                    ],
                  },
                },
              },

              hist: {
                type: "history",
                history: "shallow",
              },
            },
          },
          disabled: {},
        },
      },
    },
  },
  {
    guards: {
      validLocation: function () {
        return true;
      },
      validVideoUrl: function () {
        return true;
      },
      validPlaylistUrl: function () {
        return true;
      },
    },
  }
);

const StateService = interpret(StateManager);
