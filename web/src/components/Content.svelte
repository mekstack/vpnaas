<script lang="ts">
    export let username: string;
    export let accessToken: string;

    import { addError } from "../stores/errorStore";
    import { KeysClient, ConfusClient } from "../grpc/VpnaasServiceClientPb";
    import { User, UserConfig, UserPubkey, Pubkey } from "../grpc/vpnaas_pb";

    import { onMount } from "svelte";
    import { StatusCode } from "grpc-web";

    let userConfig: UserConfig;
    let setPubkeyButtonText: string;
    let pubkeyBoxLabel: string;
    let userPubkeyB64: string;
    let isPubkeySet = false;

    let copyConfigButtonText = "copy";
    let renderedConfig: string;

    onMount(async () => {
        getUserConfig();
    });

    $: {
        if (userConfig) {
            renderedConfig = renderWireguardConfig(userConfig);
        }
    }

    $: {
        if (isPubkeySet) {
            setPubkeyButtonText = "set!";
            pubkeyBoxLabel = "Your public key:";
        } else {
            setPubkeyButtonText = "set";
            pubkeyBoxLabel = "Enter your new public key:";
        }
    }

    async function copyConfig() {
        const configTextArea = document.querySelector(
            ".config-box"
        ) as HTMLTextAreaElement;
        configTextArea.select();

        try {
            await navigator.clipboard.writeText(configTextArea.value);
            copyConfigButtonText = "copied!";
        } catch (err) {
            copyConfigButtonText = "error";
            console.error("Failed to copy text: ", err);
        }

        setTimeout(() => (copyConfigButtonText = "copy"), 2000);
    }

    export function renderWireguardConfig(config: UserConfig) {
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

    function int2ip(int: number) {
        return [24, 16, 8, 0].map((n) => (int >> n) & 0xff).join(".");
    }

    async function getUserConfig() {
        new ConfusClient("https://vpnaas.mekstack.ru").get_config(
            new User().setUsername(username),
            {},
            (err, config) => {
                if (err) {
                    if (err.code === StatusCode.NOT_FOUND) {
                    } else {
                        console.error(err);
                        addError(
                            "Error getting Wireguard configuration: " +
                                err.message
                        );
                    }
                    return;
                }
                userConfig = config;
                userPubkeyB64 =
                    config.getUserPeer().getPubkey()?.getBytes_asB64() || "";
                isPubkeySet = true;
            }
        );
    }

    async function setPubkey() {
        // Decode the base64-encoded string and convert it to a Uint8Array
        let userPubkeyBytes = "";
        try {
            userPubkeyBytes = window.atob(userPubkeyB64);
        } catch (_) {
            addError("Invalid base64 string");
            return;
        }

        const bytes = Uint8Array.from(userPubkeyBytes, (c) => c.charCodeAt(0));

        new KeysClient("https://vpnaas.mekstack.ru").set_pubkey(
            new UserPubkey()
                .setUser(new User().setUsername(username))
                .setPubkey(new Pubkey().setBytes(bytes)),
            { Authorization: `Bearer ${accessToken}` },
            (err, _) => {
                if (err) {
                    console.error(err);
                    addError("Error setting public key: " + err.message);
                    return;
                }
                getUserConfig();
            }
        );
    }
</script>

{#if renderedConfig}
    <div class="config-container">
        <div class="title-container">
            <label class="config-label" for="config-box">Your wg0.conf:</label>
            <button class="button" on:click={copyConfig}
                >{copyConfigButtonText}</button
            >
        </div>
        <textarea class="config-box" readonly>{renderedConfig}</textarea>
    </div>
{/if}
<div class="pubkey-container">
    <div class="title-container">
        <label class="pubkey-label" for="pubkey-box">{pubkeyBoxLabel}</label>
        <button class="button" on:click={setPubkey}
            >{setPubkeyButtonText}</button
        >
    </div>
    <input
        type="text"
        class="pubkey-box"
        value={userPubkeyB64}
        on:input={(e) => {
            isPubkeySet = false;
            userPubkeyB64 = e.target.value;
        }}
        on:keydown={(e) => {
            if (e.key === "Enter") {
                setPubkey();
            }
        }}
        placeholder="wg genkey | tee priv.key | wg pubkey"
    />
</div>

<style>
    .config-box,
    .pubkey-box {
        display: block;
        width: 100%;
        padding: 0.5em 1em;
        border: 1px solid rgb(48, 52, 54);
        font-family: monospace;
        color: lightgray;
        background-color: black;
        resize: none;
        border-radius: 0;
        outline: none;
    }

    .config-box {
        min-height: 200px;
    }

    .config-container,
    .pubkey-container {
        width: 100%;
        position: relative;
        margin: 0.5em 0;
    }

    .config-label,
    .pubkey-label {
        display: block;
        font-weight: bold;
        font-size: 1.5em;
    }

    .title-container {
        display: flex;
        justify-content: space-between;
        padding: 0.3em 0;
    }

    .button {
        padding: 0 2px;
        margin: 0;
        background: none;
        border: none;
        border-radius: 0;
        color: limegreen;
        font-family: monospace;
        cursor: pointer;
    }

    .button:hover {
        color: white;
    }

    .button:active {
        background: none;
    }
</style>
