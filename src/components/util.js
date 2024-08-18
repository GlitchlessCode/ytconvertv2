/**
 * @param {string[]} strings
 * @param  {...any} expressions
 * @returns {DocumentFragment|HTMLElement}
 */
export function html(strings, ...expressions) {
  const parts = [];

  for (let i = 0; i < strings.length; i += 1) {
    parts.push(strings[i]);
    if (expressions[i] !== undefined) parts.push(expressions[i]);
  }

  const templateElement = document.createElement("template");
  templateElement.innerHTML = parts.join("");
  const fragment = document.importNode(templateElement.content, true);

  if (fragment.children.length === 1) {
    return fragment.firstElementChild;
  } else {
    return fragment;
  }
}

/**
 *
 * @param {string[]} strings
 * @param  {...any} expressions
 * @returns {CSSStyleSheet}
 */
export function css(strings, ...expressions) {
  const parts = [];

  for (let i = 0; i < strings.length; i += 1) {
    parts.push(strings[i]);
    if (expressions[i] !== undefined) parts.push(expressions[i]);
  }

  const cssText = parts.join("");
  const stylesheet = new CSSStyleSheet();
  stylesheet.replaceSync(cssText);
  return stylesheet;
}
