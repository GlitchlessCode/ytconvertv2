import Xel from "../node_modules/xel/xel.js";
import "./components/registry.js";

// * Initialization
// MenuBar
const currVersionEl = document.querySelector("#currVersion");

const thumnailsToggle = document.querySelector("#thumbnails");
let displayThumbnails = true;
const lightThemeBtn = document.querySelector("#lightTheme");
const darkThemeBtn = document.querySelector("#darkTheme");
const helpBtn = document.querySelector("#help");
const bugBtn = document.querySelector("#bug");
const closeButton = document.querySelector("#exit");

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
const locationOut = document.querySelector("#location");
const locationInBtn = document.querySelector("#location x-button");

// Single Content
// Ex: singleVideoCard.setAttribute("fileLock", "");
const singleVideoCard = document.querySelector("#CARDsingle");
const singleVideoInput = document.querySelector("#CARDsingle .link");
const singleVideoInfo = document.querySelector("#CARDsingle .info");
const singleVideoAutoFetch = document.querySelector("#CARDsingle .autofetch");
const singleVideoFileType = document.querySelector("#CARDsingle .filetype");
const singleVideoFileSelect = document.querySelector("#CARDsingle .fileselect");
const singleVideoProgressBar = document.querySelector(
  "#CARDsingle .videoprogress x-progressbar"
);
const singleVideoProgressText = document.querySelector(
  "#CARDsingle .videoprogress x-label"
);
const singleVideoFetchBtn = document.querySelector('#CARDsingle x-button[value="fetch"]');
const singleVideoExtractBtn = document.querySelector(
  '#CARDsingle x-button[value="export"]'
);
const singleVideoCancelBtn = document.querySelector(
  '#CARDsingle x-button[value="cancel"]'
);
registerToolbar(
  "VIDEO",
  singleVideoFetchBtn,
  singleVideoExtractBtn,
  singleVideoCancelBtn,
  singleVideoAutoFetch,
  singleVideoInput
);
const singleVideoFileStats = document.querySelector("#CARDsingle .filestats");

// Multi Content
const multiVideoCard = document.querySelector("#CARDmulti");
const multiVideoInput = document.querySelector("#CARDmulti .link");
const multiVideoInfo = document.querySelector("#CARDmulti .info");
const multiVideoAutoFetch = document.querySelector("#CARDmulti .autofetch");
const multiVideoFileType = document.querySelector("#CARDmulti .filetype");
const multiVideoFileSelect = document.querySelector("#CARDmulti .fileselect");
const multiVideoTopProgressBar = document.querySelector(
  "#CARDmulti .videoprogress x-progressbar"
);
const multiVideoTopProgressText = document.querySelector(
  "#CARDmulti .videoprogress x-label"
);
const multiVideoBottomProgressBar = document.querySelector(
  "#CARDmulti .playlistprogress x-progressbar"
);
const multiVideoBottomProgressText = document.querySelector(
  "#CARDmulti .playlistprogress x-label"
);
const multiVideoFetchBtn = document.querySelector('#CARDmulti x-button[value="fetch"]');
const multiVideoExtractBtn = document.querySelector(
  '#CARDmulti x-button[value="export"]'
);

const multiVideoCancelBtn = document.querySelector('#CARDmulti x-button[value="cancel"]');
registerToolbar(
  "PLAYLIST",
  multiVideoFetchBtn,
  multiVideoExtractBtn,
  multiVideoCancelBtn,
  multiVideoAutoFetch,
  multiVideoInput
);
const multiVideoFileStats = document.querySelector("#CARDmulti .filestats");

const multiVideoListAccordion = document.querySelector("#CONTENTmulti > x-accordion");
/** @type {import('./components/components/y-video-list.js').default} */
const multiVideoList = document.querySelector("#CONTENTmulti y-video-list");

// Version Notification
/** @type {HTMLDialogElement} */
const versionDialog = document.querySelector("#versionNotif");
/** @type {HTMLParagraphElement} */
const versionText = document.querySelector("#versionOutput");
const versionIgnoreBtn = document.querySelector("#versionIgnore");
const versionDownloadBtn = document.querySelector("#versionDownload");

