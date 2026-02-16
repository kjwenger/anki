<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { Collection } from "$lib/webapp/stores/collection";

    export let collections: Collection[] = [];
    export let currentCollection: Collection | null = null;
    export let loading = false;

    const dispatch = createEventDispatcher();

    function handleSelect(collection: Collection) {
        dispatch("select", collection);
    }

    function handleDelete(collection: Collection, event: Event) {
        event.stopPropagation();
        dispatch("delete", collection);
    }
</script>

<div class="w-full">
    {#if loading}
        <div class="py-8 text-center text-gray-500 dark:text-gray-400">Loading collections...</div>
    {:else if collections.length === 0}
        <div class="py-12 px-8 text-center text-gray-400 dark:text-gray-500">
            <p class="m-0 my-2">No collections found</p>
            <p class="m-0 my-2 text-sm text-gray-300 dark:text-gray-600">Create a new collection to get started</p>
        </div>
    {:else}
        <div class="flex flex-col gap-2">
            {#each collections as collection (collection.path)}
                <div
                    class="flex justify-between items-center p-4 border-2 rounded-lg cursor-pointer transition-all duration-200 hover:border-indigo-500 hover:shadow-md {currentCollection?.path === collection.path ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-900/20' : 'bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700'}"
                    on:click={() => handleSelect(collection)}
                    on:keypress={(e) => e.key === "Enter" && handleSelect(collection)}
                    role="button"
                    tabindex="0"
                >
                    <div class="flex-1 min-w-0">
                        <div class="font-medium text-gray-800 dark:text-gray-200 mb-1">{collection.name}</div>
                        <div class="text-sm text-gray-400 dark:text-gray-500 truncate">{collection.path}</div>
                    </div>
                    <div class="flex items-center gap-2">
                        {#if currentCollection?.path === collection.path}
                            <span class="bg-indigo-500 text-white px-3 py-1 rounded-full text-xs font-medium">Current</span>
                        {/if}
                        <button
                            class="bg-transparent border-none text-xl cursor-pointer p-1 px-2 opacity-60 hover:opacity-100 transition-opacity"
                            on:click={(e) => handleDelete(collection, e)}
                            aria-label="Delete collection"
                            title="Delete collection"
                        >
                            🗑️
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>
