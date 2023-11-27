# Secure Discord

A WIP browser extension that adds end-to-end encryption to Discord messaging. Note that everything is experimental and no security guarantees are made.
There are no plans to support servers or group chats. This is purely for 1-on-1 DMs.

## Planning
- Use Signal's [double ratchet](https://signal.org/docs/specifications/doubleratchet/) for encryption
- Cryptography should be done via the [Web Crypto API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Crypto_API)
- Secure private key storage with a master encryption key from the user
- Public keys can be entered in the user's Discord profile to initiate a new secure chat

## Current Progress
- [x] Create the repository
- [] Basic UI injection into Discord's web app ("send secure message" button, "decrypt message" button)
  - Cannot "hijack" user inputs (typing, enter key, etc.). Will look into auto copying the encrypted text and prompting the user to paste it in the box.
- [] Implement the double ratchet algorithm
- [] Implement key storage

## Questions
- Can I use `libsignal` directly?
  - Probably not, some of the dependencies don't compile to wasm target so it would be difficult to run in the browser
