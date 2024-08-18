import { Deferred } from "../../modules/deferred.js";
import { html, css } from "../util.js";

export default class VideoListElement extends HTMLElement {
  static #shadowTemplate = html`
    <template>
      <div class="v-flex">
        <div id="video-grid">
          <slot></slot>
        </div>
      </div>
    </template>
  `;

  static #shadowStyle = css`
    #video-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
      gap: 20px;
    }

    .h-flex {
      display: flex;
      flex-direction: row;
      justify-content: center;
      align-items: center;
    }
    .v-flex {
      display: column;
      flex-direction: vertical;
      justify-content: center;
      align-items: center;
    }
  `;

  #shadowRoot;
  #videoSlot;
  /** @type {import('./y-video.js').default[]} */
  #videoElements;

  /**
   * @typedef {{title: string, index: number, author: { name: string }, thumb: string, duration: string}} VideoInfo
   */
  /** @type {VideoInfo[]} */
  #videos;

  /** @type {number[]} */
  #loadQueue;
  #loading;
  #interrupt;

  #observer;

  constructor() {
    super();

    this.#shadowRoot = this.attachShadow({ mode: "closed", slotAssignment: "manual" });
    this.#shadowRoot.adoptedStyleSheets = [VideoListElement.#shadowStyle];
    this.#shadowRoot.append(
      document.importNode(VideoListElement.#shadowTemplate.content, true)
    );

    this.#videoSlot = this.#shadowRoot.querySelector("slot");
    this.#videoElements = [];

    this.#observer = new IntersectionObserver(this.#intersect.bind(this), {
      threshold: 0.5,
    });

    this.#loadQueue = [];
    this.#loading = false;
    this.#interrupt = false;
  }

  /**
   * @param {number} videoCount
   * @param {VideoInfo[]} videos
   */
  initVideos(videoCount, videos) {
    this.#videos = videos;
    this.#loadQueue.length = 0;
    this.#interruptLoading();

    this.#observer.disconnect();

    this.#videoElements.forEach((e) => {
      this.removeChild(e);
    });

    this.#videoElements = Array.from({ length: videoCount }, (_, i) => {
      /** @type {import('./y-video.js').default} */
      const element = document.createElement("y-video");
      element.index = i;
      this.appendChild(element);
      this.#observer.observe(element);
      return element;
    });

    this.#videoSlot.assign(...this.#videoElements);
  }

  /** @type {IntersectionObserverCallback} */
  #intersect(entries, observer) {
    entries.forEach(({ isIntersecting, target: { index }, target }) => {
      if (isIntersecting) {
        observer.unobserve(target);
        this.#startLoading(index);
      }
    });
  }

  #startLoading(index) {
    this.#loadQueue.push(index);

    if (!this.#loading) {
      this.#loading = true;
      this.#load();
    }
  }

  async #load() {
    /** @type {Deferred<void, void>} */
    const deferred = new Deferred();

    while (this.#loadQueue.length > 0 && !this.#interrupt) {
      /** @type {number} */
      const index = this.#loadQueue.shift();
      const video = this.#videos[index];

      deferred.restart();

      const img = new Image();
      img.onload = () => deferred.resolve();
      img.src = video.thumb;

      await deferred.promise;

      if (!this.#interrupt) {
        this.#display(index, video, img);
      } else {
      }
    }
    this.#interrupt = false;
    this.#loading = false;
  }

  /**
   * @param {number} index
   * @param {VideoInfo} video
   * @param {HTMLImageElement} image
   */
  #display(index, video, image) {
    const videoElement = this.#videoElements[index];
    videoElement.load(video, image);
  }

  #interruptLoading() {
    if (this.#loading) {
      this.#interrupt = true;
    }
  }
}

customElements.define("y-video-list", VideoListElement);
