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

<div class="collection-list">
    {#if loading}
        <div class="loading">Loading collections...</div>
    {:else if collections.length === 0}
        <div class="empty-state">
            <p>No collections found</p>
            <p class="hint">Create a new collection to get started</p>
        </div>
    {:else}
        <div class="collections">
            {#each collections as collection (collection.path)}
                <div
                    class="collection-item"
                    class:selected={currentCollection?.path === collection.path}
                    on:click={() => handleSelect(collection)}
                    on:keypress={(e) => e.key === "Enter" && handleSelect(collection)}
                    role="button"
                    tabindex="0"
                >
                    <div class="collection-info">
                        <div class="collection-name">{collection.name}</div>
                        <div class="collection-path">{collection.path}</div>
                    </div>
                    <div class="collection-actions">
                        {#if currentCollection?.path === collection.path}
                            <span class="current-badge">Current</span>
                        {/if}
                        <button
                            class="btn-delete"
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

<style>
    .collection-list {
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

    .collections {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .collection-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        background: white;
        border: 2px solid #eee;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .collection-item:hover {
        border-color: #667eea;
        box-shadow: 0 2px 8px rgba(102, 126, 234, 0.1);
    }

    .collection-item.selected {
        border-color: #667eea;
        background: #f8f9fe;
    }

    .collection-info {
        flex: 1;
        min-width: 0;
    }

    .collection-name {
        font-weight: 500;
        color: #333;
        margin-bottom: 0.25rem;
    }

    .collection-path {
        font-size: 0.85rem;
        color: #999;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .collection-actions {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .current-badge {
        background: #667eea;
        color: white;
        padding: 0.25rem 0.75rem;
        border-radius: 12px;
        font-size: 0.75rem;
        font-weight: 500;
    }

    .btn-delete {
        background: none;
        border: none;
        font-size: 1.25rem;
        cursor: pointer;
        padding: 0.25rem 0.5rem;
        opacity: 0.6;
        transition: opacity 0.2s;
    }

    .btn-delete:hover {
        opacity: 1;
    }
</style>
