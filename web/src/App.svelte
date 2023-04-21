<script lang="ts">
    import { onMount } from "svelte";
    import { KeysClient, ConfusClient } from "./grpc/VpnaasServiceClientPb";
    import { User, UserPubkey, Pubkey, UserConfig } from "./grpc/vpnaas_pb";

    import jwtDecode from "jwt-decode";

    let pubkey = "";
    let wireguardConfig = "";
    let username = "unknown";
    let copyText = "copy";

    interface VPNaaSToken {
        username: string;
        exp: number;
    }

    onMount(async () => {
        const accessToken = localStorage.getItem("accessToken");
        username = jwtDecode<VPNaaSToken>(accessToken).username;

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

    function int2ip(int) {
        return [24, 16, 8, 0].map((n) => (int >> n) & 0xff).join(".");
    }

    function generateWireguardConfig(config: UserConfig) {
        const userPeer = config.getUserPeer();
        const serverPeer = config.getServerPeer();
        const ip = int2ip(userPeer.getIp());
        const dns = config.getDns();
        pubkey = serverPeer.getPubkey()?.getBytes_asB64() || "";
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

    async function addPubkey() {
        const accessToken = localStorage.getItem("accessToken");
        const username = jwtDecode<VPNaaSToken>(accessToken).username;

        const setPubkeyReq = new KeysClient("http://127.0.0.1:3000").set_pubkey(
            new UserPubkey()
                .setUser(new User().setUsername(username))
                .setPubkey(
                    new Pubkey().setBytes(new TextEncoder().encode(pubkey))
                ),
            { Authorization: `Bearer ${accessToken}` },
            (err, success) => {
                if (err) {
                    console.error(err);
                    return;
                }
                console.log("Public key added successfully.");
            }
        );
    }

    function logout() {
        localStorage.removeItem("accessToken");
        location.reload();
    }

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
</script>

<header>
    <div class="title-container">
        <h1>VPNaaS</h1>
        <a href="https://github.com/mekstack/vpnaas" class="link">github</a>
        <a href="https://docs.mekstack.ru" class="link">docs</a>
    </div>
    {#if localStorage.getItem("accessToken")}
        <div class="right">
            <span class="username">
                {username}<button class="logout-button" on:click={logout}
                    >-></button
                >
            </span>
        </div>
    {/if}
</header>

{#if localStorage.getItem("accessToken")}
    <div class="container">
        {#if wireguardConfig}
            <div class="config-container">
                <label class="config-label">Your wg0.conf:</label>
                <div class="copy-container">
                    <button class="copy-button" on:click={copyConfig}
                        >{copyText}</button
                    >
                </div>
                <textarea class="config-box" readonly
                    >{wireguardConfig}</textarea
                >
            </div>
            <div class="pubkey-container">
                <label class="pubkey-label">Your public key:</label>
                <input class="pubkey-box" type="text" readonly value={pubkey} />
            </div>
        {:else}
            <div>
                <label for="pubkey">Enter your public key:</label>
                <input type="text" id="pubkey" bind:value={pubkey} />
                <button on:click={addPubkey}>Add Public Key</button>
            </div>
        {/if}
    </div>
{:else}
    <div class="login-container">
        <a class="login-button" href="/login">Log In</a>
    </div>
{/if}

<style>
    :global(body) {
        background-color: black;
        color: lightgray;
        font-family: monospace;
    }

    ::selection {
        background: lightgray;
        color: black;
    }

    .container {
        max-width: 600px;
        margin: 0 auto;
    }

    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1em 2em;
    }

    .title-container {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .link {
        color: limegreen;
        font-family: monospace;
        font-size: 1em;
        text-decoration: none;
    }

    .link:hover {
        color: white;
    }

    .right {
        display: flex;
        align-items: center;
        justify-content: bottom;
        padding: 0 0;
    }

    .username {
        margin-right: 1em;
        background: none;
        border: none;
        font-size: 1.5em;
    }

    .logout-button {
        background: none;
        border: none;
        color: limegreen;
        font-family: monospace;
        cursor: pointer;
    }

    .logout-button:hover {
        color: white;
    }

    .config-container,
    .pubkey-container {
        margin: 2em 0;
    }

    .config-label,
    .pubkey-label {
        display: block;
        font-weight: bold;
        font-size: 1.5em;
        margin: 0.3em 0;
    }

    .config-box {
        display: block;
        width: 100%;
        height: auto;
        min-height: 200px;
        padding: 1em;
        border: 1px solid white;
        font-family: monospace;
        color: lightgray;
        background-color: black;
        resize: none;
        border-radius: 0;
    }

    .config-box:focus {
        outline: none;
        box-shadow: none;
    }

    .pubkey-box {
        display: block;
        width: 100%;
        padding: 0.5em 1em;
        border: 1px solid white;
        font-family: monospace;
        color: lightgray;
        background-color: black;
        resize: none;
        border-radius: 0;
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

    .login-container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 80vh;
    }

    .login-button {
        background: none;
        border: none;
        color: limegreen;
        font-family: monospace;
        font-size: 2em;
        cursor: pointer;
    }

    .login-button:hover {
        color: white;
    }
</style>
