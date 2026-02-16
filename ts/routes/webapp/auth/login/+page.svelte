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

<div class="flex justify-center items-center min-h-screen bg-gradient-to-br from-indigo-500 to-purple-600 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl p-8 w-full max-w-md">
        <h1 class="m-0 mb-2 text-3xl text-gray-800 dark:text-gray-100 text-center font-bold">Anki Web Login</h1>
        <p class="m-0 mb-8 text-gray-500 dark:text-gray-400 text-center text-sm">Sign in to your account</p>

        {#if error}
            <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-3 mb-4 text-sm" role="alert">
                {error}
            </div>
        {/if}

        <form on:submit|preventDefault={handleLogin}>
            <div class="mb-6">
                <label for="username" class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm">Username</label>
                <input
                    id="username"
                    type="text"
                    bind:value={username}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your username"
                    disabled={loading}
                    autocomplete="username"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-none focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                />
            </div>

            <div class="mb-6">
                <label for="password" class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm">Password</label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your password"
                    disabled={loading}
                    autocomplete="current-password"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-none focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                />
            </div>

            <button
                type="submit"
                class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 disabled:bg-gray-400 text-white border-none rounded-lg text-base font-medium cursor-pointer transition-colors duration-200 disabled:cursor-not-allowed"
                disabled={loading}
            >
                {#if loading}
                    Logging in...
                {:else}
                    Login
                {/if}
            </button>
        </form>

        <div class="mt-6 text-center text-gray-500 dark:text-gray-400 text-sm">
            Don't have an account?
            <a href="/webapp/auth/register" class="text-indigo-500 dark:text-indigo-400 no-underline font-medium hover:underline">Register here</a>
        </div>
    </div>
</div>