// File Stat Table
const fileStats = {
  wav: {
    speed: "#00ff00",
    qual: "#00ff00",
    size: "#ff0000",
    comp: "#00ff00",
  },
  ogg: {
    speed: "#ffff00",
    qual: "##aaff00",
    size: "#00ff00",
    comp: "#ff9900",
  },
  mp3: {
    speed: "#ffcc00",
    qual: "#ffcc00",
    size: "#c3ff00",
    comp: "#00ff00",
  },
  mp4: {
    speed: "#ffcc00",
    qual: "#00ff00",
    size: "#ffff00",
    comp: "#00ff00",
  },
  mov: {
    speed: "#ffff00",
    qual: "#00ff00",
    size: "#ffff00",
    comp: "#ffff00",
  },
  mkv: {
    speed: "#00ff00",
    qual: "#00ff00",
    size: "#00ff00",
    comp: "#ff9900",
  },
};

// Variables
let videoDetailHash = new Uint8Array();
let playlistDetailHash = new Uint8Array();

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

api.recieveConnection(function (event, online) {
  const connectionSwatch = document.querySelector("#connectionStatus x-swatch");
  const connectionText = document.querySelector("#connectionStatus x-label");
  if (!online) {
    const notif = document.querySelector("#notification");
    notif.innerHTML = "No Internet Connection";
    notif.opened = true;

    connectionSwatch.value = "#ff0000";
    connectionText.innerHTML = "Not Connected";
  } else {
    connectionSwatch.value = "#00ff00";
    connectionText.innerHTML = "Connected";
  }
});

api.recieveState(interpretState);

api.recieveLocation(function (event, location) {
  locationOut.removeAttribute("readonly");
  locationOut.value = location;
  locationOut.setAttribute("readonly", "");
});

api.recieveVideoProgress(function (event, value) {
  singleVideoProgressBar.value = value;
  singleVideoProgressText.innerHTML = Math.round(value) + "%";
});

api.recievePlaylistVideoProgress(function (event, value) {
  multiVideoTopProgressBar.value = value;
  multiVideoTopProgressText.innerHTML = Math.round(value) + "%";
});

api.recievePlaylistProgress(function (event, value) {
  multiVideoBottomProgressBar.value = value;
  multiVideoBottomProgressText.innerHTML = Math.round(value) + "%";
});

// Event Listeners
lightThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, false));
darkThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, true));
helpBtn.addEventListener("click", api.requestOpenWiki);
bugBtn.addEventListener("click", api.requestOpenIssues);
closeButton.addEventListener("click", api.requestWindowClose);
singleVideoFileType.addEventListener(
  "toggle",
  setupSelectBox.bind(
    singleVideoFileType,
    "video",
    singleVideoFileSelect,
    singleVideoFileStats
  )
);
multiVideoFileType.addEventListener(
  "toggle",
  setupSelectBox.bind(
    multiVideoFileType,
    "playlist",
    multiVideoFileSelect,
    multiVideoFileStats
  )
);
singleVideoFileSelect.addEventListener("change", (event) => {
  selectorChange(event, "video", singleVideoFileType.value, singleVideoFileStats);
});
multiVideoFileSelect.addEventListener("change", (event) => {
  selectorChange(event, "playlist", multiVideoFileType.value, multiVideoFileStats);
});
multiVideoList.addEventListener("toggle-include", setVideoInclusion);

// Anonymous Event Listeners
thumnailsToggle.addEventListener("toggle", function () {
  displayThumbnails = !displayThumbnails;
});

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

locationInBtn.addEventListener("click", api.requestLocationSelect);

window.onload = async () => {
  selectorContent.classList.add("active");
  await Xel.whenThemeReady;
  document.body.hidden = false;
  const cover = document.querySelector("#cover");
  const img = document.querySelector("#cover img");
  await ((ms) => new Promise((r, _) => setTimeout(r, ms)))(150);
  img.setAttribute("startup", "");
  await ((ms) => new Promise((r, _) => setTimeout(r, ms)))(1000);
  img.removeAttribute("startup");
  cover.removeAttribute("startup");
};

// * Functions
/**
 * @param {Uint8Array} buf1
 * @param {Uint8Array} buf2
 * @returns
 */
