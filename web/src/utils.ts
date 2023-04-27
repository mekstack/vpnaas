import type { UserConfig } from "./grpc/vpnaas_pb";
import { addError } from "./stores/errorStore";

function int2ip(num: number): string {
    return [24, 16, 8, 0].map((n) => (num >> n) & 0xff).join(".");
}

export function renderWireguardConfig(config: UserConfig) {
    const userPeer = config.getUserPeer();
    const serverPeer = config.getServerPeer();
    const ip = int2ip(userPeer.getIp());
    const dns = config.getDns();
    const pubkey = serverPeer.getPubkey()?.getBytes_asB64() || "";
    const allowedIps = serverPeer.getAllowedIpsList().join(", ");
    const endpoint = serverPeer.getEndpoint();

    const data = {
        ip,
        dns,
        pubkey,
        allowedIps,
        endpoint,
    };

    return `
[Interface]
Address = ${data.ip}
PrivateKey = YOUR_PRIVATE_KEY
DNS = ${data.dns}

[Peer]
PublicKey = ${data.pubkey}
AllowedIPs = ${data.allowedIps}
Endpoint = ${data.endpoint}
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
