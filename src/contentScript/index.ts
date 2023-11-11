import "./styles.css";

chrome.runtime.sendMessage({ type: 'SCRIPT', data: "content script running" });

let currentChat = document.querySelector(".titleWrapper__482dc")?.textContent;
let messageBarButtons = document.querySelector(".buttons_ce5b56");


const customButton = document.createElement("div");
customButton.className = "expression-picker-chat-input-button buttonContainer__8b164";
customButton.innerHTML = `
<button name="encryptButton" class="button_f0455c emojiButtonNormal__73cd3 emojiButton__8ff6a emojiButton__30ec7 button__55e53 button_afdfd9 colorBrand_b2253e grow__4c8a4">
  <div class="contents_fb6220">
    <i class="bi-lock-fill customEmojiButton"></i>
  </div>
</button>
`;

customButton.addEventListener("click", () => {
  console.log("clicked in chat", currentChat);
  const chatBox = document.querySelector(".editor__66464 > div > span > span");
  if (chatBox) {
    console.log("chatbox", chatBox.textContent);
    chatBox.textContent = "🔒" + chatBox?.textContent;
  } else {
    console.log("no chatbox");
  }
});

const addCustomButton = () => {
  messageBarButtons = document.querySelector(".buttons_ce5b56");
  messageBarButtons?.appendChild(customButton);
};


const chatObserver = new MutationObserver((mutations) => {
  const chatName = document.querySelector(".titleWrapper__482dc")?.textContent;
  if (chatName && chatName !== currentChat) {
    currentChat = chatName;
    addCustomButton();
    console.log("in a chat with", currentChat);
  }
})

chatObserver.observe(document.body, { childList: true, subtree: true });
