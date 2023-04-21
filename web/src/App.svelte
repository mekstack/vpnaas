<script lang="ts">
    import Header from "./components/Header.svelte";
    import UserInfo from "./components/UserInfo.svelte";
    import ConfigBox from "./components/ConfigBox.svelte";
    import PubkeyBox from "./components/PubkeyBox.svelte";
    import LoginButton from "./components/LoginButton.svelte";
    import ErrorBox from "./components/ErrorBox.svelte";
    import errorStore from "./stores/errorStore";

    import jwtDecode from "jwt-decode";

    let userPubkey = "";
    let accessToken = "";
    let username = "unknown";

    if (localStorage.getItem("accessToken")) {
        accessToken = localStorage.getItem("accessToken");
        username = jwtDecode<VPNaaSToken>(accessToken).username;
    }

    let wireguardConfig = "";
    let copyText = "copy";
    let pubkeyBoxLabel = "Enter your public key:";
    let setPubkeyText = "";
    let displayConfig = true;

    interface VPNaaSToken {
        username: string;
        exp: number;
    }
</script>

<div class="error-container">
    {#each $errorStore as { message }}
        <ErrorBox {message} />
    {/each}
</div>

<div class="container">
    <Header />
    {#if accessToken}
        <div class="content">
            <ConfigBox
                {wireguardConfig}
                {copyText}
                {username}
                {displayConfig}
                bind:userPubkey
            />
            <PubkeyBox
                {setPubkeyText}
                {userPubkey}
                {pubkeyBoxLabel}
                {username}
                {accessToken}
            />
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
