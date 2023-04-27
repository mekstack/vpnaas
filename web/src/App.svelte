<script lang="ts">
    import Content from "./components/Content.svelte";
    import ErrorBox from "./components/ErrorBox.svelte";
    import errorStore from "./stores/errorStore";

    import jwtDecode from "jwt-decode";

    let accessToken: string;
    let username: string;

    interface VPNaaSToken {
        username: string;
        exp: number;
    }

    function checkForAccessTokenInHash() {
        const hashParams = new URLSearchParams(
            window.location.hash.substring(1)
        );

        if (hashParams.has("access_token")) {
            accessToken = hashParams.get("access_token");
            localStorage.setItem("accessToken", accessToken);

            // Remove access_token from the hash fragment
            const newHashParams = new URLSearchParams(hashParams);
            newHashParams.delete("access_token");
            window.location.hash = newHashParams.toString();
        }
    }
    checkForAccessTokenInHash();

    if (localStorage.getItem("accessToken")) {
        accessToken = localStorage.getItem("accessToken");
        username = jwtDecode<VPNaaSToken>(accessToken).username;
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

    {#if username}
        <div class="username">
            <h1>{username}</h1>
        </div>
    {/if}

    <div class="content">
        {#if username}
            <Content {username} {accessToken} />
        {:else}
            <button class="login-button" on:click={login}>Log In</button>
        {/if}
    </div>
</div>

<style>
    :global(body) {
        background-color: black;
        color: lightgray;
        font-family: monospace;
    }

    .content {
        position: absolute;
        top: 50%;
        left: 50%;
        min-width: 600px;
        transform: translate(-50%, -50%);
        justify-content: center;
        align-items: center;
        flex-direction: column;
        display: flex;
    }

    .header,
    .username {
        position: absolute;
        top: 0;
        display: flex;
        flex-direction: column;
        padding: 1em 2em;
    }

    .header {
        left: 0;
    }
    .username {
        right: 0;
    }

    .error-container {
        position: fixed;
        top: 1.5em;
        right: 1.5em;
        z-index: 1000;
    }

    .link,
    .login-button {
        color: limegreen;
        font-family: monospace;
        text-decoration: none;
    }

    .link:hover,
    .login-button:hover {
        color: white;
    }

    .login-button:active {
        color: white;
        background: none;
    }

    .login-button {
        background: none;
        border: none;
        font-size: 2em;
        cursor: pointer;
        width: fit-content;
    }
</style>
