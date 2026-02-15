<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { api } from "$lib/webapp/api/client";
    import { collectionStore, type Collection } from "$lib/webapp/stores/collection";
    import CollectionList from "$lib/webapp/components/CollectionList.svelte";
    import CreateCollectionDialog from "$lib/webapp/components/CreateCollectionDialog.svelte";

    let collections: Collection[] = [];
    let currentCollection: Collection | null = null;
    let loading = true;
    let error = "";
    let showCreateDialog = false;
    let deleteConfirmation: Collection | null = null;

    // Subscribe to collection store
    const unsubscribe = collectionStore.subscribe((state) => {
        collections = state.collections;
        currentCollection = state.currentCollection;
    });

    onMount(() => {
        loadCollections();
        return unsubscribe;
    });

    async function loadCollections() {
        loading = true;
        error = "";

        try {
            const response = await api.getCollections();
            collectionStore.setCollections(response.collections);
        } catch (e: any) {
            error = e.message || "Failed to load collections";
        } finally {
            loading = false;
        }
    }

    async function handleCreateCollection(event: CustomEvent) {
        const { name } = event.detail;
        error = "";

        try {
            const response = await api.createCollection(name);
            const newCollection: Collection = {
                name,
                path: response.path,
            };
            collectionStore.addCollection(newCollection);
            showCreateDialog = false;
            
            // Auto-select the newly created collection
            collectionStore.selectCollection(newCollection);
        } catch (e: any) {
            error = e.message || "Failed to create collection";
        }
    }

    function handleSelectCollection(event: CustomEvent) {
        const collection: Collection = event.detail;
        collectionStore.selectCollection(collection);
    }

    function handleDeleteRequest(event: CustomEvent) {
        const collection: Collection = event.detail;
        deleteConfirmation = collection;
    }

    async function confirmDelete() {
        if (!deleteConfirmation) return;

        error = "";

        try {
            await api.deleteCollection(deleteConfirmation.path);
            collectionStore.removeCollection(deleteConfirmation.path);
            deleteConfirmation = null;
        } catch (e: any) {
            error = e.message || "Failed to delete collection";
            deleteConfirmation = null;
        }
    }

    function cancelDelete() {
        deleteConfirmation = null;
    }

    function goToDashboard() {
        goto("/webapp");
    }
</script>

<div class="collections-page">
    <header class="page-header">
        <div class="header-content">
            <h1>Collections</h1>
            <div class="header-actions">
                <button class="btn-secondary" on:click={goToDashboard}>
                    ← Back to Dashboard
                </button>
                <button class="btn-primary" on:click={() => (showCreateDialog = true)}>
                    + New Collection
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

        <div class="collections-container">
            <CollectionList
                {collections}
                {currentCollection}
                {loading}
                on:select={handleSelectCollection}
                on:delete={handleDeleteRequest}
            />
        </div>

        {#if currentCollection}
            <div class="current-collection-info">
                <h3>Selected Collection</h3>
                <p><strong>Name:</strong> {currentCollection.name}</p>
                <p><strong>Path:</strong> {currentCollection.path}</p>
                <button class="btn-primary" on:click={goToDashboard}>
                    Continue to Dashboard
                </button>
            </div>
        {/if}
    </main>
</div>

<CreateCollectionDialog
    bind:show={showCreateDialog}
    on:create={handleCreateCollection}
/>

{#if deleteConfirmation}
    <div class="modal-overlay" on:click={cancelDelete} role="presentation">
        <div class="confirm-dialog" on:click|stopPropagation role="dialog" aria-modal="true">
            <h2>Delete Collection</h2>
            <p>Are you sure you want to delete "{deleteConfirmation.name}"?</p>
            <p class="warning">This action cannot be undone!</p>
            <div class="dialog-actions">
                <button class="btn-secondary" on:click={cancelDelete}>Cancel</button>
                <button class="btn-danger" on:click={confirmDelete}>Delete</button>
            </div>
        </div>
    </div>
{/if}

<style>
    .collections-page {
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

    h1 {
        margin: 0;
        font-size: 1.75rem;
        color: #333;
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

    .error-banner {
        background: #fee;
        border: 1px solid #fcc;
        border-radius: 8px;
        color: #c33;
        padding: 1rem;
        margin-bottom: 1.5rem;
    }

    .collections-container {
        background: #f9f9f9;
        border-radius: 8px;
        padding: 1.5rem;
        margin-bottom: 2rem;
    }

    .current-collection-info {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        padding: 1.5rem;
        border-left: 4px solid #667eea;
    }

    .current-collection-info h3 {
        margin: 0 0 1rem 0;
        color: #333;
    }

    .current-collection-info p {
        margin: 0.5rem 0;
        color: #666;
    }

    .current-collection-info strong {
        color: #333;
    }

    .current-collection-info .btn-primary {
        margin-top: 1rem;
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
