<script lang="ts">
    export let message: string;
    export let duration: number = 5000;
    let visible = true;

    setTimeout(() => {
        visible = false;
    }, duration);
</script>

{#if visible}
    <div class="error-box">
        <p>{message}</p>
    </div>
{/if}

<style>
    .error-box {
        background-color: black;
        color: #ff0033;
        padding: 0 1em;
        margin-bottom: 1em;
        border: 1px solid rgb(48, 52, 54);
        font-family: monospace;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
    }
</style>
