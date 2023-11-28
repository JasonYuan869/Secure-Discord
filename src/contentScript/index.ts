import { run } from "./uiInjection";

chrome.runtime.sendMessage({ type: 'SCRIPT', data: "content script running" });

run();
