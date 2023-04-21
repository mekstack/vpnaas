<script lang="ts">
    export let wireguardConfig: string;
    export let copyText: string;
    export let username: string;

    import { onMount } from "svelte";

    import { ConfusClient } from "../grpc/VpnaasServiceClientPb";
    import { User, UserConfig } from "../grpc/vpnaas_pb";

    onMount(async () => {
        const configReq = new ConfusClient("http://127.0.0.1:4448").get_config(
            new User().setUsername(username),
            {},
            (err, config) => {
                if (err) {
                    console.error(err);
                    return;
                }
                wireguardConfig = generateWireguardConfig(config);
            }
        );
    });

    async function copyConfig() {
        const configTextArea = document.querySelector(
            ".config-box"
        ) as HTMLTextAreaElement;
        configTextArea.select();

        try {
            await navigator.clipboard.writeText(configTextArea.value);
            copyText = "copied!";
        } catch (err) {
            copyText = "error";
            console.error("Failed to copy text: ", err);
        }

        setTimeout(() => (copyText = "copy"), 2000);
    }

    // Helper Functions
    function int2ip(int: number) {
        return [24, 16, 8, 0].map((n) => (int >> n) & 0xff).join(".");
    }

    export function generateWireguardConfig(config: UserConfig) {
        const userPeer = config.getUserPeer();
        const serverPeer = config.getServerPeer();
        const ip = int2ip(userPeer.getIp());
        const dns = config.getDns();
        const pubkey = serverPeer.getPubkey()?.getBytes_asB64() || "";
        const allowedIps = serverPeer.getAllowedIpsList().join(", ");
        const endpoint = serverPeer.getEndpoint();

        return `\
[Interface]
Address = ${ip}
PrivateKey = YOUR_PRIVATE_KEY
DNS = ${dns}

[Peer]
PublicKey = ${pubkey}
AllowedIPs = ${allowedIps}
Endpoint = ${endpoint}
`;
    }
</script>

<div class="config-container">
    <label class="config-label">Your wg0.conf:</label>
    <div class="copy-container">
        <button class="copy-button" on:click={copyConfig}>{copyText}</button>
    </div>
    <textarea class="config-box" readonly>{wireguardConfig}</textarea>
</div>

<style>
    .config-box {
        display: block;
        width: 100%;
        height: auto;
        min-height: 200px;
        padding: 1em;
        border: 1px solid rgb(48, 52, 54);
        font-family: monospace;
        color: lightgray;
        background-color: black;
        resize: none;
        border-radius: 0;
    }

    .config-container {
        margin: 2em 0;
    }

    .config-label {
        display: block;
        font-weight: bold;
        font-size: 1.5em;
        margin: 0.3em 0;
    }

    .config-box:focus {
        outline: none;
        box-shadow: none;
    }

    .config-container {
        position: relative;
        margin: 1em 0;
    }

    .copy-container {
        position: absolute;
        top: 0;
        right: 0;
    }

    .copy-button {
        background: none;
        border: none;
        color: limegreen;
        font-family: monospace;
        cursor: pointer;
    }

    .copy-button:hover {
        color: white;
    }
</style>
