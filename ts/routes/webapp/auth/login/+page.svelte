<script lang="ts">
    import { goto } from "$app/navigation";
    import { authStore } from "$lib/webapp/stores/auth";
    import { api } from "$lib/webapp/api/client";
    import { onMount } from "svelte";

    let username = "";
    let password = "";
    let error = "";
    let loading = false;

    // Redirect if already authenticated
    onMount(() => {
        const unsubscribe = authStore.subscribe((state) => {
            if (state.isAuthenticated) {
                goto("/webapp");
            }
        });
        return unsubscribe;
    });

    async function handleLogin() {
        if (!username || !password) {
            error = "Please enter both username and password";
            return;
        }

        loading = true;
        error = "";

        try {
            const response = await api.login(username, password);
            authStore.login(response.user, response.token);
            goto("/webapp");
        } catch (e: any) {
            error = e.message || "Login failed. Please try again.";
        } finally {
            loading = false;
        }
    }

    function handleKeyPress(event: KeyboardEvent) {
        if (event.key === "Enter") {
            handleLogin();
        }
    }
</script>

<div class="login-container">
    <div class="login-card">
        <h1>Anki Web Login</h1>
        <p class="subtitle">Sign in to your account</p>

        {#if error}
            <div class="error-message" role="alert">
                {error}
            </div>
        {/if}

        <form on:submit|preventDefault={handleLogin}>
            <div class="form-group">
                <label for="username">Username</label>
                <input
                    id="username"
                    type="text"
                    bind:value={username}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your username"
                    disabled={loading}
                    autocomplete="username"
                />
            </div>

            <div class="form-group">
                <label for="password">Password</label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your password"
                    disabled={loading}
                    autocomplete="current-password"
                />
            </div>

            <button type="submit" class="btn-primary" disabled={loading}>
                {#if loading}
                    Logging in...
                {:else}
                    Login
                {/if}
            </button>
        </form>

        <div class="register-link">
            Don't have an account?
            <a href="/webapp/auth/register">Register here</a>
        </div>
    </div>
</div>

<style>
    .login-container {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        padding: 1rem;
    }

    .login-card {
        background: white;
        border-radius: 8px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
        padding: 2rem;
        width: 100%;
        max-width: 400px;
    }

    h1 {
        margin: 0 0 0.5rem 0;
        font-size: 1.75rem;
        color: #333;
        text-align: center;
    }

    .subtitle {
        margin: 0 0 2rem 0;
        color: #666;
        text-align: center;
        font-size: 0.9rem;
    }

    .error-message {
        background: #fee;
        border: 1px solid #fcc;
        border-radius: 4px;
        color: #c33;
        padding: 0.75rem;
        margin-bottom: 1rem;
        font-size: 0.9rem;
    }

    .form-group {
        margin-bottom: 1.5rem;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
        color: #333;
        font-weight: 500;
        font-size: 0.9rem;
    }

    input {
        width: 100%;
        padding: 0.75rem;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 1rem;
        transition: border-color 0.2s;
        box-sizing: border-box;
    }

    input:focus {
        outline: none;
        border-color: #667eea;
    }

    input:disabled {
        background: #f5f5f5;
        cursor: not-allowed;
    }

    .btn-primary {
        width: 100%;
        padding: 0.75rem;
        background: #667eea;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-primary:hover:not(:disabled) {
        background: #5568d3;
    }

    .btn-primary:disabled {
        background: #ccc;
        cursor: not-allowed;
    }

    .register-link {
        margin-top: 1.5rem;
        text-align: center;
        color: #666;
        font-size: 0.9rem;
    }

    .register-link a {
        color: #667eea;
        text-decoration: none;
        font-weight: 500;
    }

    .register-link a:hover {
        text-decoration: underline;
    }
</style>
