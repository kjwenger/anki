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

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md px-8 py-6">
        <div class="max-w-7xl mx-auto flex justify-between items-center">
            <div class="flex items-center gap-4">
                <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">Decks</h1>
                {#if collection}
                    <span class="bg-indigo-500 text-white px-4 py-1.5 rounded-full text-sm font-medium">{collection.name}</span>
                {/if}
            </div>
            <div class="flex gap-4">
                <button class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600" on:click={goToDashboard}>
                    &larr; Dashboard
                </button>
                <button class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600" on:click={goToCollections}>
                    Switch Collection
                </button>
                <button class="px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200" on:click={() => (showCreateDialog = true)}>
                    + New Deck
                </button>
            </div>
        </div>
    </header>

    <main class="max-w-7xl mx-auto p-8">
        {#if error}
            <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-4 mb-6" role="alert">
                {error}
            </div>
        {/if}

        {#if !collection}
            <div class="bg-amber-50 dark:bg-amber-900/20 border border-amber-300 dark:border-amber-700 rounded-lg text-amber-800 dark:text-amber-300 p-4 mb-6" role="alert">
                <p class="m-0 mb-4">No collection selected. Please select a collection first.</p>
                <button class="px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200" on:click={goToCollections}>
                    Go to Collections
                </button>
            </div>
        {:else}
            <div class="bg-gray-50 dark:bg-gray-800/50 rounded-lg p-6">
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
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" on:click={cancelDelete} role="presentation">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl p-8 max-w-md w-[90%]" on:click|stopPropagation role="dialog" aria-modal="true">
            <h2 class="m-0 mb-4 text-xl text-gray-800 dark:text-gray-100 font-semibold">Delete Deck</h2>
            <p class="m-0 my-2 text-gray-500 dark:text-gray-400">Are you sure you want to delete "{deleteConfirmation.name}"?</p>
            <p class="text-red-600 dark:text-red-400 font-medium mt-4">This will delete all cards in this deck!</p>
            <div class="flex justify-end gap-4 mt-6">
                <button class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600" on:click={cancelDelete}>Cancel</button>
                <button class="px-6 py-3 bg-red-500 hover:bg-red-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200" on:click={confirmDelete}>Delete</button>
            </div>
        </div>
    </div>
{/if}
