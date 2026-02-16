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

<div class="flex justify-center items-center min-h-screen bg-gray-100 dark:bg-gray-900 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 w-full max-w-lg">
        <h1 class="m-0 mb-6 text-3xl text-gray-800 dark:text-gray-100 font-bold">Profile</h1>

        {#if $currentUser}
            <div class="mb-6">
                <div class="flex justify-between py-3 border-b border-gray-200 dark:border-gray-700">
                    <span class="font-medium text-gray-500 dark:text-gray-400">Username:</span>
                    <span class="text-gray-800 dark:text-gray-200">{$currentUser.username}</span>
                </div>
                <div class="flex justify-between py-3 border-b border-gray-200 dark:border-gray-700">
                    <span class="font-medium text-gray-500 dark:text-gray-400">Email:</span>
                    <span class="text-gray-800 dark:text-gray-200">{$currentUser.email}</span>
                </div>
                <div class="flex justify-between py-3">
                    <span class="font-medium text-gray-500 dark:text-gray-400">User ID:</span>
                    <span class="text-gray-800 dark:text-gray-200">{$currentUser.id}</span>
                </div>
            </div>

            {#if message}
                <div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg text-green-600 dark:text-green-400 p-3 mb-4">{message}</div>
            {/if}

            {#if error}
                <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-3 mb-4">{error}</div>
            {/if}

            <div class="flex gap-4 mt-6">
                <button
                    class="flex-1 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-base font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600"
                    on:click={() => goto("/webapp")}
                >
                    ← Back
                </button>
                <button
                    class="flex-1 py-3 bg-red-500 hover:bg-red-600 disabled:bg-gray-400 text-white border-none rounded-lg text-base font-medium cursor-pointer transition-colors duration-200 disabled:cursor-not-allowed"
                    on:click={handleLogout}
                    disabled={loading}
                >
                    {loading ? "Logging out..." : "Logout"}
                </button>
            </div>
        {:else}
            <p class="text-gray-500 dark:text-gray-400">No user information available.</p>
            <button
                class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-base font-medium cursor-pointer transition-colors duration-200"
                on:click={() => goto("/webapp/auth/login")}
            >
                Login
            </button>
        {/if}
    </div>
</div>
