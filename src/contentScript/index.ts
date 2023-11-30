import { run } from "./uiInjection";
import { Message, MessageType } from "../types";
function handlePortMessage(message: Message) {
  console.log("contentScript received message", message);
  switch (message.type) {
    case MessageType.INIT:
      console.log("Received response from background script");
      import("../styles.css");
      run();
      break;
  }
}

const port = chrome.runtime.connect({ name: "contentScript" });
port.onMessage.addListener(handlePortMessage);

export function sendMessage(message: Message) {
  port.postMessage(message);
}
