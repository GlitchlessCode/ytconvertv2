import Xel from "../node_modules/xel/xel.js";

// * Initialization
// MenuBar
const themeMeta = document.querySelector('meta[name="xel-theme"]');

const currVersionEl = document.querySelector("#currVersion");

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
const singleVideoProgressBar = document.querySelector(
  "#CARDsingle .videoprogress x-progressbar"
);
const singleVideoProgressText = document.querySelector(
  "#CARDsingle .videoprogress x-label"
);
const singleVideoFetchBtn = document.querySelector(
  '#CARDsingle x-button[value="fetch"]'
);
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

// Multi Content
const multiVideoCard = document.querySelector("#CARDmulti");
const multiVideoInput = document.querySelector("#CARDmulti .link");
const multiVideoInfo = document.querySelector("#CARDmulti .info");
const multiVideoAutoFetch = document.querySelector("#CARDmulti .autofetch");
const multiVideoFileType = document.querySelector("#CARDmulti .filetype");
const multiVideoFetchBtn = document.querySelector(
  '#CARDmulti x-button[value="fetch"]'
);
const multiVideoExtractBtn = document.querySelector(
  '#CARDmulti x-button[value="export"]'
);

const multiVideoCancelBtn = document.querySelector(
  '#CARDmulti x-button[value="cancel"]'
);
registerToolbar(
  "PLAYLIST",
  multiVideoFetchBtn,
  multiVideoExtractBtn,
  multiVideoCancelBtn,
  multiVideoAutoFetch,
  multiVideoInput
);

// Version Notification
/** @type {HTMLDialogElement} */
const versionDialog = document.querySelector("#versionNotif");
/** @type {HTMLParagraphElement} */
const versionText = document.querySelector("#versionOutput");
const versionIgnoreBtn = document.querySelector("#versionIgnore");
const versionDownloadBtn = document.querySelector("#versionDownload");

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
  locationOut.value = location;
});

api.recieveVideoProgress(function (event, value) {
  singleVideoProgressBar.value = Math.floor(value / 4) * 4;
  singleVideoProgressText.innerHTML = value + "%";
});

// Event Listeners
lightThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, false));
darkThemeBtn.addEventListener("click", setColorMode.bind(setColorMode, true));
helpBtn.addEventListener("click", api.requestOpenWiki);
bugBtn.addEventListener("click", api.requestOpenIssues);
closeButton.addEventListener("click", api.requestWindowClose);

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

locationInBtn.addEventListener("click", api.requestLocationSelect);

window.onload = async () => {
  selectorContent.classList.add("active");
  await Xel.whenThemeReady;
  document.body.hidden = false;
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
  themeMeta.content = `../node_modules/xel/themes/adwaita${
    mode ? "-dark" : ""
  }.css`;
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
    if (context.playlistDetails !== undefined) {
      main.appendChild(createInfoCard(context.playlistDetails));
      multiVideoInfo.expand();
    } else {
      multiVideoInfo.collapse();
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
      info.thumb == undefined
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
function registerToolbar(
  type,
  fetchBtn,
  exportBtn,
  cancelBtn,
  autofetch,
  input
) {
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

// ! TEMPORARY !
function resetFileType() {
  if (this.value !== "mp3") {
    this.value = "mp3";
  }
}
