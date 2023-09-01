// APP VERSION
const VERSION = "v1.2.1";

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
const ytpl = require("ytpl");
const Store = require("electron-store");
const { createMachine, interpret, assign } = require("xstate");
const fs = require("fs");
const { raise } = require("xstate/lib/actions");
const ffmpeg = require("fluent-ffmpeg");
const theoreticalPath = require("ffmpeg-static");
const fileName = path.join(
  process.resourcesPath,
  theoreticalPath.match(/[\\\/](?:.(?![\\\/]))+$/)[0]
);
if (fs.existsSync(fileName)) {
  ffmpeg.setFfmpegPath(fileName);
} else {
  ffmpeg.setFfmpegPath(theoreticalPath);
}

const store = new Store({
  clearInvalidConfig: true,
});

let currRelease;
let currStatus;
let currLocation = store.get("location", undefined);
let currExportSettings = {
  video: {
    type: "audio",
    ext: "wav",
  },
  playlist: {
    type: "audio",
    ext: "wav",
  },
};

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require("electron-squirrel-startup")) {
  app.quit();
}

/** @type {import("electron").WebContents} */
let webContents;

const createWindow = async () => {
  const primaryDisplay = screen.getPrimaryDisplay();
  const { width, height } = primaryDisplay.workAreaSize;

  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: Math.floor(width * 0.6),
    height: Math.floor(height * 0.75),
    minHeight: 300,
    minWidth: 300,
    icon: path.join(__dirname, "/images/ytconvertv2_logo.png"),
    webPreferences: {
      devTools: false,
      contextIsolation: true,
      preload: path.join(__dirname, "preload.js"),
    },
    frame: false,
  });

  webContents = mainWindow.webContents;

  mainWindow.setBackgroundColor(
    store.get("darkMode", true) ? "#242424" : "#f5f5f5"
  );

  // and load the index.html of the app.
  await mainWindow.loadFile(path.join(__dirname, "index.html"));

  webContents.send("xel-dark-mode", store.get("darkMode", true));

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
  ipcMain.on("open-wiki-request", function () {
    const regex =
      /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&\/\/=]*)/;
    if (net.online && regex.test(currRelease.wiki)) {
      shell.openExternal(currRelease.wiki);
    }
  });
  ipcMain.on("open-issues-request", function () {
    const regex =
      /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&\/\/=]*)/;
    if (net.online && regex.test(currRelease.issues)) {
      shell.openExternal(currRelease.issues);
    }
  });
  ipcMain.on("xstate-event", async function (event, eventName, data) {
    StateService.send(eventName, { data: data });
  });

  ipcMain.on("location-select-request", async function () {
    if (currStatus.LocationSection === true) {
      const location = await dialog.showOpenDialog({
        properties: ["openDirectory"],
      });
      if (!location.canceled) {
        currLocation = location.filePaths[0].replaceAll("\\", "/");
        store.set("location", currLocation);
      }
      StateService.send("ENTER_LOCATION");
      webContents.send("location", currLocation);
    }
  });

  ipcMain.on(
    "selector-change",
    /**
     * @param {"video"|"playlist"} exportType
     * @param {"audio"|"video"} fileType
     * @param {"wav"|"mp3"|"mp4"|"mov"} fileExtension
     */
    function (event, exportType, fileType, fileExtension) {
      currExportSettings[exportType].type = fileType;
      currExportSettings[exportType].ext = fileExtension;
    }
  );

  StateService.onTransition(() => {
    const snapshot = StateService.getSnapshot();
    currStatus = generateInterpretation(snapshot.value);
    webContents.send("xstate-transitioned", currStatus, snapshot.context);
  });

  StateService.start();
  StateService.send("INIT");

  webContents.send("current-version", VERSION);

  webContents.send("location", currLocation);

  webContents.send("connection-status", net.online);

  if (net.online) {
    currRelease = await fetchLatestRelease();
    if (currRelease.name !== VERSION) {
      webContents.send("version-outdated", VERSION, currRelease.name);
    }
  }
};

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

