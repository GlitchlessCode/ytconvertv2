const { ipcRenderer, contextBridge } = require("electron");

// Expose protected methods off of window (ie.
// window.api.sendToA) in order to use ipcRenderer
// without exposing the entire object
contextBridge.exposeInMainWorld("api", {
  sendToA: function () {
    ipcRenderer.send("A");
  },
  receiveFromD: function (func) {
    ipcRenderer.on("D", (event, ...args) => func(event, ...args));
  },
  requestWindowClose: function () {
    ipcRenderer.send("window-close-request");
  },
});
