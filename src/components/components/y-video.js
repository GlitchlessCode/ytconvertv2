import { html, css } from "../util.js";

export default class VideoDisplayElement extends HTMLElement {
  static #shadowTemplate = html`
    <template>
      <div id="card">
        <div class="loading"></div>
        <div>
          <div id="img-box">
            <slot id="img" name="img" />
          </div>
        </div>
        <div id="duration">
          <h3></h3>
        </div>
        <div id="cover">
        <h2>Click to <span id="toggle">disable</span><h2>
        </div>
      </div>
      <div id="details" class="loading">
        <h1></h1>
        <h2></h2>
      </div>
    </template>
  `;

  static #shadowStyle = css`
    :host {
      --skeleton-bg: color-mix(in srgb, var(--background-color), #7f7f7f 25%);
      --skeleton-fg: color-mix(in srgb, var(--skeleton-bg), #fff 40%);
      --skeleton-fg-transparent: color-mix(in srgb, var(--skeleton-fg) 0%, #ffffff00);
    }

    div#card {
      position: relative;
      aspect-ratio: 16/9;
      width: 100%;
      min-width: 160px;
      overflow: hidden;
      border-radius: 4.5% / 8%;
      transition-property: transform border;
      transition-duration: 0.5s;
      border: 3px solid transparent;
    }

    div#card:not(:has(+ .loading)):hover {
      transform: scale(1.025);
      border: 3px solid var(--accent-color);
    }

    div#card > div {
      position: absolute;
      width: 100%;
      height: 100%;
    }

