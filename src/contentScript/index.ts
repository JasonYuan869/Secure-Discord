import { run } from "./uiInjection";
import { Message, MessageType } from "../types";
import "../styles.css";
import "./css_fix.css";

function handlePortMessage(message: Message) {
  console.log("contentScript received message", message);
  switch (message.type) {
    case MessageType.INIT:
      console.log("Received response from background script");
      run(port);
      break;
  }
}

const port = chrome.runtime.connect({ name: "contentScript" });
port.onMessage.addListener(handlePortMessage);
