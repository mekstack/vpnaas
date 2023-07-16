<script lang="ts">
    import userStore from "../stores/userStore";
    import { renderWireguardConfig, logError, base64ToBytes } from "../utils";
    import * as grpc from "../services/grpc";
    import * as grpcWeb from "grpc-web";

    import { UserConfig, Pubkey } from "../grpc/vpnaas_pb";

    import { onMount } from "svelte";

    let username: string;
    let accessToken: string;

    userStore.subscribe(
        ({ username: storeUsername, accessToken: storeAccessToken }) => {
            username = storeUsername;
            accessToken = storeAccessToken;
        }
    );

    let userConfig: UserConfig;
    let userPubkeyBase64: string = "";
    let isPubkeySet = false;
    let setPubkeyButtonText: string;
    let pubkeyBoxLabel: string;
    let copyConfigButtonText = "copy";
    let renderedConfig: string;

    async function getConfig() {
        try {
            userConfig = await grpc.getConfig(username);
            userPubkeyBase64 =
                userConfig.getUserPeer().getPubkey()?.getBytes_asB64() || "";
            isPubkeySet = true;
        } catch (err) {
            if (err.code === grpcWeb.StatusCode.NOT_FOUND) { // user has no config yet
                userPubkeyBase64 = "";
                isPubkeySet = false;
            } else {
                logError(err);
            }
        }
    }

    async function setPubkey() {
        try {
            const pubkey = new Pubkey().setBytes(
                base64ToBytes(userPubkeyBase64.replace(/=+$/, ''))
            );
            await grpc.setPubkey(username, accessToken, pubkey);
            getConfig();
        } catch (err) {
            logError(err);
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

    onMount(async () => {
        getConfig();
    });
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
        value={userPubkeyBase64}
        on:input={(e) => {
            isPubkeySet = false;
            userPubkeyBase64 = e.target.value;
        }}
        on:keydown={(e) => {
            if (e.key === "Enter") {
                setPubkey();
            }
        }}
        placeholder="wg genkey | tee priv.key | wg pubkey"
    />
</div>

<style src="./Content.css"></style>
