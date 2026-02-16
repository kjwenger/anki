<script lang="ts">
    import { goto } from "$app/navigation";
    import { currentUser, authStore } from "$lib/webapp/stores/auth";
    import { page } from "$app/stores";

    let showUserMenu = false;
    let showMobileMenu = false;

    function handleLogout() {
        authStore.logout();
        goto("/webapp/auth/login");
    }

    function toggleUserMenu() {
        showUserMenu = !showUserMenu;
    }

    function toggleMobileMenu() {
        showMobileMenu = !showMobileMenu;
    }

    function navigateTo(path: string) {
        goto(path);
        showMobileMenu = false;
        showUserMenu = false;
    }

    $: currentPath = $page.url.pathname;
    $: isActive = (path: string) => currentPath.startsWith(path);
</script>

<nav class="sticky top-0 z-50 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 shadow-xs">
    <div class="max-w-7xl mx-auto px-4 flex justify-between items-center h-16">
        <div class="flex items-center gap-4">
            <button
                class="flex items-center gap-2 bg-transparent border-none cursor-pointer text-xl font-semibold text-gray-800 dark:text-gray-100 hover:text-indigo-500 dark:hover:text-indigo-400 p-0"
                on:click={() => navigateTo("/webapp")}
            >
                <span class="text-2xl">🎴</span>
                <span>Anki Web</span>
            </button>
            <button
                class="md:hidden bg-transparent border-none text-2xl cursor-pointer p-2 text-gray-600 dark:text-gray-300"
                on:click={toggleMobileMenu}
            >
                {showMobileMenu ? "✕" : "☰"}
            </button>
        </div>

        <div
            class="items-center {showMobileMenu ? 'flex absolute top-16 left-0 right-0 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 flex-col gap-0 p-4' : 'hidden md:flex gap-8'}"
        >
            <div class="flex gap-1 {showMobileMenu ? 'flex-col w-full' : ''}">
                <button
                    class="bg-transparent border-none px-4 py-2 cursor-pointer text-sm font-medium rounded transition-all duration-200 {showMobileMenu ? 'w-full text-left' : ''} {isActive('/webapp/decks') ? 'bg-indigo-500 text-white' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-800 dark:hover:text-gray-200'}"
                    on:click={() => navigateTo("/webapp/decks")}
                >
                    Decks
                </button>
                <button
                    class="bg-transparent border-none px-4 py-2 cursor-pointer text-sm font-medium rounded transition-all duration-200 {showMobileMenu ? 'w-full text-left' : ''} {isActive('/webapp/editor') ? 'bg-indigo-500 text-white' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-800 dark:hover:text-gray-200'}"
                    on:click={() => navigateTo("/webapp/editor")}
                >
                    Add
                </button>
                <button
                    class="bg-transparent border-none px-4 py-2 cursor-pointer text-sm font-medium rounded transition-all duration-200 {showMobileMenu ? 'w-full text-left' : ''} {isActive('/webapp/browse') ? 'bg-indigo-500 text-white' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-800 dark:hover:text-gray-200'}"
                    on:click={() => navigateTo("/webapp/browse")}
                >
                    Browse
                </button>
                <button
                    class="bg-transparent border-none px-4 py-2 cursor-pointer text-sm font-medium rounded transition-all duration-200 {showMobileMenu ? 'w-full text-left' : ''} {isActive('/webapp/stats') ? 'bg-indigo-500 text-white' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 hover:text-gray-800 dark:hover:text-gray-200'}"
                    on:click={() => navigateTo("/webapp/stats")}
                >
                    Stats
                </button>
            </div>

            <div class="relative {showMobileMenu ? 'w-full mt-2' : ''}">
                {#if $currentUser}
                    <button
                        class="flex items-center gap-2 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded px-4 py-2 cursor-pointer text-sm transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-700 {showMobileMenu ? 'w-full justify-between' : ''}"
                        on:click={toggleUserMenu}
                    >
                        <span class="text-base">👤</span>
                        <span class="font-medium text-gray-800 dark:text-gray-200">{$currentUser.username}</span>
                        <span class="text-xs text-gray-400">{showUserMenu ? "▲" : "▼"}</span>
                    </button>

                    {#if showUserMenu}
                        <div
                            class="{showMobileMenu ? 'static mt-2 shadow-none border-0 bg-gray-50 dark:bg-gray-800' : 'absolute top-full mt-2 right-0 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded shadow-lg'} min-w-[180px] z-50"
                        >
                            <button
                                class="block w-full px-4 py-3 bg-transparent border-none text-left cursor-pointer text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                                on:click={() => navigateTo("/webapp/auth/profile")}
                            >
                                Profile
                            </button>
                            <button
                                class="block w-full px-4 py-3 bg-transparent border-none text-left cursor-pointer text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                                on:click={() => navigateTo("/webapp/settings")}
                            >
                                Settings
                            </button>
                            <button
                                class="block w-full px-4 py-3 bg-transparent border-none text-left cursor-pointer text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                                on:click={() => navigateTo("/webapp/collections")}
                            >
                                Collections
                            </button>
                            <div class="h-px bg-gray-200 dark:bg-gray-600 my-1"></div>
                            <button
                                class="block w-full px-4 py-3 bg-transparent border-none text-left cursor-pointer text-sm text-red-600 dark:text-red-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                                on:click={handleLogout}
                            >
                                Logout
                            </button>
                        </div>
                    {/if}
                {/if}
            </div>
        </div>
    </div>
</nav>
