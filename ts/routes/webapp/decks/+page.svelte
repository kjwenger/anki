<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { api } from "$lib/webapp/api/client";
    import DeckTree from "$lib/webapp/components/DeckTree.svelte";
    import DeckDialog from "$lib/webapp/components/DeckDialog.svelte";
    import type { DeckNode } from "$lib/webapp/api/client";

    let decks: DeckNode[] = [];
    let loading = true;
    let error = "";
    let showCreateDialog = false;
    let showRenameDialog = false;
    let deleteConfirmation: { id: number; name: string } | null = null;
    let renameTarget: { id: number; name: string } | null = null;

    onMount(() => {
        loadDecks();
    });

    async function loadDecks() {
        loading = true;
        error = "";

        console.log("=== Loading Decks ===");

        try {
            console.log("Calling api.getDecks...");
            const response = await api.getDecks();
            console.log("Decks response:", response);
            console.log("Number of decks:", response.decks.length);
            decks = response.decks;
            console.log("Decks state updated:", decks);
        } catch (e: any) {
            console.error("Error loading decks:", e);
            error = e.message || "Failed to load decks";
        } finally {
            loading = false;
            console.log("Loading complete. Final decks:", decks);
        }
    }

    async function handleCreateDeck(event: CustomEvent) {
        const { name } = event.detail;

        console.log("=== Creating Deck ===");
        console.log("Deck name:", name);

        error = "";

        try {
            console.log("Calling api.createDeck...");
            const response = await api.createDeck(name);
            console.log("Deck created successfully:", response);
            showCreateDialog = false;
            await loadDecks();
            console.log("Deck creation complete!");
        } catch (e: any) {
            console.error("=== Error creating deck ===");
            console.error("Error object:", e);
            console.error("Error message:", e.message);
            console.error("Error status:", e.status);
            error = e.message || "Failed to create deck";
        }
    }

    function handleRenameRequest(event: CustomEvent) {
        console.log("=== Rename Request ===");
        const { id, name } = event.detail;
        console.log("Deck ID:", id);
        console.log("Current name:", name);
        renameTarget = { id, name };
        showRenameDialog = true;
        console.log("Rename dialog opened");
    }

    async function handleRenameDeck(event: CustomEvent) {
        console.log("=== Renaming Deck ===");
        if (!renameTarget) {
            console.log("No rename target!");
            return;
        }

        const { name } = event.detail;
        console.log("Rename target:", renameTarget);
        console.log("New name:", name);
        error = "";

        try {
            console.log("Calling api.renameDeck...");
            console.log("Deck ID:", renameTarget.id);
            console.log("New name:", name);
            await api.renameDeck(renameTarget.id, name);
            console.log("Rename successful!");
            showRenameDialog = false;
            renameTarget = null;
            console.log("Reloading decks...");
            await loadDecks();
            console.log("Rename complete!");
        } catch (e: any) {
            console.error("=== Error renaming deck ===");
            console.error("Error object:", e);
            console.error("Error message:", e.message);
            console.error("Error status:", e.status);
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
</script>

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md px-8 py-6">
        <div class="max-w-7xl mx-auto flex justify-between items-center">
            <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">
                Decks
            </h1>
            <div class="flex gap-4">
                <button
                    class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600"
                    on:click={goToDashboard}
                >
                    &larr; Dashboard
                </button>
                <button
                    class="px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={() => (showCreateDialog = true)}
                >
                    + New Deck
                </button>
            </div>
        </div>
    </header>

    <main class="max-w-7xl mx-auto p-8">
        {#if error}
            <div
                class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-4 mb-6"
                role="alert"
            >
                {error}
            </div>
        {/if}

        <div class="bg-gray-50 dark:bg-gray-800/50 rounded-lg p-6">
            <DeckTree
                {decks}
                {loading}
                on:study={handleStudy}
                on:rename={handleRenameRequest}
                on:delete={handleDeleteRequest}
            />
        </div>
    </main>
</div>

<DeckDialog bind:show={showCreateDialog} mode="create" on:create={handleCreateDeck} />

<DeckDialog
    bind:show={showRenameDialog}
    mode="rename"
    initialName={renameTarget?.name || ""}
    on:rename={handleRenameDeck}
/>

{#if deleteConfirmation}
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
        on:click={cancelDelete}
        role="presentation"
    >
        <div
            class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl p-8 max-w-md w-[90%]"
            on:click|stopPropagation
            on:keydown|stopPropagation
            role="dialog"
            aria-modal="true"
            tabindex="-1"
        >
            <h2 class="m-0 mb-4 text-xl text-gray-800 dark:text-gray-100 font-semibold">
                Delete Deck
            </h2>
            <p class="m-0 my-2 text-gray-500 dark:text-gray-400">
                Are you sure you want to delete "{deleteConfirmation.name}"?
            </p>
            <p class="text-red-600 dark:text-red-400 font-medium mt-4">
                This will delete all cards in this deck!
            </p>
            <div class="flex justify-end gap-4 mt-6">
                <button
                    class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600"
                    on:click={cancelDelete}
                >
                    Cancel
                </button>
                <button
                    class="px-6 py-3 bg-red-500 hover:bg-red-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={confirmDelete}
                >
                    Delete
                </button>
            </div>
        </div>
    </div>
{/if}
