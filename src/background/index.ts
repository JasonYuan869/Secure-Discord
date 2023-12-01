import { Message, MessageType, SecureDiscordConfig } from "../types";
import { test } from "websignal-lib";

function onLogin(username: string) {
  console.log('Logged in as', username);

  chrome.storage.sync.get('config', (data) => {
    if (data.config) {
      const config = data.config as SecureDiscordConfig;
      if (config.identity.username !== username) {
        console.log("Logged in identity does not match config")

        // Open options page to set configuration
        chrome.runtime.openOptionsPage();
      }
    }
  });
}

function handleContentScriptMessage(request: Message) {
  switch (request.type) {
    case MessageType.LOGGED_IN:
      const { username } = request.data;
      onLogin(username);
      break;
    case MessageType.OPEN_CHAT:
      // TODO: Do we have a saved recipient for this chat? If not, prompt user for public key and pre key bundle
      console.log(request.data);

      break;
    case MessageType.ENCRYPT_MESSAGE:
      // TODO: Use saved recipient to encrypt message
      console.log(request.data);

      break;
    case MessageType.DECRYPT_MESSAGE:
      // TODO: Use saved recipient to decrypt message
      console.log(request.data);
      break;
  }
}

function handleOptionsPageMessage(request: Message) {
  switch (request.type) {
    case MessageType.CONFIG:
      // Configuration was updated in options page, reload
      chrome.runtime.reload();
  }
}

function checkConfiguration(port: chrome.runtime.Port) {
  chrome.storage.sync.get('config', (data) => {
    if (data.config) {
      console.log('Configuration found!', data.config);
      const config = data.config as SecureDiscordConfig;
      if (!config.enabled) {
        console.log('Extension is disabled');
        return;
      }

      port.postMessage({ type: MessageType.INIT });
    } else {
      console.log('Config is not set');

      // Open options page to set configuration
      chrome.runtime.openOptionsPage();
      return;
    }
  });
}

console.log('Web worker is running');
test().then(() => {
  chrome.storage.sync.get('bob', (data) => {
    console.log("data", data);
  });
});


chrome.runtime.onConnect.addListener((port) => {
  if (port.name === "optionsPage") {
    port.onMessage.addListener(handleOptionsPageMessage);
    return;
  } else if (port.name !== "contentScript") {
    console.log('Unknown port name', port.name);
    return;
  }

  port.onMessage.addListener(handleContentScriptMessage);

  // Check for configuration in storage
  checkConfiguration(port);
});
