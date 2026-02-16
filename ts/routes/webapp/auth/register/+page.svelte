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

<div class="flex justify-center items-center min-h-screen bg-gradient-to-br from-indigo-500 to-purple-600 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl p-8 w-full max-w-md">
        <h1 class="m-0 mb-2 text-3xl text-gray-800 dark:text-gray-100 text-center font-bold">Create Account</h1>
        <p class="m-0 mb-8 text-gray-500 dark:text-gray-400 text-center text-sm">Join Anki Web to start learning</p>

        {#if error}
            <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-3 mb-4 text-sm" role="alert">
                {error}
            </div>
        {/if}

        {#if success}
            <div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg text-green-600 dark:text-green-400 p-3 mb-4 text-sm" role="status">
                {success}
            </div>
        {/if}

        <form on:submit|preventDefault={handleRegister}>
            <div class="mb-6">
                <label for="username" class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm">Username</label>
                <input
                    id="username"
                    type="text"
                    bind:value={username}
                    on:keypress={handleKeyPress}
                    placeholder="Choose a username"
                    disabled={loading}
                    autocomplete="username"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                />
            </div>

            <div class="mb-6">
                <label for="email" class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm">Email</label>
                <input
                    id="email"
                    type="email"
                    bind:value={email}
                    on:keypress={handleKeyPress}
                    placeholder="Enter your email"
                    disabled={loading}
                    autocomplete="email"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                />
            </div>

            <div class="mb-6">
                <label for="password" class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm">Password</label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    on:keypress={handleKeyPress}
                    placeholder="Choose a password (min 6 characters)"
                    disabled={loading}
                    autocomplete="new-password"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                />
            </div>

            <div class="mb-6">
                <label for="confirmPassword" class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm">Confirm Password</label>
                <input
                    id="confirmPassword"
                    type="password"
                    bind:value={confirmPassword}
                    on:keypress={handleKeyPress}
                    placeholder="Confirm your password"
                    disabled={loading}
                    autocomplete="new-password"
                    class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                />
            </div>

            <button
                type="submit"
                class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 disabled:bg-gray-400 text-white border-none rounded-lg text-base font-medium cursor-pointer transition-colors duration-200 disabled:cursor-not-allowed"
                disabled={loading}
            >
                {#if loading}
                    Creating account...
                {:else}
                    Register
                {/if}
            </button>
        </form>

        <div class="mt-6 text-center text-gray-500 dark:text-gray-400 text-sm">
            Already have an account?
            <a href="/webapp/auth/login" class="text-indigo-500 dark:text-indigo-400 no-underline font-medium hover:underline">Login here</a>
        </div>
    </div>
</div>
