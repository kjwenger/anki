<script lang="ts">
    import { goto } from "$app/navigation";
    import { api } from "$lib/webapp/api/client";

    let username = "";
    let email = "";
    let password = "";
    let confirmPassword = "";
    let error = "";
    let success = "";
    let loading = false;

    async function handleRegister() {
        error = "";
        success = "";

        // Validation
        if (!username || !email || !password || !confirmPassword) {
            error = "Please fill in all fields";
            return;
        }

        if (username.length < 3) {
            error = "Username must be at least 3 characters";
            return;
        }

        if (!email.includes("@")) {
            error = "Please enter a valid email address";
            return;
        }

        if (password.length < 6) {
            error = "Password must be at least 6 characters";
            return;
        }

        if (password !== confirmPassword) {
            error = "Passwords do not match";
            return;
        }

        loading = true;

        try {
            await api.register(username, email, password);
            success = "Registration successful! Redirecting to login...";
            setTimeout(() => {
                goto("/webapp/auth/login");
            }, 2000);
        } catch (e: any) {
            error = e.message || "Registration failed. Please try again.";
        } finally {
            loading = false;
        }
    }

    function handleKeyPress(event: KeyboardEvent) {
        if (event.key === "Enter") {
            handleRegister();
        }
    }
</script>

<div class="register-container">
    <div class="register-card">
        <h1>Create Account</h1>
        <p class="subtitle">Join Anki Web to start learning</p>

        {#if error}
            <div class="error-message" role="alert">
                {error}
            </div>
        {/if}

        {#if success}
            <div class="success-message" role="status">
                {success}
            </div>
        {/if}

        <form on:submit|preventDefault={handleRegister}>
            <div class="form-group">
                <label for="username">Username</label>
                <input
                    id="username"
                    type="text"
                    bind:value={username}
                    on:keypress={handleKeyPress}
                    placeholder="Choose a username"
                    disabled={loading}
                    autocomplete="username"
                />
            </div>

            <div class="form-group">
                <label for="email">Email</label>
                <input
                    id="email"
                    type="email"
                    bind:value={email}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your email"
                    disabled={loading}
                    autocomplete="email"
                />
            </div>

            <div class="form-group">
                <label for="password">Password</label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    on:keypress={handleKeyPress}
                    placeholder="Choose a password (min 6 characters)"
                    disabled={loading}
                    autocomplete="new-password"
                />
            </div>

            <div class="form-group">
                <label for="confirmPassword">Confirm Password</label>
                <input
                    id="confirmPassword"
                    type="password"
                    bind:value={confirmPassword}
                    on:keypress={handleKeyPress}
                    placeholder="Confirm your password"
                    disabled={loading}
                    autocomplete="new-password"
                />
            </div>

            <button type="submit" class="btn-primary" disabled={loading}>
                {#if loading}
                    Creating account...
                {:else}
                    Register
                {/if}
            </button>
        </form>

        <div class="login-link">
            Already have an account?
            <a href="/webapp/auth/login">Login here</a>
        </div>
    </div>
</div>

<style>
    .register-container {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        padding: 1rem;
    }

    .register-card {
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

    .success-message {
        background: #efe;
        border: 1px solid #cfc;
        border-radius: 4px;
        color: #3c3;
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

    .login-link {
        margin-top: 1.5rem;
        text-align: center;
        color: #666;
        font-size: 0.9rem;
    }

    .login-link a {
        color: #667eea;
        text-decoration: none;
        font-weight: 500;
    }

    .login-link a:hover {
        text-decoration: underline;
    }
</style>
