const { ipcRenderer, contextBridge } = require("electron");

contextBridge.exposeInMainWorld("api", {
  sendDarkMode: function (darkMode) {
    if (typeof darkMode !== "boolean")
      throw new TypeError("Cannot invoke function with invalid parameters");
    ipcRenderer.send("xel-dark-mode", darkMode);
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
  requestWindowClose: function () {
    ipcRenderer.send("window-close-request");
  },
  requestOpenRelease: function () {
    ipcRenderer.send("open-release-request");
  },
});