function testBuffers(buf1, buf2) {
  if (buf1.byteLength != buf2.byteLength) return false;
  for (let i = 0; i < buf1.byteLength; i++) {
    if (buf1[i] != buf2[i]) return false;
  }
  return true;
}

function setColorMode(mode) {
  Xel.theme = `../node_modules/xel/themes/adwaita${mode ? "-dark" : ""}.css`;
  lightThemeBtn.toggled = !mode;
  darkThemeBtn.toggled = mode;
  api.sendDarkMode(mode);
}

function fetch(type, url) {
  const notif = document.querySelector("#notification");
  try {
    const regex =
      /^(https?:\/\/)?(www\.)[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&\/\/=]*)$/;
    if (regex.test(url)) {
      api.sendEvent(`FETCH_${type}_URL`, url);
    } else {
      notif.innerHTML = "Please enter a valid URL";
      notif.opened = true;
    }
  } catch (error) {
    notif.innerHTML = "An Error Occurred";
    notif.opened = true;
  }
}

async function interpretState(event, newState, context) {
  locationOut.disabled = !newState.LocationSection;
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
  singleVideoFileSelect.disabled = !newState.SingleExtractBtn;

  singleVideoCancelBtn.disabled = !newState.SingleCancelBtn;
  navbarSingleImg.showSpinner(newState.SingleCancelBtn);

  multiVideoFetchBtn.disabled = !newState.MultiFetchBtn;
  multiVideoInput.disabled = !newState.MultiFetchBtn;
  multiVideoAutoFetch.disabled = !newState.MultiFetchBtn;

  multiVideoExtractBtn.disabled = !newState.MultiExtractBtn;
  multiVideoFileType.children[0].disabled = !newState.MultiExtractBtn;
  multiVideoFileType.children[1].disabled = !newState.MultiExtractBtn;
  multiVideoFileSelect.disabled = !newState.MultiExtractBtn;

  multiVideoCancelBtn.disabled = !newState.MultiCancelBtn;
  navbarMultiImg.showSpinner(newState.MultiCancelBtn);

  const videoDigest = new Uint8Array(
    await crypto.subtle.digest(
      "sha-256",
      new TextEncoder().encode(JSON.stringify(context.videoDetails))
    )
  );
  if (!testBuffers(videoDetailHash, videoDigest)) {
    videoDetailHash = videoDigest;
    const main = singleVideoInfo.querySelector("main");
    main.innerHTML = "";
    if (context.videoDetails !== undefined) {
      main.appendChild(createInfoCard(context.videoDetails));
      singleVideoInfo.expand();
    } else {
      singleVideoInfo.collapse();
    }
  }

  const playlistDigest = new Uint8Array(
    await crypto.subtle.digest(
      "sha-256",
      new TextEncoder().encode(JSON.stringify(context.playlistDetails))
    )
  );
  if (!testBuffers(playlistDetailHash, playlistDigest)) {
    playlistDetailHash = playlistDigest;
    const main = multiVideoInfo.querySelector("main");
    main.innerHTML = "";
    const { playlistDetails } = context;

    if (playlistDetails !== undefined) {
      main.appendChild(createInfoCard(playlistDetails));
      multiVideoList.initVideos(playlistDetails.items.length, playlistDetails.items);
      multiVideoListAccordion.expand();
      multiVideoInfo.expand();
    } else {
      multiVideoInfo.collapse();
      multiVideoListAccordion.collapse();
      multiVideoList.initVideos(0, []);
    }
  }
}

function setDisplay(element) {
  selectorContent.classList.remove("active");
  singleVideoContent.classList.remove("active");
  multiVideoContent.classList.remove("active");
  element.classList.add("active");
}

/**
 *
 * @param {{thumb: string|undefined, title: string, author: {avatar: string, name: string}|undefined, adjustedValue: string|undefined, date: string|undefined, adjustedValueTitle: string|undefined}} info
 */
