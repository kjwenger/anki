<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { createEventDispatcher } from "svelte";
    import { api } from "../api/client";

    export let cardId: number;
    export let currentFlag: number = 0;

    const dispatch = createEventDispatcher<{
        action: { type: string; flag?: number };
    }>();

    let showDropdown = false;
    let showFlagSubmenu = false;

    function toggleDropdown() {
        showDropdown = !showDropdown;
        showFlagSubmenu = false;
    }

    function closeDropdown() {
        showDropdown = false;
        showFlagSubmenu = false;
    }

    async function handleFlag(flag: number) {
        try {
            await api.flagCard(cardId, flag);
            dispatch("action", { type: "flag", flag });
            closeDropdown();
        } catch (e: any) {
            console.error("Failed to flag card:", e);
        }
    }

    async function handleSuspend() {
        try {
            await api.suspendCard(cardId);
            dispatch("action", { type: "suspend" });
            closeDropdown();
        } catch (e: any) {
            console.error("Failed to suspend card:", e);
        }
    }

    async function handleBury() {
        try {
            await api.buryCard(cardId);
            dispatch("action", { type: "bury" });
            closeDropdown();
        } catch (e: any) {
            console.error("Failed to bury card:", e);
        }
    }

    // Flag colors matching Anki desktop
    const flagColors = [
        { id: 0, name: "No Flag", color: "gray" },
        { id: 1, name: "Red", color: "red" },
        { id: 2, name: "Orange", color: "orange" },
        { id: 3, name: "Green", color: "green" },
        { id: 4, name: "Blue", color: "blue" },
    ];

    function getFlagColor(flag: number): string {
        return flagColors.find((f) => f.id === flag)?.color || "gray";
    }

    // Close dropdown when clicking outside
    function handleOutsideClick(event: MouseEvent) {
        const target = event.target as HTMLElement;
        if (showDropdown && !target.closest(".card-actions-dropdown")) {
            closeDropdown();
        }
    }
</script>

<svelte:window on:click={handleOutsideClick} />

<div class="relative card-actions-dropdown">
    <button
        on:click={toggleDropdown}
        class="px-4 py-2 bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors flex items-center gap-2"
        title="Card actions"
    >
        <span>⋮</span>
        <span>More</span>
        {#if currentFlag > 0}
            <span
                class="w-3 h-3 rounded-full"
                style="background-color: {getFlagColor(currentFlag)}"
            ></span>
        {/if}
    </button>

    {#if showDropdown}
        <div
            class="absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg z-50"
        >
            <!-- Flag submenu -->
            {#if showFlagSubmenu}
                <div class="p-2">
                    <button
                        on:click={() => {
                            showFlagSubmenu = false;
                        }}
                        class="w-full text-left px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded mb-1"
                    >
                        ← Back
                    </button>
                    {#each flagColors.slice(1) as flag}
                        <button
                            on:click={() => handleFlag(flag.id)}
                            class="w-full text-left px-3 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 rounded flex items-center gap-2 text-gray-800 dark:text-gray-200"
                        >
                            <span
                                class="w-4 h-4 rounded-full"
                                style="background-color: {flag.color}"
                            ></span>
                            {flag.name}
                        </button>
                    {/each}
                </div>
            {:else}
                <!-- Main menu -->
                <div class="p-2">
                    <button
                        on:click={() => {
                            showFlagSubmenu = true;
                        }}
                        class="w-full text-left px-3 py-2 text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded flex items-center gap-2"
                    >
                        <span>🚩</span>
                        <span>Flag</span>
                        {#if currentFlag > 0}
                            <span
                                class="ml-auto w-3 h-3 rounded-full"
                                style="background-color: {getFlagColor(currentFlag)}"
                            ></span>
                        {/if}
                    </button>
                    <button
                        on:click={handleSuspend}
                        class="w-full text-left px-3 py-2 text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded flex items-center gap-2"
                    >
                        <span>⏸️</span>
                        <span>Suspend</span>
                    </button>
                    <button
                        on:click={handleBury}
                        class="w-full text-left px-3 py-2 text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded flex items-center gap-2"
                    >
                        <span>⚰️</span>
                        <span>Bury</span>
                    </button>
                </div>
            {/if}
        </div>
    {/if}
</div>
