const { ipcRenderer, contextBridge } = require("electron");

contextBridge.exposeInMainWorld("api", {
  sendDarkMode: function (darkMode) {
    if (typeof darkMode !== "boolean")
      throw new TypeError("Cannot invoke function with invalid parameters");
    ipcRenderer.send("xel-dark-mode", darkMode);
  },
  sendEvent: function (event) {
    if (typeof event !== "string")
      throw new TypeError("Cannot invoke function with invalid parameters");
    const matchingSet = new Set([
      "ENTER_URL",
      "FETCH_VIDEO_URL",
      "VIDEO_URL_CHANGE",
      "START_VIDEO_EXTRACT",
      "CANCEL_VIDEO_EXTRACT",
      "FINISH_VIDEO_EXTRACT",
      "FETCH_PLAYLIST_URL",
      "PLAYLIST_URL_CHANGE",
      "START_PLAYLIST_EXTRACT",
      "CANCEL_PLAYLIST_EXTRACT",
      "FINISH_PLAYLIST_EXTRACT",
    ]);
    const size = matchingSet.size;
    matchingSet.add(event);
    if (matchingSet.size !== size)
      throw new TypeError("Cannot invoke function with invalid parameters");
    ipcRenderer.send("xstate-event", `${event}`);
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
  requestWindowClose: function () {
    ipcRenderer.send("window-close-request");
  },
  requestOpenRelease: function () {
    ipcRenderer.send("open-release-request");
  },
});