async function fetchVideoInfo(ytUrl) {
  const result = {
    fetchedUrl: undefined,
    thumb: undefined,
    title: undefined,
    author: undefined,
    adjustedValue: undefined,
    adjustedValueTitle: "Duration",
    date: undefined,
  };
  const wrapper = {};
  try {
    if (!ytdl.validateURL(ytUrl)) throw new Error();
    const info = (await ytdl.getInfo(ytUrl)).videoDetails;
    const thumbnails = info.thumbnails;
    const date = new Date(info.publishDate);
    const months = [
      "January",
      "February",
      "March",
      "April",
      "May",
      "June",
      "July",
      "August",
      "September",
      "October",
      "November",
      "December",
    ];

    const totalSeconds = parseInt(info.lengthSeconds);
    const seconds = (totalSeconds % 60).toString().padStart(2, "0");
    const minutes = (Math.floor(totalSeconds / 60) % 60)
      .toString()
      .padStart(2, "0");
    const hours = Math.floor(totalSeconds / 3600)
      .toString()
      .padStart(2, "0");

    result.fetchedUrl = ytUrl;
    result.thumb = thumbnails[thumbnails.length - 1].url;
    result.title = info.title;
    result.author = {
      avatar: info.author.thumbnails[0],
      name: info.author.name,
    };
    result.adjustedValue = `${hours}:${minutes}:${seconds}`;
    result.date = `Uploaded on <b>${
      months[date.getUTCMonth()]
    } ${date.getUTCDate()}, ${date.getUTCFullYear()}</b>`;
    wrapper.valid = true;
  } catch (error) {
    result.title = "No Video Found";
    result.thumb = "https://placehold.co/1920x1080";
    wrapper.valid = false;
  }
  wrapper.result = result;
  return wrapper;
}

async function fetchPlaylistInfo(ytUrl) {
  const result = {
    fetchedUrl: undefined,
    thumb: undefined,
    title: undefined,
    author: undefined,
    adjustedValue: undefined,
    adjustedValueTitle: "Videos",
    date: undefined,
  };
  const wrapper = {};
  try {
    if (!ytpl.validateID(ytUrl)) throw new Error();
    const info = await ytpl(ytUrl, { pages: Infinity });
    result.fetchedUrl = ytUrl;
    result.thumb = info.bestThumbnail.url;
    result.title = info.title;
    result.author = {
      avatar: info.author.avatars[info.author.avatars.length - 1],
      name: info.author.name,
    };
    result.adjustedValue = info.items.length;
    result.date = info.lastUpdated;
    wrapper.valid = true;
  } catch (error) {
    result.title = "No Playlist Found";
    result.thumb = "https://placehold.co/1920x1080";
    wrapper.valid = false;
  }
  wrapper.result = result;
  return wrapper;
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
    FileLocationValid: fs.existsSync(currLocation),
  };
  return result;
}

let videoReference = undefined;
function extractVideoFromURL(
  location,
  url,
  name,
  event,
  extension,
  videoFinish
) {
  let regex = /[\\\/:*?<>|]/gm;
  let cutTitle = name.replace(regex, "_");
  const outputLocation = `${location}/${cutTitle}.${extension}`;
  const video = ytdl(url, { filter: "audioonly", quality: "highestaudio" });
  webContents.send(event, 0);
  video.addListener("progress", function (_, current, total) {
    webContents.send(event, Math.floor((current / total) * 100));
  });
  videoReference = video;
  video.addListener("end", function () {
    videoReference = undefined;
    videoFinish();
  });
  ffmpeg()
    .input(video)
    .outputFormat(extension)
    .stream(fs.createWriteStream(outputLocation));
}

let playlistReference = undefined;
async function extractPlaylistFromURL(
  location,
  url,
  name,
  extension,
  playlistFinish
) {
  let regex = /[\\\/:*?<>|]/gm;
  let cutTitle = name.replace(regex, "_");
  const outputLocation = `${location}/${cutTitle}`;
  if (!fs.existsSync(outputLocation)) {
    fs.mkdirSync(outputLocation);
  }
  webContents.send("playlist-progress", 0);
  const playlist = (await ytpl(url)).items;
  async function runList(depth) {
    const video = playlist[depth];
    try {
      await new Promise((resolve, reject) => {
        webContents.send(
          "playlist-progress",
          Math.floor((depth / playlist.length) * 100)
        );
        playlistReference = {
          destroy: () => {
            if (videoReference !== undefined) {
              videoReference.destroy();
              videoReference = undefined;
            }
            reject();
          },
        };
        extractVideoFromURL(
          outputLocation,
          video.url,
          video.title,
          "playlist-video-progress",
          extension,
          resolve
        );
      });
      if (depth < playlist.length - 1) {
        runList(depth + 1);
      } else {
        webContents.send("playlist-progress", 100);
        playlistReference = undefined;
        playlistFinish();
      }
    } catch (error) {}
  }
  runList(0);
}

// * Async Guards (aka invocations)
async function validVideoUrl(context, { data }) {
  const info = await fetchVideoInfo(data);
  if (info.valid) {
    return info;
  }
  throw info;
}

async function validPlaylistUrl(context, { data }) {
  const info = await fetchPlaylistInfo(data);
  if (info.valid) {
    return info;
  }
  throw info;
}

