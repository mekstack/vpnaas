<div class="login-container">
    <a class="login-button" href="/login">Log In</a>
</div>

<style>
    .login-container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: calc(100% - var(--header-height));
    }

    .login-button {
        background: none;
        border: none;
        color: limegreen;
        font-family: monospace;
        font-size: 2em;
        cursor: pointer;
    }

    .login-button:hover {
        color: white;
    }
</style>
