/**
 * Helpers for selecting elements with CSS selectors.
 */

/**
 * querySelector wrapper that gets elements with class containing the given string.
 * Useful for getting elements where class names have a random string appended to them.
 * @param className The class name to search for
 * @returns The first element with a class starting with the given string, or null if none exists
 */
export function queryClass(className: string) {
  return document.querySelector(`[class*="${className}"]`);
}

/**
 * Gets the name of the current chat using the title at the top of the page.
 * @returns The name of the current chat, or null if no chat is open.
 */
export function getChatName() {
  return queryClass("titleWrapper")?.textContent;
}


/**
 * Gets the text in the chat box.
 * @returns The text in the chat box, or null if no chat is open.
 */
export function getChatBoxText() {
  return queryClass("editor_")?.textContent;
}


/**
 * Gets the buttons in the message bar.
 * @returns The buttons in the message bar, or null if no chat is open.
 */
export function getMessageBarButtons() {
  return queryClass("buttons");
}

export function getCurrentUsername() {
  return queryClass("panelTitleContainer")?.textContent;
}
