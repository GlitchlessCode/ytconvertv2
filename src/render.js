// * Initialization
// Content boxes
const selectorContent = document.querySelector("#CONTENTselector");
const singleVideoContent = document.querySelector("#CONTENTsingle");
const multiVideoContent = document.querySelector("#CONTENTmulti");

// Version Notification
/** @type {HTMLDialogElement} */
const versionDialog = document.querySelector("#versionNotif");
/** @type {HTMLParagraphElement} */
const versionText = document.querySelector("#versionOutput");
const versionIgnoreBtn = document.querySelector("#versionIgnore");
const versionDownloadBtn = document.querySelector("#versionDownload");

// Color Theme
const themeMeta = document.querySelector('meta[name="xel-theme"]');
const lightThemeBtn = document.querySelector("#lightTheme");
const darkThemeBtn = document.querySelector("#darkTheme");

// Menu Items
const currVersionEl = document.querySelector("#currVersion");
const closeButton = document.querySelector("#exit");

// * Event Listeners
// IPC event listeners
api.recieveDarkMode(function (event, data) {
  setColorMode(data);
});

api.recieveCurrentVersion(function (event, currVersion) {
  currVersionEl.innerText = currVersion;
});

api.recieveReleaseNotice(function (event, thisVersion, ghVersion) {
  versionText.innerHTML = `This version (${thisVersion}) is out of date! Some things may not work normally, or the app may not even work at all! You can get the newest version (${ghVersion}) of this app by clicking the button below.`;
  versionDialog.showModal();
  const aborter = new AbortController();
  versionIgnoreBtn.addEventListener(
    "click",
    function () {
      aborter.abort();
      versionDialog.close();
    },
    {
      signal: aborter.signal,
    }
  );
  versionDownloadBtn.addEventListener(
    "click",
    function () {
      aborter.abort();
      api.requestOpenRelease();
    },
    {
      signal: aborter.signal,
    }
  );
});

// Event Listeners
lightThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, false));
darkThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, true));

// Anonymous Event Listeners
closeButton.addEventListener("click", function () {
  api.requestWindowClose();
});

window.onload = () => {
  selectorContent.classList.add("active");
};

// * Functions
function setColorMode(mode) {
  themeMeta.content = `../node_modules/xel/themes/adwaita${
    mode ? "-dark" : ""
  }.css`;
  lightThemeBtn.toggled = !mode;
  darkThemeBtn.toggled = mode;
  api.sendDarkMode(mode);
}
