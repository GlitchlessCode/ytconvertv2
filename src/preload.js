const { ipcRenderer, contextBridge } = require("electron");

contextBridge.exposeInMainWorld("api", {
  sendDarkMode: function (darkMode) {
    if (typeof darkMode !== "boolean")
      throw new TypeError("Cannot invoke function with invalid parameters");
    ipcRenderer.send("xel-dark-mode", darkMode);
  },
  sendEvent: function (event, url) {
    if (typeof event !== "string")
      throw new TypeError("Cannot invoke function with invalid parameters");
    const matchingSet = new Set([
      "FETCH_VIDEO_URL",
      "VIDEO_URL_CHANGE",
      "START_VIDEO_EXTRACT",
      "CANCEL_VIDEO_EXTRACT",
      "FETCH_PLAYLIST_URL",
      "PLAYLIST_URL_CHANGE",
      "START_PLAYLIST_EXTRACT",
      "CANCEL_PLAYLIST_EXTRACT",
    ]);
    const size = matchingSet.size;
    matchingSet.add(event);
    if (matchingSet.size !== size)
      throw new TypeError("Cannot invoke function with invalid parameters");

    if (event == "FETCH_VIDEO_URL" || event == "FETCH_PLAYLIST_URL") {
      const regex =
        /^(https?:\/\/)?(www\.)[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&\/\/=]*)$/;
      if (!regex.test(url)) throw new Error("url is not a URL");
      ipcRenderer.send("xstate-event", `${event}`, `${url}`);
    } else {
      ipcRenderer.send("xstate-event", `${event}`);
    }
  },
  sendSelectorState: function (exportType, fileType, fileExtension) {
    if (
      typeof exportType !== "string" ||
      !(exportType === "video" || exportType === "playlist")
    )
      throw new TypeError("Cannot invoke function with invalid parameters");
    if (
      typeof fileType !== "string" ||
      !((fileType === "audio") /*|| fileType === "video"*/)
    )
      throw new TypeError("Cannot invoke function with invalid parameters");
    if (fileType === "audio") {
      if (
        typeof fileExtension !== "string" ||
        !(
          fileExtension === "wav" ||
          fileExtension === "mp3" ||
          fileExtension === "ogg"
        )
      )
        throw new TypeError("Cannot invoke function with invalid parameters");
    } else {
      if (
        typeof fileExtension !== "string" ||
        !(
          fileExtension === "mp4" ||
          fileExtension === "mov" ||
          fileExtension === "mkv"
        )
      )
        throw new TypeError("Cannot invoke function with invalid parameters");
    }
    ipcRenderer.send(
      "selector-change",
      `${exportType}`,
      `${fileType}`,
      `${fileExtension}`
    );
  },
  recieveDarkMode: function (func) {
    ipcRenderer.on("xel-dark-mode", (event, ...args) => func(event, ...args));
  },
  recieveCurrentVersion: function (func) {
    ipcRenderer.on("current-version", (event, ...args) => func(event, ...args));
  },
  recieveReleaseNotice: function (func) {
    ipcRenderer.on("version-outdated", (event, ...args) =>
      func(event, ...args)
    );
  },
  recieveState: function (func) {
    ipcRenderer.on("xstate-transitioned", (event, ...args) =>
      func(event, ...args)
    );
  },
  recieveConnection: function (func) {
    ipcRenderer.on("connection-status", (event, ...args) => {
      func(event, ...args);
    });
  },
  recieveLocation: function (func) {
    ipcRenderer.on("location", (event, ...args) => {
      func(event, ...args);
    });
  },
  recieveInputClear(func) {
    ipcRenderer.on("clear-input", (event, ...args) => {
      func(event, ...args);
    });
  },
  recieveVideoProgress(func) {
    ipcRenderer.on("video-progress", (event, ...args) => {
      func(event, ...args);
    });
  },
  recievePlaylistVideoProgress(func) {
    ipcRenderer.on("playlist-video-progress", (event, ...args) => {
      func(event, ...args);
    });
  },
  recievePlaylistProgress(func) {
    ipcRenderer.on("playlist-progress", (event, ...args) => {
      func(event, ...args);
    });
  },
  requestWindowClose: function () {
    ipcRenderer.send("window-close-request");
  },
  requestOpenRelease: function () {
    ipcRenderer.send("open-release-request");
  },
  requestOpenWiki: function () {
    ipcRenderer.send("open-wiki-request");
  },
  requestOpenIssues: function () {
    ipcRenderer.send("open-issues-request");
  },
  requestLocationSelect: function () {
    ipcRenderer.send("location-select-request");
  },
});
