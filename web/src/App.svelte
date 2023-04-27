<script lang="ts">
    import Content from "./components/Content.svelte";
    import ErrorBox from "./components/ErrorBox.svelte";

    import errorStore from "./stores/errorStore";
    import userStore, { setUsername, setAccessToken } from "./stores/userStore";

    import {
        checkForAccessTokenInHash,
        getUsernameFromAccessToken,
        getAccessTokenFromLocalStorage,
        login,
    } from "./services/auth";

    import { onMount } from "svelte";

    let accessToken: string;
    let username: string;

    userStore.subscribe(($userStore) => {
        username = $userStore.username;
        accessToken = $userStore.accessToken;
    });

    onMount(() => {
        const accessTokenFromHash = checkForAccessTokenInHash();
        const accessTokenFromLocalStorage = getAccessTokenFromLocalStorage();

        const accessToken = accessTokenFromHash || accessTokenFromLocalStorage;
        if (accessToken) {
            setAccessToken(accessToken);
            setUsername(getUsernameFromAccessToken(accessToken));
        }
    });
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
        {#if accessToken}
            <Content />
        {:else}
            <button class="login-button" on:click={login}>Log In</button>
        {/if}
    </div>
</div>

<style src="./App.css"></style>
