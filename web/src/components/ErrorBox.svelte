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
        background-color: rgba(255, 0, 0, 0.9);
        color: white;
        padding: 1em;
        margin-bottom: 1em;
        border-radius: 4px;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
    }
</style>
