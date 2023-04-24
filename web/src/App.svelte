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

    if (localStorage.getItem("accessToken")) {
        accessToken = localStorage.getItem("accessToken");
        username = jwtDecode<VPNaaSToken>(accessToken).username;
    }

    async function fetchUserConfig() {
        const configReq = new ConfusClient("http://127.0.0.1:4448").get_config(
            new User().setUsername(username),
            {},
            (err, config) => {
                if (err) {
                    if (err.code === StatusCode.NOT_FOUND) {
                    } else {
                        console.error(err);
                        addError(
                            "Error fetching Wireguard configuration: " +
                                err.message
                        );
                    }
                    return;
                }
                userConfig = config;
                userPubkey =
                    config.getUserPeer().getPubkey()?.getBytes_asB64() || "";
            }
        );
    }

    if (username != "") {
        if (username != "") {
            onMount(async () => {
                fetchUserConfig();
            });
        }
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
                <a class="login-button" href="/login">Log In</a>
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
    }

    .username {
        position: absolute;
        top: 0;
        right: 0;
        display: flex;
        flex-direction: column;
        align-items: left;
        padding: 1em 2em;
    }

    .header {
        position: absolute;
        top: 0;
        left: 0;
        display: flex;
        flex-direction: column;
        align-items: left;
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
        font-size: em;
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
