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

<div class="deck-tree">
    {#if loading}
        <div class="loading">Loading decks...</div>
    {:else if decks.length === 0}
        <div class="empty-state">
            <p>No decks found</p>
            <p class="hint">Create a new deck to get started</p>
        </div>
    {:else}
        <div class="decks">
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

<style>
    .deck-tree {
        width: 100%;
    }

    .loading {
        padding: 2rem;
        text-align: center;
        color: #666;
    }

    .empty-state {
        padding: 3rem 2rem;
        text-align: center;
        color: #999;
    }

    .empty-state p {
        margin: 0.5rem 0;
    }

    .empty-state .hint {
        font-size: 0.9rem;
        color: #bbb;
    }

    .decks {
        display: flex;
        flex-direction: column;
    }
</style>
