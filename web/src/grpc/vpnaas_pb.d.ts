import * as jspb from 'google-protobuf'



export class Success extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Success.AsObject;
  static toObject(includeInstance: boolean, msg: Success): Success.AsObject;
  static serializeBinaryToWriter(message: Success, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Success;
  static deserializeBinaryFromReader(message: Success, reader: jspb.BinaryReader): Success;
}

export namespace Success {
  export type AsObject = {
  }
}

export class Empty extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Empty.AsObject;
  static toObject(includeInstance: boolean, msg: Empty): Empty.AsObject;
  static serializeBinaryToWriter(message: Empty, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Empty;
  static deserializeBinaryFromReader(message: Empty, reader: jspb.BinaryReader): Empty;
}

export namespace Empty {
  export type AsObject = {
  }
}

export class User extends jspb.Message {
  getUsername(): string;
  setUsername(value: string): User;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): User.AsObject;
  static toObject(includeInstance: boolean, msg: User): User.AsObject;
  static serializeBinaryToWriter(message: User, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): User;
  static deserializeBinaryFromReader(message: User, reader: jspb.BinaryReader): User;
}

export namespace User {
  export type AsObject = {
    username: string,
  }
}

export class Pubkey extends jspb.Message {
  getBytes(): Uint8Array | string;
  getBytes_asU8(): Uint8Array;
  getBytes_asB64(): string;
  setBytes(value: Uint8Array | string): Pubkey;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Pubkey.AsObject;
  static toObject(includeInstance: boolean, msg: Pubkey): Pubkey.AsObject;
  static serializeBinaryToWriter(message: Pubkey, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Pubkey;
  static deserializeBinaryFromReader(message: Pubkey, reader: jspb.BinaryReader): Pubkey;
}

export namespace Pubkey {
  export type AsObject = {
    bytes: Uint8Array | string,
  }
}

export class UserPubkey extends jspb.Message {
  getUser(): User | undefined;
  setUser(value?: User): UserPubkey;
  hasUser(): boolean;
  clearUser(): UserPubkey;

  getPubkey(): Pubkey | undefined;
  setPubkey(value?: Pubkey): UserPubkey;
  hasPubkey(): boolean;
  clearPubkey(): UserPubkey;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserPubkey.AsObject;
  static toObject(includeInstance: boolean, msg: UserPubkey): UserPubkey.AsObject;
  static serializeBinaryToWriter(message: UserPubkey, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserPubkey;
  static deserializeBinaryFromReader(message: UserPubkey, reader: jspb.BinaryReader): UserPubkey;
}

export namespace UserPubkey {
  export type AsObject = {
    user?: User.AsObject,
    pubkey?: Pubkey.AsObject,
  }
}

export class Peer extends jspb.Message {
  getPubkey(): Pubkey | undefined;
  setPubkey(value?: Pubkey): Peer;
  hasPubkey(): boolean;
  clearPubkey(): Peer;

  getIp(): number;
  setIp(value: number): Peer;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Peer.AsObject;
  static toObject(includeInstance: boolean, msg: Peer): Peer.AsObject;
  static serializeBinaryToWriter(message: Peer, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Peer;
  static deserializeBinaryFromReader(message: Peer, reader: jspb.BinaryReader): Peer;
}

export namespace Peer {
  export type AsObject = {
    pubkey?: Pubkey.AsObject,
    ip: number,
  }
}

export class Peers extends jspb.Message {
  getPeersList(): Array<Peer>;
  setPeersList(value: Array<Peer>): Peers;
  clearPeersList(): Peers;
  addPeers(value?: Peer, index?: number): Peer;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Peers.AsObject;
  static toObject(includeInstance: boolean, msg: Peers): Peers.AsObject;
  static serializeBinaryToWriter(message: Peers, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Peers;
  static deserializeBinaryFromReader(message: Peers, reader: jspb.BinaryReader): Peers;
}

export namespace Peers {
  export type AsObject = {
    peersList: Array<Peer.AsObject>,
  }
}

export class ServerPeer extends jspb.Message {
  getPubkey(): Pubkey | undefined;
  setPubkey(value?: Pubkey): ServerPeer;
  hasPubkey(): boolean;
  clearPubkey(): ServerPeer;

  getEndpoint(): string;
  setEndpoint(value: string): ServerPeer;

  getAllowedIpsList(): Array<string>;
  setAllowedIpsList(value: Array<string>): ServerPeer;
  clearAllowedIpsList(): ServerPeer;
  addAllowedIps(value: string, index?: number): ServerPeer;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ServerPeer.AsObject;
  static toObject(includeInstance: boolean, msg: ServerPeer): ServerPeer.AsObject;
  static serializeBinaryToWriter(message: ServerPeer, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ServerPeer;
  static deserializeBinaryFromReader(message: ServerPeer, reader: jspb.BinaryReader): ServerPeer;
}

export namespace ServerPeer {
  export type AsObject = {
    pubkey?: Pubkey.AsObject,
    endpoint: string,
    allowedIpsList: Array<string>,
  }
}

export class UserConfig extends jspb.Message {
  getUser(): Peer | undefined;
  setUser(value?: Peer): UserConfig;
  hasUser(): boolean;
  clearUser(): UserConfig;

  getInterfaceConfigList(): Array<string>;
  setInterfaceConfigList(value: Array<string>): UserConfig;
  clearInterfaceConfigList(): UserConfig;
  addInterfaceConfig(value: string, index?: number): UserConfig;

  getPeerConfigList(): Array<string>;
  setPeerConfigList(value: Array<string>): UserConfig;
  clearPeerConfigList(): UserConfig;
  addPeerConfig(value: string, index?: number): UserConfig;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserConfig.AsObject;
  static toObject(includeInstance: boolean, msg: UserConfig): UserConfig.AsObject;
  static serializeBinaryToWriter(message: UserConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserConfig;
  static deserializeBinaryFromReader(message: UserConfig, reader: jspb.BinaryReader): UserConfig;
}

export namespace UserConfig {
  export type AsObject = {
    user?: Peer.AsObject,
    interfaceConfigList: Array<string>,
    peerConfigList: Array<string>,
  }
}

