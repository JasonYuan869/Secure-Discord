export enum MessageType {
  CONFIG,
  INIT,
  LOGGED_IN,
  OPEN_CHAT,
  ENCRYPT_MESSAGE,
  DECRYPT_MESSAGE,
  MESSAGE_CIPHERTEXT,
}

export interface Message {
  type: MessageType,
  data: any,
}

export interface UserIdentityInfo {
  username?: string;
  userId?: number;
  identityKeyPair?: string;
}

// base64 encoded string of SessionRecord protobuf to be stored in storage.sync
export type SavedSession = string;

export interface SecureDiscordConfig {
  enabled: boolean;
  identity: UserIdentityInfo;
}
