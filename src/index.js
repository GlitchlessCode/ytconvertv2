// APP VERSION
const VERSION = "v1.3.0";

// Imports
const { app, BrowserWindow, ipcMain, screen, dialog, net, shell } = require("electron");
const path = require("path");
const ytdl = require("ytdl-core");
const ytpl = require("ytpl");
const Store = require("electron-store");
const { createMachine, interpret, assign } = require("xstate");
const fs = require("fs");
const { raise } = require("xstate/lib/actions");
const ffmpeg = require("fluent-ffmpeg");
/** @type {string} */
const theoreticalPath = require("ffmpeg-static");
const Deferred = require("./internal-modules/deferred");
const child_process = require("child_process");
const { PassThrough } = require("stream");
const fileName = path.join(
  process.resourcesPath,
  theoreticalPath.match(/[\\\/](?:.(?![\\\/]))+$/)[0]
);
const trueFFmpegPath = path.resolve(
  (() => {
    if (fs.existsSync(fileName)) {
      return fileName;
    } else {
      return theoreticalPath;
    }
  })()
);

ffmpeg.setFfmpegPath(trueFFmpegPath);

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

  mainWindow.setBackgroundColor(store.get("darkMode", true) ? "#242424" : "#f5f5f5");

  // and load the index.html of the app.
  await mainWindow.loadFile(path.join(__dirname, "index.html"));

  webContents.send("xel-dark-mode", store.get("darkMode", true));

  ipcMain.on("window-close-request", async function () {
    if (playlistReference !== undefined) {
      await playlistReference.destroy();
    }
    if (videoReference !== undefined) {
      await videoReference.destroy();
    }
    mainWindow.close();
  });
  ipcMain.on("xel-dark-mode", function (event, data) {
    store.set("darkMode", data);
  });

  ipcMain.on(
    "video-inclusion",
    /**
     * @param {number} index
     * @param {boolean} inclusion
     */
    function (event, index, inclusion) {
      StateService.send("VIDEO_INCLUSION_CHANGE", { data: { index, inclusion } });
    }
  );

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
     * @param {"wav"|"mp3"|"ogg"|"mp4"|"mov"|"mkv"} fileExtension
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
    const minutes = (Math.floor(totalSeconds / 60) % 60).toString().padStart(2, "0");
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
  /**
   * @type {{
   * fetchedUrl: string|undefined,
   * thumb: string,
   * title: string,
   * author: { avatar: string, name: string} | undefined,
   * adjustedValue: number | undefined,
   * adjustedValueTitle: "Videos",
   * items: {title: string, index: number, author: {name: string}, thumb: string, duration: string}[] | undefined,
   * included: Set<number>
   * }}
   */
  const result = {
    fetchedUrl: undefined,
    thumb: undefined,
    title: undefined,
    author: undefined,
    adjustedValue: undefined,
    adjustedValueTitle: "Videos",
    date: undefined,
    items: [],
    included: new Set(),
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
    result.items = info.items.map(
      ({ title, index, author: { name }, bestThumbnail: { url }, duration }) => ({
        title,
        index: index - 1,
        author: { name },
        thumb: url,
        duration,
      })
    );
    info.items.forEach((item) => result.included.add(item.index - 1));
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
/**
 * @param {string} location
 * @param {string} url
 * @param {string} name
 * @param {"video-progress"|"playlist-video-progress"} event
 * @param {string} extension
 * @param {"audio"|"video"} extractType
 */
function extractYTVideoFromURL(
  location,
  url,
  name,
  event,
  extension,
  extractType,
  videoFinish
) {
  if (extractType == "audio") {
    let regex = /[\\\/:*?<>|"'`]/gm;
    let cutTitle = name.replace(regex, "_");
    const outputLocation = `${location}/${cutTitle}.${extension}`;
    const video = ytdl(url, { filter: "audioonly", quality: "highestaudio" });
    webContents.send(event, 0);
    video.addListener("progress", function (_, current, total) {
      webContents.send(event, (current / total) * 100);
    });

    const command = ffmpeg().input(video).outputFormat(extension).save(outputLocation);

    videoReference = {
      destroy: async () => {
        video.destroy();

        const killDeferred = new Deferred();
        command.on("error", () => {
          killDeferred.resolve();
        });
        command.kill();
        await killDeferred.promise;
      },
    };
    video.addListener("end", function () {
      videoReference = undefined;
      videoFinish();
    });
  } else {
    extractVideo(location, url, name, event, extension, videoFinish);
  }
}

/**
 * @param {string} location
 * @param {string} url
 * @param {string} name
 * @param {"video-progress"|"playlist-video-progress"} event
 * @param {string} extension
 * @param {"audio"|"video"} extractType
 */
async function extractVideo(location, url, name, event, extension, videoFinish) {
  webContents.send(event, 0);

  /** @typedef {"init"|"video"|"audio"|"merge"|"recode"|"destroyed"} TestState */
  /** @type {{state: TestState,context:{[x:string]:any},update(newState: TestState, context: {[x:string]:any}), destroyed: readonly boolean}} */
  let state = {
    state: "init",
    context: {},
    update(newState, context) {
      this.state = newState;
      Object.assign(this.context, context);
    },
    get destroyed() {
      return this.state == "destroyed";
    },
  };

  let progressBar = new Proxy(
    {
      vidStrm: 0,
      audStrm: 0,
      recComm: 0,
    },
    {
      set(obj, prop, value) {
        const bool = Reflect.set(obj, prop, Math.max(0, Math.min(value, 1)));
        webContents.send(
          event,
          (Object.values(obj).reduce((prev, curr) => prev + curr, 0) / 3) * 100
        );
        return bool;
      },
    }
  );

  videoReference = {
    destroy: async () => {
      let killMerge = false;
      switch (state.state) {
        case "recode":
          const killDeferred = new Deferred();
          const command = state.context.recodeProcess;
          command.on("error", () => {
            killDeferred.resolve();
          });
          command.kill();
          await killDeferred.promise;
        case "merge":
          killMerge = true;
        case "audio":
          state.context.audioBuffer.destroy();
          state.context.audioStream.destroy();
        case "video":
          state.context.videoBuffer.destroy();
          state.context.videoStream.destroy();
        case "merge":
          if (killMerge) {
            const killDeferred = new Deferred();
            state.context.mergeProcess
              .on("close", () => {
                killDeferred.resolve();
              })
              .kill();
            await killDeferred.promise;
          }
          break;
      }

      cleanup();

      state.update("destroyed", {});
    },
  };

  /** @type {Deferred<{itag:number}[],{itag:number}[]>} */
  const fetchDeferred = new Deferred();

  let totalTime = -1;
  ytdl.getInfo(url, { filter: "videoonly" }).then((info) => {
    const { formats, videoDetails } = info;
    fetchDeferred.resolve(formats);
    totalTime = parseInt(videoDetails.lengthSeconds) * 1000;
  });

  const permittedFormats = [
    335, 303, 248, 334, 302, 247, 333, 246, 245, 244, 332, 243, 331, 242, 330, 219,
  ];
  const formats = new Set((await fetchDeferred.promise).map((v) => v.itag));

  const format = permittedFormats.find((f) => formats.has(f));

  if (!Number.isInteger(format)) throw new Error("No valid formats found");

  if (state.destroyed) return;

  /** @type {Deferred<void, void>} */
  const videoDeferred = new Deferred();
  const videoBuffer = new PassThrough({ highWaterMark: 1024 * 1024 * 512 });

  /** @type {import("stream").Readable} */
  const videoStream = ytdl(url, { quality: format })
    .on("progress", function (_, current, total) {
      progressBar["vidStrm"] = current / total;
    })
    .on("end", function () {
      videoDeferred.resolve();
    })
    .pipe(videoBuffer);
  state.update("video", { videoStream, videoBuffer });

  if (state.destroyed) return;

  const audioDeferred = new Deferred();
  const audioBuffer = new PassThrough({ highWaterMark: 1024 * 1024 * 512 });

  /** @type {import("stream").Readable} */
  const audioStream = ytdl(url, { filter: "audioonly", quality: "highestaudio" })
    .on("progress", function (_, current, total) {
      progressBar["audStrm"] = current / total;
    })
    .on("end", function () {
      audioDeferred.resolve();
    })
    .pipe(audioBuffer);
  state.update("audio", { audioStream, audioBuffer });

  const outputFormat = (() => {
    switch (extension) {
      case "mp4":
        return "mp4";
      case "mov":
        return "mov";
      case "mkv":
        return "matroska";
      default:
        throw new Error("Unknown extension");
    }
  })();

  const mergeDeferred = new Deferred();
  const mergeProcess = child_process
    .spawn(
      trueFFmpegPath,
      [
        // supress stderr
        "-loglevel",
        "8",
        "-hide_banner",

        // create pipes
        "-i",
        "pipe:3",
        "-i",
        "pipe:4",
        // set movflags if not mkv
        ...(extension !== "mkv" ? ["-movflags", "frag_keyframe"] : []),

        // map audio and video
        "-map",
        "0:a",
        "-map",
        "1:v",
        // set codecs
        "-vcodec",
        "libx264",
        "-acodec",
        "aac",

        // output
        "-f",
        outputFormat,
        "pipe:5",
      ],
      {
        windowsHide: true,
        stdio: [
          "inherit",
          "inherit",
          "inherit",
          // pipe audio, video, and output
          "pipe",
          "pipe",
          "pipe",
        ],
      }
    )
    .on("close", () => {
      mergeDeferred.resolve();
    });

  state.update("merge", { mergeProcess });

  audioBuffer.pipe(mergeProcess.stdio[3]);
  videoBuffer.pipe(mergeProcess.stdio[4]);

  let regex = /[\\\/:*?<>|"'`]/gm;
  let cutTitle = name.replace(regex, "_");
  const outputLocation = `${location}/${cutTitle}.${extension}`;

  const recodeDeferred = new Deferred();
  const recodeProcess = ffmpeg()
    .addInput(mergeProcess.stdio[5])
    .videoCodec("copy")
    .audioCodec("copy")
    .save(outputLocation)
    .on("end", () => recodeDeferred.resolve())
    .on("progress", (event) => {
      try {
        progressBar["recComm"] = convertTimestampToMillis(event?.timemark) / totalTime;
      } catch (error) {}
    });

  state.update("recode", { recodeProcess });

  await videoDeferred.promise;
  progressBar["vidStrm"] = 1;
  await audioDeferred.promise;
  progressBar["audStrm"] = 1;
  await recodeDeferred.promise;
  progressBar["recComm"] = 1;

  cleanup();

  function cleanup() {
    videoReference = undefined;
    videoFinish();
  }
}

/**
 * @param {string} timeString
 */
function convertTimestampToMillis(timeString) {
  const regex =
    /^(?<hour>[0-9]+):(?<minute>[0-9]{2}):(?<second>[0-9]{2})\.(?<milli>[0-9]+)$/;

  const match = regex.exec(timeString);
  if (match == null) throw new Error("Regex failed to match against " + timeString);

  const hour = parseInt(match.groups.hour);
  const minute = parseInt(match.groups.minute);
  const second = parseInt(match.groups.second);
  const milli = parseInt(match.groups.milli.padEnd(3, 0).slice(0, 3));

  return ((hour * 60 + minute) * 60 + second) * 1000 + milli;
}

let playlistReference = undefined;
/**
 * @param {string} location
 * @param {string} url
 * @param {string} name
 * @param {string} extension
 * @param {"audio"|"video"} extractType
 * @param {Set<number>} inclusionSet
 */
async function extractPlaylistFromURL(
  location,
  url,
  name,
  extension,
  extractType,
  inclusionSet,
  playlistFinish
) {
  let run = true;
  playlistReference = {
    destroy: async () => {
      run = false;
    },
  };
  let regex = /[\\\/:*?<>|"'`]/gm;
  let cutTitle = name.replace(regex, "_");
  const outputLocation = `${location}/${cutTitle}`;
  if (!fs.existsSync(outputLocation)) {
    fs.mkdirSync(outputLocation);
  }
  webContents.send("playlist-progress", 0);
  const playlist = (await ytpl(url)).items.filter((item) =>
    inclusionSet.has(item.index - 1)
  );
  async function runList(depth) {
    const video = playlist[depth];
    try {
      await new Promise((resolve, reject) => {
        if (!run) {
          if (videoReference !== undefined) {
            videoReference.destroy();
            videoReference = undefined;
          }
          reject();
        }
        webContents.send(
          "playlist-progress",
          Math.floor((depth / inclusionSet.size) * 100)
        );
        playlistReference = {
          destroy: async () => {
            if (videoReference !== undefined) {
              await videoReference.destroy();
              videoReference = undefined;
            }
            reject();
          },
        };
        extractYTVideoFromURL(
          outputLocation,
          video.url,
          video.title,
          "playlist-video-progress",
          extension,
          extractType,
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
                  VIDEO_INCLUSION_CHANGE: {
                    internal: true,
                    actions: assign({
                      playlistDetails: (context, event) => {
                        const set = context.playlistDetails.included;

                        if (event.data.inclusion) {
                          set.add(event.data.index);
                        } else {
                          set.delete(event.data.index);
                        }

                        return context.playlistDetails;
                      },
                    }),
                  },
                  START_PLAYLIST_EXTRACT: [
                    {
                      target: [
                        "extracting",
                        "#StateManager.LocationSelector.disabled",
                        "#StateManager.SingleVideoFetcher.disabled",
                      ],
                      cond: "validPlaylistExtractSettings",
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
      validPlaylistExtractSettings: function (context) {
        return fs.existsSync(currLocation) && context.playlistDetails.included.size > 0;
      },
    },
    actions: {
      startVideoExtract: function (context) {
        extractYTVideoFromURL(
          currLocation,
          context.videoDetails.fetchedUrl,
          context.videoDetails.title,
          "video-progress",
          currExportSettings.video.ext,
          currExportSettings.video.type,
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
          currExportSettings.playlist.type,
          context.playlistDetails.included,
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
