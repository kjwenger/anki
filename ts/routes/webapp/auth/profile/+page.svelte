<script lang="ts">
    import { authStore, currentUser } from "$lib/webapp/stores/auth";
    import { goto } from "$app/navigation";
    import { api } from "$lib/webapp/api/client";

    let loading = false;
    let message = "";
    let error = "";

    async function handleLogout() {
        loading = true;
        error = "";

        try {
            await api.logout();
            authStore.logout();
            goto("/webapp/auth/login");
        } catch (e: any) {
            error = e.message || "Logout failed";
        } finally {
            loading = false;
        }
    }
</script>

<div class="profile-container">
    <div class="profile-card">
        <h1>Profile</h1>

        {#if $currentUser}
            <div class="user-info">
                <div class="info-row">
                    <span class="label">Username:</span>
                    <span class="value">{$currentUser.username}</span>
                </div>
                <div class="info-row">
                    <span class="label">Email:</span>
                    <span class="value">{$currentUser.email}</span>
                </div>
                <div class="info-row">
                    <span class="label">User ID:</span>
                    <span class="value">{$currentUser.id}</span>
                </div>
            </div>

            {#if message}
                <div class="success-message">{message}</div>
            {/if}

            {#if error}
                <div class="error-message">{error}</div>
            {/if}

            <div class="actions">
                <button
                    class="btn-secondary"
                    on:click={() => goto("/webapp")}
                >
                    Back to Dashboard
                </button>
                <button
                    class="btn-danger"
                    on:click={handleLogout}
                    disabled={loading}
                >
                    {loading ? "Logging out..." : "Logout"}
                </button>
            </div>
        {:else}
            <p>No user information available.</p>
            <button class="btn-primary" on:click={() => goto("/webapp/auth/login")}>
                Login
            </button>
        {/if}
    </div>
</div>

<style>
    .profile-container {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        background: #f5f5f5;
        padding: 1rem;
    }

    .profile-card {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        padding: 2rem;
        width: 100%;
        max-width: 500px;
    }

    h1 {
        margin: 0 0 1.5rem 0;
        font-size: 1.75rem;
        color: #333;
    }

    .user-info {
        margin-bottom: 1.5rem;
    }

    .info-row {
        display: flex;
        justify-content: space-between;
        padding: 0.75rem 0;
        border-bottom: 1px solid #eee;
    }

    .info-row:last-child {
        border-bottom: none;
    }

    .label {
        font-weight: 500;
        color: #666;
    }

    .value {
        color: #333;
    }

    .success-message {
        background: #efe;
        border: 1px solid #cfc;
        border-radius: 4px;
        color: #3c3;
        padding: 0.75rem;
        margin-bottom: 1rem;
    }

    .error-message {
        background: #fee;
        border: 1px solid #fcc;
        border-radius: 4px;
        color: #c33;
        padding: 0.75rem;
        margin-bottom: 1rem;
    }

    .actions {
        display: flex;
        gap: 1rem;
        margin-top: 1.5rem;
    }

    button {
        flex: 1;
        padding: 0.75rem;
        border: none;
        border-radius: 4px;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-primary {
        background: #667eea;
        color: white;
    }

    .btn-primary:hover {
        background: #5568d3;
    }

    .btn-secondary {
        background: #f0f0f0;
        color: #333;
    }

    .btn-secondary:hover {
        background: #e0e0e0;
    }

    .btn-danger {
        background: #dc3545;
        color: white;
    }

    .btn-danger:hover:not(:disabled) {
        background: #c82333;
    }

    button:disabled {
        background: #ccc;
        cursor: not-allowed;
    }
</style>
