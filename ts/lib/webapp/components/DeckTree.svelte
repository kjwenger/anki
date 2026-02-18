<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import DeckNode from "./DeckNode.svelte";

    export let decks: Array<{
        id: number;
        name: string;
        new_count: number;
        learn_count: number;
        review_count: number;
    }> = [];
    export let loading = false;

    const dispatch = createEventDispatcher();

    function handleStudy(event: CustomEvent) {
        dispatch("study", event.detail);
    }

    function handleRename(event: CustomEvent) {
        dispatch("rename", event.detail);
    }

    function handleDelete(event: CustomEvent) {
        dispatch("delete", event.detail);
    }
</script>

<div class="w-full">
    {#if loading}
        <div class="py-8 text-center text-gray-500 dark:text-gray-400">
            Loading decks...
        </div>
    {:else if decks.length === 0}
        <div class="py-12 px-8 text-center text-gray-400 dark:text-gray-500">
            <p class="m-0 my-2">No decks found</p>
            <p class="m-0 my-2 text-sm text-gray-300 dark:text-gray-600">
                Create a new deck to get started
            </p>
        </div>
    {:else}
        <div class="flex flex-col">
            {#each decks as deck (deck.id)}
                <DeckNode
                    {deck}
                    on:study={handleStudy}
                    on:rename={handleRename}
                    on:delete={handleDelete}
                />
            {/each}
        </div>
    {/if}
</div>
