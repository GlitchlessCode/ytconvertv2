/**
 * @template T, E
 */
module.exports = class Deferred {
  get resolved() {
    return this.#resolved;
  }

  /** @type {Promise<T|E>} */
  promise;
  #resolve;
  #reject;
  #resolved;

  constructor() {
    this.#resolved = false;
    this.promise = new Promise((res, rej) => {
      this.#resolve = res;
      this.#reject = rej;
    });
  }

  /**
   * @param {T} value
   */
  resolve(value) {
    if (this.#resolved) return;
    this.#resolved = true;
    this.#resolve(value);
  }

  /**
   * @param {E} value
   */
  reject(value) {
    if (this.#resolved) return;
    this.#resolved = true;
    this.#reject(value);
  }

  restart() {
    this.#resolved = false;
    this.promise = new Promise((res, rej) => {
      this.#resolve = res;
      this.#reject = rej;
    });
  }
};
