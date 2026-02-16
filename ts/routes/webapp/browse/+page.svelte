<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { onMount } from "svelte";
    import { api } from "$lib/webapp/api/client";

    let searchQuery = "";
    let searchMode: "cards" | "notes" = "cards";
    let results: number[] = [];
    let loading = false;
    let error = "";
    let selectedIds: Set<number> = new Set();
    let cardDetails: Map<number, any> = new Map();

    onMount(() => {
        // Optionally load all cards initially
        handleSearch();
    });

    async function handleSearch() {
        loading = true;
        error = "";
        try {
            const query = searchQuery.trim() || ""; // Empty query returns all

            if (searchMode === "cards") {
                const response = await api.searchCards(query);
                results = response.card_ids;
                // Load card details for first batch (first 50)
                await loadCardDetails(results.slice(0, 50));
            } else {
                const response = await api.searchNotes(query);
                results = response.note_ids;
            }
        } catch (e: any) {
            error = e.message || "Search failed";
            results = [];
        } finally {
            loading = false;
        }
    }

    async function loadCardDetails(cardIds: number[]) {
        // Load card details in parallel
        const promises = cardIds.map(async (id) => {
            try {
                const card = await api.getCard(id);
                cardDetails.set(id, card);
                cardDetails = cardDetails; // Trigger reactivity
            } catch (e) {
                // Ignore errors for individual cards
            }
        });

        await Promise.all(promises);
    }

    function toggleSelect(id: number) {
        if (selectedIds.has(id)) {
            selectedIds.delete(id);
        } else {
            selectedIds.add(id);
        }
        selectedIds = selectedIds; // Trigger reactivity
    }

    function selectAll() {
        selectedIds = new Set(results);
    }

    function deselectAll() {
        selectedIds.clear();
        selectedIds = selectedIds;
    }

    async function handleBulkSuspend() {
        if (selectedIds.size === 0) return;

        try {
            const promises = Array.from(selectedIds).map((id) =>
                api.post(`/api/v1/cards/${id}/suspend`, {}),
            );
            await Promise.all(promises);
            alert(`Suspended ${selectedIds.size} cards`);
            deselectAll();
        } catch (e: any) {
            error = e.message || "Bulk suspend failed";
        }
    }

    async function handleBulkDelete() {
        if (selectedIds.size === 0) return;
        if (
            !confirm(
                `Are you sure you want to delete ${selectedIds.size} ${searchMode}?`,
            )
        )
            return;

        try {
            const promises = Array.from(selectedIds).map((id) =>
                searchMode === "cards"
                    ? api.delete(`/api/v1/cards/${id}`)
                    : api.delete(`/api/v1/notes/${id}`),
            );
            await Promise.all(promises);
            alert(`Deleted ${selectedIds.size} ${searchMode}`);
            deselectAll();
            handleSearch(); // Refresh
        } catch (e: any) {
            error = e.message || "Bulk delete failed";
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter" && !loading) {
            handleSearch();
        }
    }
