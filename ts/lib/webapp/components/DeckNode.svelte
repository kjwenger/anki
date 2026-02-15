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

<div class="deck-node">
    <div class="deck-info">
        <div class="deck-name">{deck.name}</div>
        <div class="deck-counts">
            <span class="count new" title="New cards">{deck.new_count}</span>
            <span class="count learn" title="Learning cards"
                >{deck.learn_count}</span
            >
            <span class="count review" title="Review cards"
                >{deck.review_count}</span
            >
        </div>
    </div>

    <div class="deck-actions">
        {#if totalDue > 0}
            <button class="btn-study" on:click={handleStudy}>
                Study Now ({totalDue})
            </button>
        {:else}
            <span class="no-cards">No cards due</span>
        {/if}
        <button
            class="btn-icon"
            on:click={handleRename}
            aria-label="Rename deck"
            title="Rename"
        >
            ✏️
        </button>
        <button
            class="btn-icon"
            on:click={handleDelete}
            aria-label="Delete deck"
            title="Delete"
        >
            🗑️
        </button>
    </div>
</div>

<style>
    .deck-node {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        background: white;
        border: 2px solid #eee;
        border-radius: 8px;
        margin-bottom: 0.5rem;
        transition: all 0.2s;
    }

    .deck-node:hover {
        border-color: #667eea;
        box-shadow: 0 2px 8px rgba(102, 126, 234, 0.1);
    }

    .deck-info {
        flex: 1;
        min-width: 0;
    }

    .deck-name {
        font-weight: 500;
        color: #333;
        margin-bottom: 0.5rem;
        font-size: 1.1rem;
    }

    .deck-counts {
        display: flex;
        gap: 1rem;
    }

    .count {
        padding: 0.25rem 0.75rem;
        border-radius: 12px;
        font-size: 0.85rem;
        font-weight: 500;
    }

    .count.new {
        background: #e3f2fd;
        color: #1976d2;
    }

    .count.learn {
        background: #fff3e0;
        color: #f57c00;
    }

    .count.review {
        background: #e8f5e9;
        color: #388e3c;
    }

    .deck-actions {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .no-cards {
        color: #999;
        font-size: 0.9rem;
        font-style: italic;
    }

    .btn-study {
        background: #667eea;
        color: white;
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 4px;
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-study:hover {
        background: #5568d3;
    }

    .btn-icon {
        background: none;
        border: none;
        font-size: 1.25rem;
        cursor: pointer;
        padding: 0.25rem 0.5rem;
        opacity: 0.6;
        transition: opacity 0.2s;
    }

    .btn-icon:hover {
        opacity: 1;
    }
</style>
