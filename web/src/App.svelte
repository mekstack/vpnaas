<script lang="ts">
    import ConfigBox from "./components/ConfigBox.svelte";
    import PubkeyBox from "./components/PubkeyBox.svelte";
    import ErrorBox from "./components/ErrorBox.svelte";
    import errorStore from "./stores/errorStore";
    import { addError } from "./stores/errorStore";

    import jwtDecode from "jwt-decode";
    import { onMount } from "svelte";
    import { StatusCode } from "grpc-web";

    import { ConfusClient } from "./grpc/VpnaasServiceClientPb";
    import { User, UserConfig } from "./grpc/vpnaas_pb";

    let accessToken = "";
    let username = "";
    let userPubkey = "";
    let userConfig: UserConfig;

    interface VPNaaSToken {
        username: string;
        exp: number;
    }

    checkForAccessTokenInHash();

    if (localStorage.getItem("accessToken")) {
        accessToken = localStorage.getItem("accessToken");
        username = jwtDecode<VPNaaSToken>(accessToken).username;
    }

    function checkForAccessTokenInHash() {
        const hashParams = new URLSearchParams(
            window.location.hash.substring(1)
        );

        if (hashParams.has("access_token")) {
            const accessToken = hashParams.get("access_token");
            localStorage.setItem("accessToken", accessToken);

            // Remove access_token from the hash fragment
            const newHashParams = new URLSearchParams(hashParams);
            newHashParams.delete("access_token");
            window.location.hash = newHashParams.toString();
        }
    }

    async function fetchUserConfig() {
        const configReq = new ConfusClient(
            "https://vpnaas.mekstack.ru"
        ).get_config(new User().setUsername(username), {}, (err, config) => {
            if (err) {
                if (err.code === StatusCode.NOT_FOUND) {
                } else {
                    console.error(err);
                    addError(
                        "Error fetching Wireguard configuration: " + err.message
                    );
                }
                return;
            }
            userConfig = config;
            userPubkey =
                config.getUserPeer().getPubkey()?.getBytes_asB64() || "";
        });
    }

    if (username != "") {
        onMount(async () => {
            fetchUserConfig();
        });
    }

    async function login() {
        const redirectBackUrl = encodeURIComponent(window.location.href);
        const authUrl = `https://auth.mekstack.ru/login?redirect_back_url=${redirectBackUrl}`;
        window.location.href = authUrl;
    }
</script>

<div class="error-container">
    {#each $errorStore as { message }}
        <ErrorBox {message} />
    {/each}
</div>

<div class="container">
    <div class="header">
        <h1>VPNaaS</h1>
        <a href="https://github.com/mekstack/vpnaas" class="link">github</a>
        <a href="https://docs.mekstack.ru" class="link">docs</a>
    </div>

    <div class="username">
        <h1>{username}</h1>
    </div>

    <div class="content">
        {#if username != ""}
            {#if userConfig}
                <ConfigBox {userConfig} />
            {/if}
            <PubkeyBox
                {userPubkey}
                {username}
                {accessToken}
                {fetchUserConfig}
            />
        {:else}
            <div class="login-container">
                <button class="login-button" on:click={login}>Log In</button>
            </div>
        {/if}
    </div>
</div>

<style>
    /* General Styles */

    :global(body) {
        background-color: black;
        color: lightgray;
        font-family: monospace;
    }

    ::selection {
        background: lightgray;
        color: black;
    }

    /* Container Styles */

    .content {
        position: absolute;
        top: 50%;
        left: 50%;
        min-width: 600px;
        transform: translate(-50%, -50%);
        justify-content: center;
        flex-direction: column;
        display: flex;
    }

    .login-container {
        justify-content: center;
        display: flex;
    }

    .username {
        position: absolute;
        top: 0;
        right: 0;
        display: flex;
        flex-direction: column;
        padding: 1em 2em;
    }

    .header {
        position: absolute;
        top: 0;
        left: 0;
        display: flex;
        flex-direction: column;
        padding: 1em 2em;
    }

    .error-container {
        position: fixed;
        top: 1em;
        right: 1em;
        z-index: 1000;
    }

    .link {
        color: limegreen;
        font-family: monospace;
        text-decoration: none;
    }

    .link:hover {
        color: white;
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
