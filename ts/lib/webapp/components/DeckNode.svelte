<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { goto } from "$app/navigation";
    import { api } from "../api/client";

    export let deck: {
        id: number;
        name: string;
        collapsed: boolean;
        new_count: number;
        learn_count: number;
        review_count: number;
        children: any[];
    };
    export let level: number = 0;

    const dispatch = createEventDispatcher();

    async function toggleCollapse() {
        try {
            // Optimistic update
            deck.collapsed = !deck.collapsed;
            await api.updateDeck(deck.id, { collapsed: deck.collapsed });
        } catch (e) {
            console.error("Failed to update deck collapse state:", e);
            // Revert on failure
            deck.collapsed = !deck.collapsed;
        }
    }

    function handleStudy() {
        goto(`/webapp/review/overview?deck=${deck.id}`);
    }

    function handleRename() {
        dispatch("rename", { id: deck.id, name: getDisplayName(deck.name) });
    }

    function handleDelete() {
        dispatch("delete", { id: deck.id, name: getDisplayName(deck.name) });
    }

    function getDisplayName(fullName: string): string {
        const parts = fullName.split("::");
        return parts[parts.length - 1];
    }

    $: totalDue = deck.new_count + deck.learn_count + deck.review_count;
    $: hasChildren = deck.children && deck.children.length > 0;
</script>

<div class="deck-row-container mb-2">
    <div
        class="flex justify-between items-center p-4 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg transition-all duration-200 hover:border-indigo-500 hover:shadow-md"
        style="margin-left: {level * 1.5}rem"
    >
        <div class="flex items-center gap-3 flex-1 min-w-0">
            {#if hasChildren}
                <button
                    on:click={toggleCollapse}
                    class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded transition-colors text-gray-500 dark:text-gray-400"
                    aria-label={deck.collapsed ? "Expand" : "Collapse"}
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="transition-transform duration-200 {deck.collapsed
                            ? '-rotate-90'
                            : ''}"
                    >
                        <polyline points="6 9 12 15 18 9"></polyline>
                    </svg>
                </button>
            {:else}
                <div class="w-6"></div>
            {/if}

            <div class="flex-1 min-w-0">
                <div class="font-medium text-gray-800 dark:text-gray-200 text-lg truncate">
                    {getDisplayName(deck.name)}
                </div>
                <div class="flex gap-3 mt-1">
                    <span
                        class="px-2 py-0.5 rounded-full text-xs font-bold bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400"
                        title="New cards"
                    >
                        {deck.new_count}
                    </span>
                    <span
                        class="px-2 py-0.5 rounded-full text-xs font-bold bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400"
                        title="Learning cards"
                    >
                        {deck.learn_count}
                    </span>
                    <span
                        class="px-2 py-0.5 rounded-full text-xs font-bold bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400"
                        title="Review cards"
                    >
                        {deck.review_count}
                    </span>
                </div>
            </div>
        </div>

        <div class="flex items-center gap-2 ml-4">
            {#if totalDue > 0}
                <button
                    class="px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 whitespace-nowrap"
                    on:click={handleStudy}
                >
                    Study ({totalDue})
                </button>
            {:else}
                <span
                    class="text-gray-400 dark:text-gray-500 text-xs italic whitespace-nowrap"
                >
                    No cards due
                </span>
            {/if}
            <button
                class="bg-transparent border-none text-base cursor-pointer p-1 px-2 opacity-60 hover:opacity-100 transition-opacity"
                on:click={handleRename}
                aria-label="Rename deck"
                title="Rename"
            >
                ✏️
            </button>
            <button
                class="bg-transparent border-none text-base cursor-pointer p-1 px-2 opacity-60 hover:opacity-100 transition-opacity"
                on:click={handleDelete}
                aria-label="Delete deck"
                title="Delete"
            >
                🗑️
            </button>
        </div>
    </div>

    {#if hasChildren && !deck.collapsed}
        <div class="mt-2">
            {#each deck.children as child (child.id)}
                <svelte:self
                    deck={child}
                    level={level + 1}
                    on:study
                    on:rename
                    on:delete
                />
            {/each}
        </div>
    {/if}
</div>
