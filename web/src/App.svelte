<script lang="ts">
    import Content from "./components/Content.svelte";
    import ErrorBox from "./components/ErrorBox.svelte";

    import errorStore from "./stores/errorStore";
    import userStore, { setUsername, setAccessToken } from "./stores/userStore";

    import {
        getAccessTokenAfterSignin,
        getUsernameFromAccessToken,
        getAccessTokenFromLocalStorage,
        checkTokenExpiry,
        login,
    } from "./services/auth";

    import { onMount } from "svelte";

    let accessToken: string;
    let username: string;

    userStore.subscribe(($userStore) => {
        username = $userStore.username;
        accessToken = $userStore.accessToken;
    });

    onMount(async () => {
        checkTokenExpiry();

        const newAccessToken = await getAccessTokenAfterSignin();
        const localAccessToken = getAccessTokenFromLocalStorage();

        const accessToken = newAccessToken || localAccessToken;
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
        <a href="https://mekstack.ru" class="link">console</a>
        <a href="https://github.com/mekstack/vpnaas" class="link">star me</a>
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
