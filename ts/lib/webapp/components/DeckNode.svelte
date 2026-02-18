<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let deck: {
        id: number;
        name: string;
        new_count: number;
        learn_count: number;
        review_count: number;
    };

    const dispatch = createEventDispatcher();

    function handleStudy() {
        dispatch("study", { id: deck.id });
    }

    function handleRename() {
        dispatch("rename", { id: deck.id, name: deck.name });
    }

    function handleDelete() {
        dispatch("delete", { id: deck.id, name: deck.name });
    }

    $: totalDue = deck.new_count + deck.learn_count + deck.review_count;
</script>

<div
    class="flex justify-between items-center p-4 bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 rounded-lg mb-2 transition-all duration-200 hover:border-indigo-500 hover:shadow-md"
>
    <div class="flex-1 min-w-0">
        <div class="font-medium text-gray-800 dark:text-gray-200 mb-2 text-lg">
            {deck.name}
        </div>
        <div class="flex gap-4">
            <span
                class="px-3 py-1 rounded-full text-sm font-medium bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400"
                title="New cards"
            >
                {deck.new_count}
            </span>
            <span
                class="px-3 py-1 rounded-full text-sm font-medium bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400"
                title="Learning cards"
            >
                {deck.learn_count}
            </span>
            <span
                class="px-3 py-1 rounded-full text-sm font-medium bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400"
                title="Review cards"
            >
                {deck.review_count}
            </span>
        </div>
    </div>

    <div class="flex items-center gap-2">
        {#if totalDue > 0}
            <button
                class="px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                on:click={handleStudy}
            >
                Study Now ({totalDue})
            </button>
        {:else}
            <span class="text-gray-400 dark:text-gray-500 text-sm italic">
                No cards due
            </span>
        {/if}
        <button
            class="bg-transparent border-none text-xl cursor-pointer p-1 px-2 opacity-60 hover:opacity-100 transition-opacity"
            on:click={handleRename}
            aria-label="Rename deck"
            title="Rename"
        >
            ✏️
        </button>
        <button
            class="bg-transparent border-none text-xl cursor-pointer p-1 px-2 opacity-60 hover:opacity-100 transition-opacity"
            on:click={handleDelete}
            aria-label="Delete deck"
            title="Delete"
        >
            🗑️
        </button>
    </div>
</div>
