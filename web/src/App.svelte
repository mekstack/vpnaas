<script lang="ts">
    import Header from "./components/Header.svelte";
    import UserInfo from "./components/UserInfo.svelte";
    import ConfigBox from "./components/ConfigBox.svelte";
    import PubkeyBox from "./components/PubkeyBox.svelte";
    import LoginButton from "./components/LoginButton.svelte";
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
    <Header />
    {#if username != ""}
        <div class="content">
            {#if userConfig}
                <ConfigBox {userConfig} />
            {/if}
            <PubkeyBox {userPubkey} {username} {accessToken} {fetchUserConfig} />
        </div>
        <UserInfo {username} />
    {:else}
        <LoginButton />
    {/if}
</div>

<style>
    /* General Styles */

    :global(body) {
        background-color: black;
        color: lightgray;
        font-family: monospace;
        height: 100vh;
        margin: 0;
    }

    ::selection {
        background: lightgray;
        color: black;
    }

    /* Container Styles */

    .container {
        display: flex;
        justify-content: space-between;
    }

    .content {
        flex-basis: 60%;
        max-width: 600px;
        margin-top: 4%; /* Adjust the margin as needed */
    }

    .error-container {
        position: fixed;
        top: 1em;
        right: 1em;
        z-index: 1000;
    }
</style>
