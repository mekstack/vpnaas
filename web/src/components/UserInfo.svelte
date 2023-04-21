<script lang="ts">
    export let username;

    function logout() {
        localStorage.removeItem("accessToken");
        location.reload();
    }
</script>

<div class="right">
    <h1>
        {username}<button class="logout-button" on:click={logout}>-></button>
    </h1>
</div>

<style>
    .right {
        display: flex;
        padding: 0em 2em;
        flex-direction: column;
    }

    .logout-button {
        background: none;
        border: none;
        color: limegreen;
        font-family: monospace;
        cursor: pointer;
    }

    .logout-button:hover {
        color: white;
    }
</style>
