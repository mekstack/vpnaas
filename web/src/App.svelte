<script lang="ts">
    import Header from "./components/Header.svelte";
    import UserInfo from "./components/UserInfo.svelte";
    import ConfigBox from "./components/ConfigBox.svelte";
    import PubkeyBox from "./components/PubkeyBox.svelte";
    import LoginButton from "./components/LoginButton.svelte";

    import jwtDecode from "jwt-decode";

    let accessToken = "";
    let username = "unknown";

    if (localStorage.getItem("accessToken")) {
        accessToken = localStorage.getItem("accessToken");
        username = jwtDecode<VPNaaSToken>(accessToken).username;
    }

    let userPubkey = "";
    let wireguardConfig = "";
    let copyText = "copy";
    let pubkeyBoxLabel = "Enter your public key:";

    interface VPNaaSToken {
        username: string;
        exp: number;
    }

    function logout() {
        localStorage.removeItem("accessToken");
        location.reload();
    }
</script>

<div class="container">
    <Header />
    {#if accessToken}
        <div class="content">
            <ConfigBox {wireguardConfig} {copyText} {username} />
            <PubkeyBox {userPubkey} {pubkeyBoxLabel} {username} {accessToken} />
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
        /* justify-content: space-between; */
    }

    .content {
        flex-basis: 60%;
        max-width: 600px;
        margin-top: 4%; /* Adjust the margin as needed */
    }
</style>
