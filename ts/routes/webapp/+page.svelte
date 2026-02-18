<script lang="ts">
    import { currentUser } from "$lib/webapp/stores/auth";
    import { goto } from "$app/navigation";

    function handleProfile() {
        goto("/webapp/auth/profile");
    }

    function handleCollections() {
        goto("/webapp/collections");
    }

    function handleDecks() {
        goto("/webapp/decks");
    }

    function handleEditor() {
        goto("/webapp/editor");
    }

    function handleBrowse() {
        goto("/webapp/browse");
    }

    function handleStats() {
        goto("/webapp/stats");
    }

    function handleSettings() {
        goto("/webapp/settings");
    }
</script>

<div
    class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"
>
    <header
        class="bg-white dark:bg-gray-800 shadow-md px-8 py-4 flex justify-between items-center"
    >
        <h1 class="m-0 text-2xl text-gray-800 dark:text-gray-100 font-bold">
            Anki Web Dashboard
        </h1>
        <div class="flex items-center gap-4">
            {#if $currentUser}
                <span class="text-gray-500 dark:text-gray-400 text-sm">
                    Welcome, {$currentUser.username}!
                </span>
                <button
                    class="px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg cursor-pointer text-sm font-medium transition-colors duration-200"
                    on:click={handleProfile}
                >
                    Profile
                </button>
            {/if}
        </div>
    </header>

    <main class="max-w-7xl mx-auto p-8">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl cursor-pointer group"
            >
                <h2
                    class="m-0 mb-2 text-xl text-gray-800 dark:text-gray-100 font-semibold"
                >
                    Collections
                </h2>
                <p class="m-0 mb-4 text-gray-500 dark:text-gray-400 text-sm">
                    Manage your flashcard collections
                </p>
                <button
                    class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={handleCollections}
                >
                    View Collections
                </button>
            </div>

            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl cursor-pointer group"
            >
                <h2
                    class="m-0 mb-2 text-xl text-gray-800 dark:text-gray-100 font-semibold"
                >
                    Study
                </h2>
                <p class="m-0 mb-4 text-gray-500 dark:text-gray-400 text-sm">
                    Start your study session
                </p>
                <button
                    class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={handleDecks}
                >
                    Browse Decks
                </button>
            </div>

            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl cursor-pointer group"
            >
                <h2
                    class="m-0 mb-2 text-xl text-gray-800 dark:text-gray-100 font-semibold"
                >
                    Add Cards
                </h2>
                <p class="m-0 mb-4 text-gray-500 dark:text-gray-400 text-sm">
                    Create new flashcards
                </p>
                <button
                    class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={handleEditor}
                >
                    Add Cards
                </button>
            </div>

            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl cursor-pointer group"
            >
                <h2
                    class="m-0 mb-2 text-xl text-gray-800 dark:text-gray-100 font-semibold"
                >
                    Browse
                </h2>
                <p class="m-0 mb-4 text-gray-500 dark:text-gray-400 text-sm">
                    Search and manage cards
                </p>
                <button
                    class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={handleBrowse}
                >
                    Browse Cards
                </button>
            </div>

            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl cursor-pointer group"
            >
                <h2
                    class="m-0 mb-2 text-xl text-gray-800 dark:text-gray-100 font-semibold"
                >
                    Statistics
                </h2>
                <p class="m-0 mb-4 text-gray-500 dark:text-gray-400 text-sm">
                    View your learning progress
                </p>
                <button
                    class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={handleStats}
                >
                    View Stats
                </button>
            </div>

            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 transition-all duration-200 hover:-translate-y-1 hover:shadow-xl cursor-pointer group"
            >
                <h2
                    class="m-0 mb-2 text-xl text-gray-800 dark:text-gray-100 font-semibold"
                >
                    Settings
                </h2>
                <p class="m-0 mb-4 text-gray-500 dark:text-gray-400 text-sm">
                    Configure your preferences
                </p>
                <button
                    class="w-full py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={handleSettings}
                >
                    Settings
                </button>
            </div>
        </div>

        <div
            class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 border-l-4 border-indigo-500"
        >
            <h3 class="m-0 mb-4 text-gray-800 dark:text-gray-100 font-semibold">
                Welcome to Anki Web!
            </h3>
            <p class="m-0 my-2 text-gray-500 dark:text-gray-400 leading-relaxed">
                This is your central hub for managing flashcards, studying, and tracking
                your progress.
            </p>
            <p class="m-0 my-2 text-gray-500 dark:text-gray-400 leading-relaxed">
                Phase 3.1 (Authentication UI) is now complete! More features coming
                soon.
            </p>
        </div>
    </main>
</div>
