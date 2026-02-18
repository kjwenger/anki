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
        console.log("=== Login Page Mount ===");
        const unsubscribe = authStore.subscribe((state) => {
            console.log("Login page - Auth state:", state);
            console.log("localStorage token:", localStorage.getItem("anki_auth_token"));
            console.log("localStorage user:", localStorage.getItem("anki_user"));

            if (state.isAuthenticated && state.token) {
                console.log("User already authenticated, redirecting to /webapp");
                goto("/webapp");
            } else {
                console.log("User not authenticated, showing login form");
            }
        });
        return unsubscribe;
    });

    async function handleLogin() {
        if (!username || !password) {
            error = "Please enter both username and password";
            return;
        }

        console.log("=== Login Attempt ===");
        console.log("Username:", username);
        console.log("Password length:", password.length);

        loading = true;
        error = "";

        try {
            console.log("Calling api.login...");
            const response = await api.login(username, password);
            console.log("Login API response:", response);
            console.log("Response user:", response.user);
            console.log("Response token:", response.token);

            console.log("Calling authStore.login...");
            authStore.login(response.user, response.token);

            console.log("Checking auth state after login...");
            let authState: any;
            const unsub = authStore.subscribe((state) => {
                authState = state;
            });
            unsub();
            console.log("Auth state after login:", authState);
            console.log(
                "localStorage token after login:",
                localStorage.getItem("anki_auth_token"),
            );
            console.log(
                "localStorage user after login:",
                localStorage.getItem("anki_user"),
            );

            console.log("Redirecting to /webapp");
            goto("/webapp");
        } catch (e: any) {
            console.error("=== Login Error ===");
            console.error("Error:", e);
            console.error("Error message:", e.message);
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

<div
    class="flex justify-center items-center min-h-screen bg-gradient-to-br from-indigo-500 to-purple-600 p-4"
>
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl p-8 w-full max-w-md">
        <h1
            class="m-0 mb-2 text-3xl text-gray-800 dark:text-gray-100 text-center font-bold"
        >
            Anki Web Login
        </h1>
        <p class="m-0 mb-8 text-gray-500 dark:text-gray-400 text-center text-sm">
            Sign in to your account
        </p>

        {#if error}
            <div
                class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-3 mb-4 text-sm"
                role="alert"
            >
                {error}
            </div>
        {/if}

        <form on:submit|preventDefault={handleLogin}>
            <div class="mb-6">
                <label
                    for="username"
                    class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm"
                >
                    Username
                </label>
                <input
                    id="username"
                    type="text"
                    bind:value={username}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your username"
                    disabled={loading}
                    autocomplete="username"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                />
            </div>

            <div class="mb-6">
                <label
                    for="password"
                    class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm"
                >
                    Password
                </label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your password"
                    disabled={loading}
                    autocomplete="current-password"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
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
            <a
                href="/webapp/auth/register"
                class="text-indigo-500 dark:text-indigo-400 no-underline font-medium hover:underline"
            >
                Register here
            </a>
        </div>
    </div>
</div>
