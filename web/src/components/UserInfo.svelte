<script lang="ts">
    export let username;

    function logout() {
        localStorage.removeItem("accessToken");
        location.reload();
    }
</script>

<div class="right">
    <span class="username">
        {username}<button class="logout-button" on:click={logout}>-></button>
    </span>
</div>

<style>
    .username {
        font-size: 1.5em;
    }

    .right {
        padding: 1em 2em;
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