    #img-box {
      width: 100%;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      overflow: hidden;
    }

    ::slotted(img) {
      flex-shrink: 0;
      min-width: 100%;
      min-height: 100%;

      animation: video-preview-img 0.5s backwards ease-in-out;
    }

    :host(.disabled) {
      filter: grayscale(90%) brightness(50%);
    }

    div#card .loading {
      transition-duration: 0.5s;
      transition-property: opacity;
      background-color: var(--skeleton-bg);
      width: 100%;
      height: 100%;
      background: linear-gradient(
          100deg,
          var(--skeleton-fg-transparent) 46%,
          var(--skeleton-fg) 49%,
          var(--skeleton-fg) 51%,
          var(--skeleton-fg-transparent) 54%
        )
        var(--skeleton-bg);
      background-size: 200% 100%;
      background-position-x: 120%;
      animation: 1.25s loading ease-in-out alternate infinite;
    }

    div#duration {
      position: relative;
    }

    div#cover {
      display: flex;
      background: rgba(0, 0, 0, 0.6);
      color: #fff;
      opacity: 0;
      transition-duration: 0.5s;
      transition-property: opacity;
      justify-content: center;
      align-items: center;
    }

    div#card:not(:has(+ .loading)):hover div#cover {
      cursor: pointer;
      opacity: 1;
    }

    div#cover h2 {
      cursor: pointer;
      -webkit-touch-callout: none; /* iOS Safari */
      -webkit-user-select: none; /* Safari */
      -khtml-user-select: none; /* Konqueror HTML */
      -moz-user-select: none; /* Old versions of Firefox */
      -ms-user-select: none; /* Internet Explorer/Edge */
      user-select: none;
    }

    div#duration h3 {
      position: absolute;
      right: 3%;
      bottom: 5%;
      font-size: 0.9em;
      background: rgba(0, 0, 0, 0.6);
      color: #fff;
      padding: 0 0.4em;
      margin: 0;
      border-radius: 0.3em;
      animation: fade 0.5s backwards ease-in-out;
    }

    div#details h1::before,
    div#details h2::before {
      content: "";
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      transition-duration: 0.5s;
      transition-property: opacity;
      opacity: 0;
      border-radius: 0.4em;
      pointer-events: none;
      background-color: var(--skeleton-bg);
      background: linear-gradient(
          100deg,
          var(--skeleton-fg-transparent) 46%,
          var(--skeleton-fg) 49%,
          var(--skeleton-fg) 51%,
          var(--skeleton-fg-transparent) 54%
        )
        var(--skeleton-bg);
      background-size: 200% 100%;
      background-position-x: 120%;
      animation: 1.25s loading ease-in-out alternate infinite;
      z-index: 2;
    }

    div#details.loading h1::before,
    div#details.loading h2::before {
      opacity: 1;
    }

    div#details h1,
    div#details h2 {
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
      position: relative;
      z-index: 1;
      min-height: 1.6em;
    }

    div#details h1 {
      font-size: 1.2em;
      margin: 0.3em 0;
      width: 90%;
    }

    div#details h2 {
      font-size: 0.9em;
      margin: 0.3em 0;
      width: 50%;
    }

    @keyframes loading {
      to {
        background-position-x: -20%;
      }
    }

    .hidden {
      opacity: 0;
    }
  `;

  #shadowRoot;

  /** @type {number} */
  #index;

  /** @type {HTMLSlotElement} */
  #imgSlot;
  /** @type {HTMLDivElement} */
  #detailDiv;
  /** @type {HTMLHeadingElement} */
  #titleElement;
  /** @type {HTMLHeadingElement} */
  #authorElement;
  /** @type {HTMLHeadingElement} */
  #durationElement;
  /** @type {HTMLSpanElement} */
  #toggleSpan;
  /** @type {HTMLDivElement} */
  #loadingElement;

  #toggled;
  #loaded;

  constructor() {
    super();

    this.#shadowRoot = this.attachShadow({ mode: "closed", slotAssignment: "manual" });
    this.#shadowRoot.adoptedStyleSheets = [VideoDisplayElement.#shadowStyle];
    this.#shadowRoot.append(
      document.importNode(VideoDisplayElement.#shadowTemplate.content, true)
    );

    this.#imgSlot = this.#shadowRoot.querySelector("slot#img");
    this.#detailDiv = this.#shadowRoot.querySelector("div#details");
    this.#titleElement = this.#shadowRoot.querySelector("div#details h1");
    this.#authorElement = this.#shadowRoot.querySelector("div#details h2");
    this.#durationElement = this.#shadowRoot.querySelector("div#duration h3");
    this.#toggleSpan = this.#shadowRoot.querySelector("div#cover span#toggle");
    this.#loadingElement = this.#shadowRoot.querySelector("div#card .loading");

    this.#shadowRoot
      .querySelector("div#cover")
      .addEventListener("click", this.#toggle.bind(this));
    this.#toggled = true;
    this.#loaded = false;
  }

  get index() {
    return this.#index;
  }
  set index(value) {
    if (this.#index == undefined) {
      this.#index = value;
    }
  }

  /**
   * @typedef {{title: string, index: number, author: { name: string }, thumb: string, duration: string}} VideoInfo
   */

  /**
   * @param {VideoInfo} video
   * @param {HTMLImageElement} image
   */
  load(video, image) {
    this.#loaded = true;
    this.#imgSlot.assign(image);
    this.append(image);

    this.#titleElement.innerText = video.title;
    this.#authorElement.innerText = video.author.name;
    this.#durationElement.innerText = video.duration;

    this.#detailDiv.classList.remove("loading");
    this.#loadingElement.classList.add("hidden");
  }

  #toggle() {
    if (this.#loaded) {
      this.#toggled = !this.#toggled;
      if (!this.#toggled) {
        this.classList.add("disabled");
        this.#toggleSpan.innerText = "enable";
      } else {
        this.classList.remove("disabled");
        this.#toggleSpan.innerText = "disable";
      }

      this.dispatchEvent(
        new CustomEvent("toggle-include", {
          bubbles: true,
          detail: { include: this.#toggled, index: this.index },
        })
      );
    }
  }
}

customElements.define("y-video", VideoDisplayElement);
