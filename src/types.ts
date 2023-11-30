export enum MessageType {
  CONFIG,
  INIT
}

export interface Message {
  type: MessageType,
  data: any,
}

export interface UserIdentityInfo {
  username?: string;
  userId?: number;
  identityKeyPair?: Uint8Array;
}
export interface SecureDiscordConfig {
  enabled: boolean;
  identity: UserIdentityInfo;
}
