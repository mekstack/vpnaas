import type { UserConfig } from "./grpc/vpnaas_pb";
import { addError } from "./stores/errorStore";

function int2ip(num: number): string {
    return [24, 16, 8, 0].map((n) => (num >> n) & 0xff).join(".");
}

export function renderWireguardConfig(config: UserConfig) {
    const interfaceConfigBlock = config.getInterfaceConfigList().join("\n");
    const peerConfigBlock = config.getPeerConfigList().join("\n");
    const ip = int2ip(config.getUser()?.getIp() || 0);

    return `
[Interface]
Address = ${ip}
${interfaceConfigBlock}

[Peer]
${peerConfigBlock}
`.trim();
}

export function base64ToBytes(base64: string): Uint8Array {
    const binaryStr = window.atob(base64);
    const bytes = new Uint8Array(binaryStr.length);
    for (let i = 0; i < binaryStr.length; i++) {
        bytes[i] = binaryStr.charCodeAt(i);
    }
    return bytes;
}

export function logError(err: Error): void {
    console.error(err);
    addError(err.message);
}