// * State Machine
const StateManager = createMachine(
  {
    type: "parallel",
    id: "StateManager",
    predictableActionArguments: true,
    context: {
      videoDetails: undefined,
      playlistDetails: undefined,
    },
    states: {
      LocationSelector: {
        initial: "init",
        states: {
          enabled: {
            on: {
              ENTER_LOCATION: [
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
                      target: "validate_video_url",
                      cond: "validLocation",
                    },
                    {
                      target: "not_ready",
                      actions: raise("ENTER_LOCATION"),
                    },
                  ],
                },
              },
              validate_video_url: {
                invoke: {
                  src: validVideoUrl,
                  onDone: {
                    target: "ready",
                    actions: assign({
                      videoDetails: (context, event) => event.data.result,
                    }),
                  },
                  onError: {
                    target: "not_ready",
                    actions: assign({
                      videoDetails: (context, event) => event.data.result,
                    }),
                  },
                },
              },
              ready: {
                on: {
                  VIDEO_URL_CHANGE: {
                    target: "not_ready",
                    actions: [
                      assign({
                        videoDetails: (context, event) => undefined,
                      }),
                      raise("ENTER_LOCATION"),
                    ],
                  },
                  START_VIDEO_EXTRACT: [
                    {
                      target: [
                        "extracting",
                        "#StateManager.LocationSelector.disabled",
                        "#StateManager.MultiVideoFetcher.disabled",
                      ],
                      cond: "validLocation",
                      actions: "startVideoExtract",
                    },
                    {
                      target: "ready",
                      actions: raise("ENTER_LOCATION"),
                    },
                  ],
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
                    actions: "cancelVideoExtract",
                  },
                  FINISH_VIDEO_EXTRACT: {
                    target: [
                      "not_ready",
                      "#StateManager.LocationSelector.enabled",
                      "#StateManager.MultiVideoFetcher.enabled.hist",
                    ],
                    actions: [
                      assign({
                        videoDetails: (context, event) => undefined,
                      }),
                      () => {
                        webContents.send("clear-input", "VIDEO");
                      },
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
                      target: "validate_playlist_url",
                      cond: "validLocation",
                    },
                    {
                      target: "not_ready",
                      actions: raise("ENTER_LOCATION"),
                    },
                  ],
                },
              },
              validate_playlist_url: {
                invoke: {
                  src: validPlaylistUrl,
                  onDone: {
                    target: "ready",
                    actions: assign({
                      playlistDetails: (context, event) => event.data.result,
                    }),
                  },
                  onError: {
                    target: "not_ready",
                    actions: assign({
                      playlistDetails: (context, event) => event.data.result,
                    }),
                  },
                },
              },
              ready: {
                on: {
                  PLAYLIST_URL_CHANGE: {
                    target: "not_ready",
                    actions: [
                      assign({
                        playlistDetails: (context, event) => undefined,
                      }),
                      raise("ENTER_LOCATION"),
                    ],
                  },
                  START_PLAYLIST_EXTRACT: [
                    {
                      target: [
                        "extracting",
                        "#StateManager.LocationSelector.disabled",
                        "#StateManager.SingleVideoFetcher.disabled",
                      ],
                      cond: "validLocation",
                      actions: "startPlaylistExtract",
                    },
                    {
                      target: "ready",
                      actions: raise("ENTER_LOCATION"),
                    },
                  ],
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
                    actions: "cancelPlaylistExtract",
                  },
                  FINISH_PLAYLIST_EXTRACT: {
                    target: [
                      "not_ready",
                      "#StateManager.LocationSelector.enabled",
                      "#StateManager.SingleVideoFetcher.enabled.hist",
                    ],
                    actions: [
                      assign({
                        playlistDetails: (context, event) => undefined,
                      }),
                      () => {
                        webContents.send("clear-input", "PLAYLIST");
                      },
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
        return fs.existsSync(currLocation);
      },
    },
    actions: {
      startVideoExtract: function (context) {
        extractVideoFromURL(
          currLocation,
          context.videoDetails.fetchedUrl,
          context.videoDetails.title,
          "video-progress",
          currExportSettings.video.ext,
          function () {
            StateService.send("FINISH_VIDEO_EXTRACT");
          }
        );
      },
      cancelVideoExtract: function () {
        if (videoReference !== undefined) {
          videoReference.destroy();
          videoReference = undefined;
        }
      },
      startPlaylistExtract: function (context) {
        extractPlaylistFromURL(
          currLocation,
          context.playlistDetails.fetchedUrl,
          context.playlistDetails.title,
          currExportSettings.playlist.ext,
          function () {
            StateService.send("FINISH_PLAYLIST_EXTRACT");
          }
        );
      },
      cancelPlaylistExtract: function () {
        if (playlistReference !== undefined) {
          playlistReference.destroy();
          playlistReference = undefined;
        }
      },
    },
  }
);

const StateService = interpret(StateManager);

app.on("ready", createWindow);

app.on("window-all-closed", () => {
  app.quit();
});
