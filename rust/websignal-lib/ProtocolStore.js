/**
 * Stores data in chrome.storage.sync. Since byte arrays cannot be stored directly (only JSON-serializable objects),
 * we convert them to base64 strings before storing them.
 */

/**
 * Converts a base64 string to a `Uint8Array`.
 * @param {string} base64
 * @return {Uint8Array}
 */
function base64ToUint8Array(base64) {
  return Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
}

/**
 * Converts a `Uint8Array` to a base64 string.
 * @param {Uint8Array} array
 * @return {string}
 */
function uint8ArrayToBase64(array) {
  return btoa(String.fromCharCode(...array));
}

// export class SessionStore {
//
// }
//
// export class PreKeyStore {
//
// }
//
// export class SignedPreKeyStore {
//
// }
//
// export class KyberPreKeyStore {
//
// }

export class IdentityKeyStore {
  /**
   * @param {Uint8Array} keyPair
   * @param {number} registrationId
   */
  constructor(keyPair, registrationId) {
    this.keyPair = keyPair;
    this.registrationId = registrationId;

    // Take the middle 16 bytes of the keypair as the store name.
    this.storeName = `${uint8ArrayToBase64(this.keyPair).slice(10, 10 + 16)}-identityKeyStore`;
  }

  async getIdentityKeyPair() {
    return this.keyPair;
  }

  async getLocalRegistrationId() {
    return this.registrationId;
  }

  /**
   * Saves an identity to the store for this owner.
   * @param {string} username
   * @param {Uint8Array} identityKey
   * @return {Promise<boolean>} - true if the operation replaced an existing identity, false if it did not.
   */
  async saveIdentity(username, identityKey) {
    const identityKeyBase64 = uint8ArrayToBase64(identityKey);
    const currentStore = await chrome.storage.sync.get(this.storeName);
    if (!currentStore[username] || currentStore[username].identityKey !== identityKeyBase64) {
      const updatedStore = { ...currentStore, [username]: { identityKey: identityKeyBase64 } };
      await chrome.storage.sync.set({ [this.storeName]: updatedStore });
      return !!currentStore[username];
    }
    return false; // no change
  }

  /**
   * Check whether an identity is trusted.
   *
   * An identity is trusted if the identity key is the same as the one in the store,
   * or if there is no key in the store for this user.
   *
   * @param {string} username
   * @param {Uint8Array} identityKey
   * @param {bool} _direction - Unused
   * @return {Promise<boolean>} - true if the identity is trusted, false if it is not.
   */
  async isTrustedIdentity(username, identityKey, _direction) {
    const currentStore = await chrome.storage.sync.get(this.storeName);
    if (currentStore[username]) {
      return currentStore[username].identityKey === uint8ArrayToBase64(identityKey);
    }
    return true;
  }

  async getIdentity(username) {
    const currentStore = await chrome.storage.sync.get(this.storeName);
    if (currentStore[username]) {
      return base64ToUint8Array(currentStore[username].identityKey);
    }
    return null;
  }
}

// export class SenderKeyStore {
//
// }

export class SignalProtocolStore {
  constructor(keyPair, registrationId) {
    // this.sessionStore = new SessionStore();
    // this.preKeyStore = new PreKeyStore();
    // this.signedPreKeyStore = new SignedPreKeyStore();
    // this.kyberPreKeyStore = new KyberPreKeyStore();
    this.identityKeyStore = new IdentityKeyStore(keyPair, registrationId);
    // this.senderKeyStore = new SenderKeyStore();
  }

  async getIdentityKeyPair() {
    return await this.identityKeyStore.getIdentityKeyPair();
  }

  async getLocalRegistrationId() {
    return await this.identityKeyStore.getLocalRegistrationId();
  }

  async saveIdentity(username, identityKey) {
    return await this.identityKeyStore.saveIdentity(username, identityKey);
  }

  async isTrustedIdentity(username, identityKey, direction) {
    return await this.identityKeyStore.isTrustedIdentity(username, identityKey, direction);
  }

  async getIdentity(username) {
    return await this.identityKeyStore.getIdentity(username);
  }
}
