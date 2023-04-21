<script lang="ts">
    export let userPubkey: string;
    export let pubkeyBoxLabel: string;
    export let username: string;
    export let accessToken: string;

    import { KeysClient } from "../grpc/VpnaasServiceClientPb";
    import { User, UserPubkey, Pubkey } from "../grpc/vpnaas_pb";

    async function addPubkey() {
        const setPubkeyReq = new KeysClient("http://127.0.0.1:3000").set_pubkey(
            new UserPubkey()
                .setUser(new User().setUsername(username))
                .setPubkey(
                    new Pubkey().setBytes(new TextEncoder().encode(userPubkey))
                ),
            { Authorization: `Bearer ${accessToken}` },
            (err, success) => {
                if (err) {
                    console.error(err);
                    return;
                }
                console.log("Public key added successfully.");
            }
        );
    }
</script>

<div class="pubkey-container">
    <label class="pubkey-label">{pubkeyBoxLabel}</label>
    <input class="pubkey-box" type="text" readonly value={userPubkey} />
</div>

<style>
    .pubkey-container {
        margin: 2em 0;
    }

    .pubkey-label {
        display: block;
        font-weight: bold;
        font-size: 1.5em;
        margin: 0.3em 0;
    }

    .pubkey-box {
        display: block;
        width: 100%;
        padding: 0.5em 1em;
        border: 1px solid rgb(48, 52, 54);
        font-family: monospace;
        color: lightgray;
        background-color: black;
        resize: none;
        border-radius: 0;
    }
</style>
