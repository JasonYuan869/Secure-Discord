enum MessageType {
  CONFIG,
  INIT
}

interface Message {
  type: MessageType,
  data: any,
}

interface SecureDiscordConfig {
  enabled: boolean;
  username: string;
  userId: number;
  identity: Uint8Array | null;
}
