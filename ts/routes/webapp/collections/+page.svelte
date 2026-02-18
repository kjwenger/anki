<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { api } from "$lib/webapp/api/client";
    import { collectionStore, type Collection } from "$lib/webapp/stores/collection";
    import { authStore } from "$lib/webapp/stores/auth";
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
        // Debug: Check auth state on mount
        const auth = authStore;
        let authState: any;
        const unsub = auth.subscribe((state) => {
            authState = state;
        });
        unsub();

        console.log("=== Collections Page Mount ===");
        console.log("Auth state:", authState);
        console.log("localStorage token:", localStorage.getItem("anki_auth_token"));
        console.log("localStorage user:", localStorage.getItem("anki_user"));
        console.log("==============================");

        loadCollections();
        return unsubscribe;
    });

    async function loadCollections() {
        loading = true;
        error = "";

        console.log("=== Loading Collections ===");

        let authState: any;
        const unsub = authStore.subscribe((state) => {
            authState = state;
        });
        unsub();
        console.log("Auth state before API call:", authState);

        try {
            const response = await api.getCollections();
            console.log("Collections response:", response);
            collectionStore.setCollections(response.collections);
        } catch (e: any) {
            console.error("Error loading collections:", e);
            error = e.message || "Failed to load collections";
        } finally {
            loading = false;
        }
    }

    async function handleCreateCollection(event: CustomEvent) {
        const { name } = event.detail;

        console.log("=== Creating Collection ===");
        console.log("Collection name:", name);

        let authState: any;
        const unsub = authStore.subscribe((state) => {
            authState = state;
        });
        unsub();
        console.log("Auth state before create:", authState);
        console.log("localStorage token:", localStorage.getItem("anki_auth_token"));
        console.log("localStorage user:", localStorage.getItem("anki_user"));

        error = "";

        try {
            console.log("Calling api.createCollection...");
            const response = await api.createCollection(name);
            console.log("Collection created successfully:", response);

            const newCollection: Collection = {
                name,
                path: response.path,
            };

            console.log("New collection object:", newCollection);
            collectionStore.addCollection(newCollection);
            showCreateDialog = false;

            // Auto-select the newly created collection
            collectionStore.selectCollection(newCollection);
            console.log("Collection creation complete!");
        } catch (e: any) {
            console.error("=== Error creating collection ===");
            console.error("Error object:", e);
            console.error("Error message:", e.message);
            console.error("Error status:", e.status);
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

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md px-8 py-6">
        <div class="max-w-7xl mx-auto flex justify-between items-center">
            <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">
                Collections
            </h1>
            <div class="flex gap-4">
                <button
                    class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600"
                    on:click={goToDashboard}
                >
                    &larr; Back
                </button>
                <button
                    class="px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={() => (showCreateDialog = true)}
                >
                    + New Collection
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

        <div class="bg-gray-50 dark:bg-gray-800/50 rounded-lg p-6 mb-8">
            <CollectionList
                {collections}
                {currentCollection}
                {loading}
                on:select={handleSelectCollection}
                on:delete={handleDeleteRequest}
            />
        </div>

        {#if currentCollection}
            <div
                class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 border-l-4 border-indigo-500"
            >
                <h3 class="m-0 mb-4 text-gray-800 dark:text-gray-100 font-semibold">
                    Selected Collection
                </h3>
                <p class="m-0 my-2 text-gray-500 dark:text-gray-400">
                    <strong class="text-gray-800 dark:text-gray-200">Name:</strong>
                    {currentCollection.name}
                </p>
                <p class="m-0 my-2 text-gray-500 dark:text-gray-400">
                    <strong class="text-gray-800 dark:text-gray-200">Path:</strong>
                    {currentCollection.path}
                </p>
                <button
                    class="mt-4 px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200"
                    on:click={goToDashboard}
                >
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
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
        on:click={cancelDelete}
        role="presentation"
    >
        <div
            class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl p-8 max-w-md w-[90%]"
            on:click|stopPropagation
            role="dialog"
            aria-modal="true"
        >
            <h2 class="m-0 mb-4 text-xl text-gray-800 dark:text-gray-100 font-semibold">
                Delete Collection
            </h2>
            <p class="m-0 my-2 text-gray-500 dark:text-gray-400">
                Are you sure you want to delete "{deleteConfirmation.name}"?
            </p>
            <p class="text-red-600 dark:text-red-400 font-medium mt-4">
                This action cannot be undone!
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
