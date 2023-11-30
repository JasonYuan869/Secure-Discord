import { Message, MessageType, SecureDiscordConfig } from "../types";

function handleContentScriptMessage(request: Message) {
  switch (request.type) {
    case MessageType.CONFIG:
      console.log(request.data);
      break;
    case MessageType.INIT:
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
