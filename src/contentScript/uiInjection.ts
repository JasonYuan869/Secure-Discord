import { queryClass, getChatName, getMessageBarButtons, getChatBoxText, getCurrentUsername } from "./utils/domHelpers";
import SecureButton from './components/SecureButton.svelte';
import Toasts from "./components/Toasts.svelte";
import FloatingWindow from "./components/Tooltip.svelte";
import { currentChat, currentUser } from "./stores";
import { get } from "svelte/store";
import { MessageType } from "../types";

const addToastContainer = () => {
  // Check if the toasts injection already exists
  const existingToastContainer = queryClass("svelte-toasts-container");
  if (existingToastContainer) {
    return;
  }

  // Add the toasts injection to the page
  const svelteToastContainer = document.createElement("div");
  document.body.appendChild(svelteToastContainer);

  const toastContainer = new Toasts({
    target: svelteToastContainer,
  });
};

export function run(port: chrome.runtime.Port) {

  const addSecureButton = () => {
    const messageBarButtons = getMessageBarButtons();
    if (!messageBarButtons) {
      return;
    }

    // Duplicate one of the buttons
    const svelteButton = messageBarButtons.children[2].cloneNode(true);
    if (!svelteButton) {
      return;
    }
    const oldButtonIcon = svelteButton.lastChild?.lastChild?.lastChild;
    if (!oldButtonIcon || !oldButtonIcon.parentElement) {
      return;
    }

    const buttonWrapper = oldButtonIcon.parentElement;
    buttonWrapper.removeChild(oldButtonIcon);

    // Svelte injection!
    const secureButton = new SecureButton({
      target: buttonWrapper,
      props: { port }
    });

    // Add the button to the message bar buttons
    messageBarButtons.appendChild(svelteButton);
  };
  addToastContainer();

  // Observer that is run whenever the chat changes
  const chatObserver = new MutationObserver(() => {
    const username = getCurrentUsername();
    if (username && username !== get(currentUser)) {
      currentUser.set(username);
      port.postMessage({ type: MessageType.LOGGED_IN, data: { username } });
    }

    const chatName = getChatName();
    if (chatName && chatName !== get(currentChat) && !chatName.includes(":") && chatName !== "Discord") {
      currentChat.set(chatName);
      port.postMessage({ type: MessageType.OPEN_CHAT, data: { chatName } });
      addSecureButton();
    }
  });

  // Start the chat observer
  chatObserver.observe(document.body, { childList: true, subtree: true });
}