function createInfoCard(info) {
  const parser = new DOMParser();
  return parser.parseFromString(
    `
  <x-card class="infoCard">
    ${
      info.thumb == undefined || displayThumbnails == false
        ? ``
        : `<img src="${info.thumb}"type="thumbnail"/>`
    }
    <main>
      <x-box vertical>
        <h2><strong>${info.title}</strong></h2>
        ${info.date == undefined ? `` : `<h4>${info.date}</h4>`}
        ${
          info.author == undefined
            ? ``
            : `
            <x-box>
              <img src="${info.author.avatar.url}" type="avatar"/>
              <x-label><strong>${info.author.name}</strong></x-label>
            </x-box>`
        }
        ${
          info.adjustedValue == undefined
            ? ``
            : `<p>${info.adjustedValueTitle}: ${info.adjustedValue}</p>`
        }
      </x-box>
    </main>
  </x-card>
`,
    "text/html"
  ).body.firstChild;
}

/**
 *
 * @param {"VIDEO"|"PLAYLIST"} type
 * @param {HTMLElement} fetchBtn
 * @param {HTMLElement} exportBtn
 * @param {HTMLElement} cancelBtn
 * @param {HTMLElement} autofetch
 * @param {HTMLElement} input
 */
function registerToolbar(type, fetchBtn, exportBtn, cancelBtn, autofetch, input) {
  fetchBtn.addEventListener("click", function () {
    fetch(type, input.value);
  });
  exportBtn.addEventListener("click", function () {
    api.sendEvent(`START_${type}_EXTRACT`);
  });
  cancelBtn.addEventListener("click", function () {
    api.sendEvent(`CANCEL_${type}_EXTRACT`);
  });

  let timeout;
  input.addEventListener("input", () => {
    api.sendEvent(`${type}_URL_CHANGE`);
    if (autofetch.toggled) {
      if (timeout !== undefined) clearTimeout(timeout);
      timeout = setTimeout(fetch.bind(fetch, type, input.value), 400);
    }
  });

  api.recieveInputClear(function (event, data) {
    if (data == type) {
      input.value = "";
    }
  });
}

/**
 * @param {string} exportType
 * @param {HTMLElement} selector
 * @param {HTMLElement} filestatsEl
 */
function setupSelectBox(exportType, selector, filestatsEl) {
  const items = selector.querySelectorAll("x-menuitem");
  const labels = selector.querySelectorAll("x-label");
  if (this.value === "video") {
    items[0].setAttribute("value", "mp4");
    items[1].setAttribute("value", "mov");
    items[2].setAttribute("value", "mkv");
    labels[0].innerHTML = ".mp4";
    labels[1].innerHTML = ".mov";
    labels[2].innerHTML = ".mkv";
    selector.value = "mp4";

    const colours = filestatsEl.querySelectorAll("x-swatch");
    colours[0].value = fileStats[selector.value].speed;
    colours[1].value = fileStats[selector.value].qual;
    colours[2].value = fileStats[selector.value].size;
    colours[3].value = fileStats[selector.value].comp;
  } else {
    items[0].setAttribute("value", "wav");
    items[1].setAttribute("value", "ogg");
    items[2].setAttribute("value", "mp3");
    labels[0].innerHTML = ".wav";
    labels[1].innerHTML = ".ogg";
    labels[2].innerHTML = ".mp3";
    selector.value = "wav";

    const colours = filestatsEl.querySelectorAll("x-swatch");
    colours[0].value = fileStats[selector.value].speed;
    colours[1].value = fileStats[selector.value].qual;
    colours[2].value = fileStats[selector.value].size;
    colours[3].value = fileStats[selector.value].comp;
  }

  api.sendSelectorState(exportType, this.value, selector.value);
}

/**
 * @param {{detail: {include: boolean, index: number}}}
 */
function setVideoInclusion({ detail }) {
  api.sendVideoInclusion(detail.index, detail.include);
}

function selectorChange(event, exportType, fileType, filestatsEl) {
  const colours = filestatsEl.querySelectorAll("x-swatch");
  colours[0].value = fileStats[event.detail.newValue].speed;
  colours[1].value = fileStats[event.detail.newValue].qual;
  colours[2].value = fileStats[event.detail.newValue].size;
  colours[3].value = fileStats[event.detail.newValue].comp;
  api.sendSelectorState(exportType, fileType, event.detail.newValue);
}
