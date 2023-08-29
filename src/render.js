// * Initialization
// Navbar
const navbar = document.querySelector("x-tabs");
const navbarSingleImg = {
  spinner: document.querySelector('x-tab[value="single"] x-throbber'),
  icon: document.querySelector('x-tab[value="single"] x-icon'),
  showSpinner: function (bool) {
    this.spinner.hidden = !bool;
    this.icon.hidden = bool;
  },
};
Object.freeze(navbarSingleImg);
const navbarMultiImg = {
  spinner: document.querySelector('x-tab[value="multi"] x-throbber'),
  icon: document.querySelector('x-tab[value="multi"] x-icon'),
  showSpinner: function (bool) {
    this.spinner.hidden = !bool;
    this.icon.hidden = bool;
  },
};
Object.freeze(navbarMultiImg);

// Content boxes
const selectorContent = document.querySelector("#CONTENTselector");
const singleVideoContent = document.querySelector("#CONTENTsingle");
const multiVideoContent = document.querySelector("#CONTENTmulti");

// Selector Content
const locationIn = document.querySelector("#location");
const locationInBtn = document.querySelector("#location x-button");

// Single Content
// Ex: singleVideoCard.setAttribute("fileLock", "");
const singleVideoCard = document.querySelector("#CARDsingle");
const singleVideoInput = document.querySelector("#CARDsingle .link");
const singleVideoInfo = document.querySelector("#CARDsingle .info");
const singleVideoAutoFetch = document.querySelector("#CARDsingle .autofetch");
const singleVideoFileType = document.querySelector("#CARDsingle .filetype");
const singleVideoButtons = document.querySelector("#CARDsingle .buttons");
const singleVideoFetchBtn = document.querySelector(
  '#CARDsingle x-button[value="fetch"]'
);
const singleVideoExtractBtn = document.querySelector(
  '#CARDsingle x-button[value="export"]'
);

const singleVideoCancelBtn = document.querySelector(
  '#CARDsingle x-button[value="cancel"]'
);

// Multi Content
const multiVideoCard = document.querySelector("#CARDmulti");
const multiVideoInput = document.querySelector("#CARDmulti .link");
const multiVideoInfo = document.querySelector("#CARDmulti .info");
const multiVideoAutoFetch = document.querySelector("#CARDmulti .autofetch");
const multiVideoFileType = document.querySelector("#CARDmulti .filetype");
const multiVideoButtons = document.querySelector("#CARDmulti .buttons");
const multiVideoFetchBtn = document.querySelector(
  '#CARDmulti x-button[value="fetch"]'
);
const multiVideoExtractBtn = document.querySelector(
  '#CARDmulti x-button[value="export"]'
);

const multiVideoCancelBtn = document.querySelector(
  '#CARDmulti x-button[value="cancel"]'
);

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

api.recieveState(interpretState);

// Event Listeners
lightThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, false));
darkThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, true));

singleVideoFileType.addEventListener("toggle", resetFileType);
multiVideoFileType.addEventListener("toggle", resetFileType);

// Anonymous Event Listeners
navbar.addEventListener("change", function () {
  switch (navbar.value) {
    case "location":
      setDisplay(selectorContent);
      break;
    case "single":
      setDisplay(singleVideoContent);
      break;
    case "multi":
      setDisplay(multiVideoContent);
      break;
  }
});

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

function interpretState(event, newState) {
  locationIn.disabled = !newState.LocationSection;
  newState.SingleSection
    ? singleVideoCard.removeAttribute("disabled")
    : singleVideoCard.setAttribute("disabled", "");
  newState.MultiSection
    ? multiVideoCard.removeAttribute("disabled")
    : multiVideoCard.setAttribute("disabled", "");
  newState.FileLocationValid
    ? (singleVideoCard.removeAttribute("fileLock"),
      multiVideoCard.removeAttribute("fileLock"))
    : (singleVideoCard.setAttribute("fileLock", ""),
      multiVideoCard.setAttribute("fileLock", ""));

  singleVideoFetchBtn.disabled = !newState.SingleFetchBtn;
  singleVideoInput.disabled = !newState.SingleFetchBtn;
  singleVideoAutoFetch.disabled = !newState.SingleFetchBtn;

  singleVideoExtractBtn.disabled = !newState.SingleExtractBtn;
  singleVideoFileType.children[0].disabled = !newState.SingleExtractBtn;
  singleVideoFileType.children[1].disabled = !newState.SingleExtractBtn;

  singleVideoCancelBtn.disabled = !newState.SingleCancelBtn;
  navbarSingleImg.showSpinner(newState.SingleCancelBtn);

  multiVideoFetchBtn.disabled = !newState.MultiFetchBtn;
  multiVideoInput.disabled = !newState.MultiFetchBtn;
  multiVideoAutoFetch.disabled = !newState.MultiFetchBtn;

  multiVideoExtractBtn.disabled = !newState.MultiExtractBtn;
  multiVideoFileType.children[0].disabled = !newState.MultiExtractBtn;
  multiVideoFileType.children[1].disabled = !newState.MultiExtractBtn;

  multiVideoCancelBtn.disabled = !newState.MultiCancelBtn;
  navbarMultiImg.showSpinner(newState.MultiCancelBtn);
}

function setDisplay(element) {
  selectorContent.classList.remove("active");
  singleVideoContent.classList.remove("active");
  multiVideoContent.classList.remove("active");
  element.classList.add("active");
}

// ! TEMPORARY !
function resetFileType(event) {
  if (this.value !== "mp3") {
    this.value = "mp3";
  }
}
