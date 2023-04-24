<script lang="ts">
    export let userPubkey: string;
    export let username: string;
    export let accessToken: string;
    export let fetchUserConfig: () => Promise<void>;

    import { addError } from "../stores/errorStore";
    import { KeysClient } from "../grpc/VpnaasServiceClientPb";
    import { User, UserPubkey, Pubkey } from "../grpc/vpnaas_pb";

    let setPubkeyText = "";
    let pubkeyBoxLabel = "";
    let pubkeyIsNew = !!userPubkey;

    $: {
        if (pubkeyIsNew) {
            setPubkeyText = "set";
            pubkeyBoxLabel = "Enter your public key:";
        } else {
            setPubkeyText = "set!";
            pubkeyBoxLabel = "Your public key:";
        }
    }

    function isValidBase64(str) {
        try {
            window.atob(str);
            return true;
        } catch (e) {
            return false;
        }
    }

    async function setPubkey() {
        if (!isValidBase64(userPubkey)) {
            addError(
                "Invalid base64 string"
            );
            return;
        }

        // Decode the base64-encoded string and convert it to a Uint8Array
        const decodedKey = window.atob(userPubkey);
        const keyBytes = new Uint8Array(decodedKey.length);
        for (let i = 0; i < decodedKey.length; i++) {
            keyBytes[i] = decodedKey.charCodeAt(i);
        }

        const setPubkeyReq = new KeysClient("http://127.0.0.1:3000").set_pubkey(
            new UserPubkey()
                .setUser(new User().setUsername(username))
                .setPubkey(new Pubkey().setBytes(keyBytes)),
            { Authorization: `Bearer ${accessToken}` },
            (err, success) => {
                if (err) {
                    console.error(err);
                    addError("Error setting public key: " + err.message);
                    return;
                }
                setPubkeyText = "set!";
                pubkeyBoxLabel = "Your public key:";
                pubkeyIsNew = false;
                fetchUserConfig();
            }
        );
    }
</script>

<div class="pubkey-container">
    <label class="pubkey-label">{pubkeyBoxLabel}</label>
    <div class="set-container">
        <button class="set-button" on:click={setPubkey}>{setPubkeyText}</button>
    </div>
    <input
        type="text"
        class="pubkey-box"
        value={userPubkey}
        on:input={(e) => {
            pubkeyIsNew = true;
            userPubkey = e.target.value;
        }}
        on:keydown={(e) => {
            if (e.key === "Enter") {
                setPubkey();
            }
        }}
        placeholder="Enter your base64 encoded public key"
    />
</div>

<style>
    ::selection {
        background: lightgray;
        color: black;
    }

    .pubkey-box {
        display: block;
        width: 100%;
        padding: 0.5em 1em;
        border: 1px solid rgb(48, 52, 54);
        font-family: monospace;
        height: 100%;
        color: lightgray;
        background-color: black;
        resize: none;
        border-radius: 0;
    }

    .pubkey-container {
        margin: 2em 0;
        position: relative;
    }

    .pubkey-label {
        display: block;
        font-weight: bold;
        font-size: 1.5em;
        margin: 0.3em 0;
    }

    .set-container {
        position: absolute;
        top: 0;
        right: 0;
    }

    .set-button {
        background: none;
        border: none;
        color: limegreen;
        font-family: monospace;
        cursor: pointer;
    }

    .set-button:hover {
        color: white;
    }

    .set-button:active {
        color: black;
        background-color: limegreen;
        border-radius: 0;
    }
</style>