</script>

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md px-8 py-6">
        <div class="max-w-[1400px] mx-auto flex justify-between items-center">
            <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">Browse {searchMode === "cards" ? "Cards" : "Notes"}</h1>
            <div class="flex gap-4">
                <a href="/webapp" class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 no-underline rounded-lg inline-block text-sm hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"> &larr; Back </a>
            </div>
        </div>
    </header>

    <main class="max-w-[1400px] mx-auto p-8">
        {#if error}
            <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-4 mb-6">{error}</div>
        {/if}

        <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md mb-6">
            <div class="flex gap-2 mb-4">
                <button
                    class="px-4 py-2 border-2 rounded-lg cursor-pointer transition-all duration-200 {searchMode === 'cards' ? 'bg-indigo-500 border-indigo-500 text-white' : 'bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 text-gray-800 dark:text-gray-200'}"
                    on:click={() => (searchMode = "cards")}
                >
                    Cards
                </button>
                <button
                    class="px-4 py-2 border-2 rounded-lg cursor-pointer transition-all duration-200 {searchMode === 'notes' ? 'bg-indigo-500 border-indigo-500 text-white' : 'bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 text-gray-800 dark:text-gray-200'}"
                    on:click={() => (searchMode = "notes")}
                >
                    Notes
                </button>
            </div>

            <div class="flex gap-2">
                <input
                    type="text"
                    bind:value={searchQuery}
                    on:keydown={handleKeydown}
                    placeholder="Search query (empty = all)"
                    class="flex-1 px-3 py-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg text-sm bg-white dark:bg-gray-700 dark:text-gray-100 focus:outline-hidden focus:border-indigo-500 transition-colors"
                />
                <button
                    class="px-8 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg cursor-pointer font-medium disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                    on:click={handleSearch}
                    disabled={loading}
                >
                    {loading ? "Searching..." : "Search"}
                </button>
            </div>
        </div>

        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden">
            <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center flex-wrap gap-4">
                <div class="font-semibold text-gray-800 dark:text-gray-200">
                    {results.length}
                    {searchMode} found
                    {#if selectedIds.size > 0}
                        ({selectedIds.size} selected)
                    {/if}
                </div>
                {#if results.length > 0}
                    <div class="flex gap-2 flex-wrap">
                        <button class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg cursor-pointer text-sm hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors" on:click={selectAll}>
                            Select All
                        </button>
                        <button class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg cursor-pointer text-sm hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors" on:click={deselectAll}>
                            Deselect All
                        </button>
                        {#if selectedIds.size > 0}
                            <button class="px-4 py-2 bg-orange-500 hover:bg-orange-600 text-white border-none rounded-lg cursor-pointer text-sm transition-colors" on:click={handleBulkSuspend}>
                                Suspend Selected
                            </button>
                            <button class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white border-none rounded-lg cursor-pointer text-sm transition-colors" on:click={handleBulkDelete}>
                                Delete Selected
                            </button>
                        {/if}
                    </div>
                {/if}
            </div>

            <div class="overflow-x-auto">
                {#if loading}
                    <div class="text-center py-12 text-gray-500 dark:text-gray-400">Searching...</div>
                {:else if results.length === 0}
                    <div class="text-center py-12 text-gray-500 dark:text-gray-400">No {searchMode} found</div>
                {:else}
                    <table class="w-full border-collapse">
                        <thead>
                            <tr>
                                <th class="w-10 text-center px-3 py-3 bg-gray-50 dark:bg-gray-700/50 font-semibold text-gray-500 dark:text-gray-400 text-xs uppercase border-b border-gray-200 dark:border-gray-700">
                                    <input
                                        type="checkbox"
                                        checked={selectedIds.size ===
                                            results.length}
                                        on:change={() => {
                                            if (
                                                selectedIds.size ===
                                                results.length
                                            ) {
                                                deselectAll();
                                            } else {
                                                selectAll();
                                            }
                                        }}
                                    />
                                </th>
                                <th class="px-3 py-3 text-left bg-gray-50 dark:bg-gray-700/50 font-semibold text-gray-500 dark:text-gray-400 text-xs uppercase border-b border-gray-200 dark:border-gray-700">ID</th>
                                {#if searchMode === "cards"}
                                    <th class="px-3 py-3 text-left bg-gray-50 dark:bg-gray-700/50 font-semibold text-gray-500 dark:text-gray-400 text-xs uppercase border-b border-gray-200 dark:border-gray-700">Deck</th>
                                    <th class="px-3 py-3 text-left bg-gray-50 dark:bg-gray-700/50 font-semibold text-gray-500 dark:text-gray-400 text-xs uppercase border-b border-gray-200 dark:border-gray-700">Due</th>
                                    <th class="px-3 py-3 text-left bg-gray-50 dark:bg-gray-700/50 font-semibold text-gray-500 dark:text-gray-400 text-xs uppercase border-b border-gray-200 dark:border-gray-700">Interval</th>
                                {/if}
                                <th class="px-3 py-3 text-left bg-gray-50 dark:bg-gray-700/50 font-semibold text-gray-500 dark:text-gray-400 text-xs uppercase border-b border-gray-200 dark:border-gray-700">Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each results.slice(0, 100) as id}
                                {@const card = cardDetails.get(id)}
                                <tr
                                    class="border-b border-gray-100 dark:border-gray-700/50 transition-colors {selectedIds.has(id) ? 'bg-blue-50 dark:bg-blue-900/20 hover:bg-blue-100 dark:hover:bg-blue-900/30' : 'hover:bg-gray-50 dark:hover:bg-gray-700/30'}"
                                >
                                    <td class="w-10 text-center px-3 py-3">
                                        <input
                                            type="checkbox"
                                            checked={selectedIds.has(id)}
                                            on:change={() => toggleSelect(id)}
                                        />
                                    </td>
                                    <td class="px-3 py-3 text-gray-800 dark:text-gray-200">{id}</td>
                                    {#if searchMode === "cards"}
                                        <td class="px-3 py-3 text-gray-800 dark:text-gray-200">{card?.deck_id || "Loading..."}</td>
                                        <td class="px-3 py-3 text-gray-800 dark:text-gray-200">{card?.due || "-"}</td>
                                        <td class="px-3 py-3 text-gray-800 dark:text-gray-200">{card?.interval || "-"}</td>
                                    {/if}
                                    <td class="px-3 py-3">
                                        <button
                                            class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded cursor-pointer text-xs hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                                            on:click={() =>
                                                alert(`View ${searchMode} ${id}`)}
                                        >
                                            View
                                        </button>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                    {#if results.length > 100}
                        <div class="py-4 text-center text-gray-500 dark:text-gray-400 text-sm border-t border-gray-200 dark:border-gray-700">
                            Showing first 100 of {results.length}
                            {searchMode}
                        </div>
                    {/if}
                {/if}
            </div>
        </div>
    </main>
</div>
