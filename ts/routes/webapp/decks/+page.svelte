<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { api } from "$lib/webapp/api/client";
    import { currentCollection } from "$lib/webapp/stores/collection";
    import DeckTree from "$lib/webapp/components/DeckTree.svelte";
    import DeckDialog from "$lib/webapp/components/DeckDialog.svelte";

    interface Deck {
        id: number;
        name: string;
        new_count: number;
        learn_count: number;
        review_count: number;
    }

    let decks: Deck[] = [];
    let loading = true;
    let error = "";
    let showCreateDialog = false;
    let showRenameDialog = false;
    let deleteConfirmation: { id: number; name: string } | null = null;
    let renameTarget: { id: number; name: string } | null = null;

    $: collection = $currentCollection;

    onMount(() => {
        if (!collection) {
            goto("/webapp/collections");
            return;
        }
        loadDecks();
    });

    async function loadDecks() {
        loading = true;
        error = "";

        try {
            const response = await api.getDecks();
            decks = response.decks;
        } catch (e: any) {
            error = e.message || "Failed to load decks";
        } finally {
            loading = false;
        }
    }

    async function handleCreateDeck(event: CustomEvent) {
        const { name } = event.detail;
        error = "";

        try {
            await api.createDeck(name);
            showCreateDialog = false;
            await loadDecks();
        } catch (e: any) {
            error = e.message || "Failed to create deck";
        }
    }

    function handleRenameRequest(event: CustomEvent) {
        const { id, name } = event.detail;
        renameTarget = { id, name };
        showRenameDialog = true;
    }

    async function handleRenameDeck(event: CustomEvent) {
        if (!renameTarget) return;

        const { name } = event.detail;
        error = "";

        try {
            await api.renameDeck(renameTarget.id, name);
            showRenameDialog = false;
            renameTarget = null;
            await loadDecks();
        } catch (e: any) {
            error = e.message || "Failed to rename deck";
        }
    }

    function handleDeleteRequest(event: CustomEvent) {
        const { id, name } = event.detail;
        deleteConfirmation = { id, name };
    }

    async function confirmDelete() {
        if (!deleteConfirmation) return;

        error = "";

        try {
            await api.deleteDeck(deleteConfirmation.id);
            deleteConfirmation = null;
            await loadDecks();
        } catch (e: any) {
            error = e.message || "Failed to delete deck";
            deleteConfirmation = null;
        }
    }

    function cancelDelete() {
        deleteConfirmation = null;
    }

    function handleStudy(event: CustomEvent) {
        const { id } = event.detail;
        goto(`/webapp/review?deck=${id}`);
    }

    function goToDashboard() {
        goto("/webapp");
    }

    function goToCollections() {
        goto("/webapp/collections");
    }
</script>

<div class="decks-page">
    <header class="page-header">
        <div class="header-content">
            <div class="header-left">
                <h1>Decks</h1>
                {#if collection}
                    <span class="collection-badge">{collection.name}</span>
                {/if}
            </div>
            <div class="header-actions">
                <button class="btn-secondary" on:click={goToDashboard}>
                    ← Dashboard
                </button>
                <button class="btn-secondary" on:click={goToCollections}>
                    Switch Collection
                </button>
                <button
                    class="btn-primary"
                    on:click={() => (showCreateDialog = true)}
                >
                    + New Deck
                </button>
            </div>
        </div>
    </header>

    <main class="page-content">
        {#if error}
            <div class="error-banner" role="alert">
                {error}
            </div>
        {/if}

        {#if !collection}
            <div class="warning-banner" role="alert">
                <p>No collection selected. Please select a collection first.</p>
                <button class="btn-primary" on:click={goToCollections}>
                    Go to Collections
                </button>
            </div>
        {:else}
            <div class="decks-container">
                <DeckTree
                    {decks}
                    {loading}
                    on:study={handleStudy}
                    on:rename={handleRenameRequest}
                    on:delete={handleDeleteRequest}
                />
            </div>
        {/if}
    </main>
</div>

<DeckDialog
    bind:show={showCreateDialog}
    mode="create"
    on:create={handleCreateDeck}
/>

<DeckDialog
    bind:show={showRenameDialog}
    mode="rename"
    initialName={renameTarget?.name || ""}
    on:rename={handleRenameDeck}
/>

{#if deleteConfirmation}
    <div class="modal-overlay" on:click={cancelDelete} role="presentation">
        <div
            class="confirm-dialog"
            on:click|stopPropagation
            role="dialog"
            aria-modal="true"
        >
            <h2>Delete Deck</h2>
            <p>Are you sure you want to delete "{deleteConfirmation.name}"?</p>
            <p class="warning">This will delete all cards in this deck!</p>
            <div class="dialog-actions">
                <button class="btn-secondary" on:click={cancelDelete}
                    >Cancel</button
                >
                <button class="btn-danger" on:click={confirmDelete}
                    >Delete</button
                >
            </div>
        </div>
    </div>
{/if}

<style>
    .decks-page {
        min-height: 100vh;
        background: #f5f5f5;
    }

    .page-header {
        background: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        padding: 1.5rem 2rem;
    }

    .header-content {
        max-width: 1200px;
        margin: 0 auto;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .header-left {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    h1 {
        margin: 0;
        font-size: 1.75rem;
        color: #333;
    }

    .collection-badge {
        background: #667eea;
        color: white;
        padding: 0.375rem 0.875rem;
        border-radius: 16px;
        font-size: 0.85rem;
        font-weight: 500;
    }

    .header-actions {
        display: flex;
        gap: 1rem;
    }

    .page-content {
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem;
    }

    .error-banner,
    .warning-banner {
        background: #fee;
        border: 1px solid #fcc;
        border-radius: 8px;
        color: #c33;
        padding: 1rem;
        margin-bottom: 1.5rem;
    }

    .warning-banner {
        background: #fffbea;
        border-color: #ffd93d;
        color: #6b5d00;
    }

    .warning-banner p {
        margin: 0 0 1rem 0;
    }

    .decks-container {
        background: #f9f9f9;
        border-radius: 8px;
        padding: 1.5rem;
    }

    button {
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 4px;
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-primary {
        background: #667eea;
        color: white;
    }

    .btn-primary:hover {
        background: #5568d3;
    }

    .btn-secondary {
        background: #f0f0f0;
        color: #333;
    }

    .btn-secondary:hover {
        background: #e0e0e0;
    }

    .btn-danger {
        background: #dc3545;
        color: white;
    }

    .btn-danger:hover {
        background: #c82333;
    }

    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .confirm-dialog {
        background: white;
        border-radius: 8px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
        padding: 2rem;
        max-width: 400px;
        width: 90%;
    }

    .confirm-dialog h2 {
        margin: 0 0 1rem 0;
        color: #333;
        font-size: 1.25rem;
    }

    .confirm-dialog p {
        margin: 0.5rem 0;
        color: #666;
    }

    .confirm-dialog .warning {
        color: #c33;
        font-weight: 500;
        margin-top: 1rem;
    }

    .dialog-actions {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
        margin-top: 1.5rem;
    }
</style>
