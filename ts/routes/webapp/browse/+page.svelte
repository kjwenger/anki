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

<div class="browse-page">
    <header class="page-header">
        <div class="header-content">
            <h1>Browse {searchMode === "cards" ? "Cards" : "Notes"}</h1>
            <div class="header-actions">
                <a href="/webapp/decks" class="btn-secondary"> ← Back </a>
            </div>
        </div>
    </header>

    <main class="page-content">
        {#if error}
            <div class="error-banner">{error}</div>
        {/if}

        <div class="search-bar">
            <div class="mode-selector">
                <button
                    class="mode-btn"
                    class:active={searchMode === "cards"}
                    on:click={() => (searchMode = "cards")}
                >
                    Cards
                </button>
                <button
                    class="mode-btn"
                    class:active={searchMode === "notes"}
                    on:click={() => (searchMode = "notes")}
                >
                    Notes
                </button>
            </div>

            <div class="search-input-container">
                <input
                    type="text"
                    bind:value={searchQuery}
                    on:keydown={handleKeydown}
                    placeholder="Search query (empty = all)"
                    class="search-input"
                />
                <button
                    class="btn-search"
                    on:click={handleSearch}
                    disabled={loading}
                >
                    {loading ? "Searching..." : "Search"}
                </button>
            </div>
        </div>

        <div class="results-section">
            <div class="results-header">
                <div class="results-info">
                    {results.length}
                    {searchMode} found
                    {#if selectedIds.size > 0}
                        ({selectedIds.size} selected)
                    {/if}
                </div>
                {#if results.length > 0}
                    <div class="bulk-actions">
                        <button class="btn-small" on:click={selectAll}>
                            Select All
                        </button>
                        <button class="btn-small" on:click={deselectAll}>
                            Deselect All
                        </button>
                        {#if selectedIds.size > 0}
                            <button
                                class="btn-small btn-warning"
                                on:click={handleBulkSuspend}
                            >
                                Suspend Selected
                            </button>
                            <button
                                class="btn-small btn-danger"
                                on:click={handleBulkDelete}
                            >
                                Delete Selected
                            </button>
                        {/if}
                    </div>
                {/if}
            </div>

            <div class="results-table">
                {#if loading}
                    <div class="loading">Searching...</div>
                {:else if results.length === 0}
                    <div class="no-results">No {searchMode} found</div>
                {:else}
                    <table>
                        <thead>
                            <tr>
                                <th class="col-checkbox">
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
                                <th>ID</th>
                                {#if searchMode === "cards"}
                                    <th>Deck</th>
                                    <th>Due</th>
                                    <th>Interval</th>
                                {/if}
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each results.slice(0, 100) as id}
                                {@const card = cardDetails.get(id)}
                                <tr class:selected={selectedIds.has(id)}>
                                    <td class="col-checkbox">
                                        <input
                                            type="checkbox"
                                            checked={selectedIds.has(id)}
                                            on:change={() => toggleSelect(id)}
                                        />
                                    </td>
                                    <td>{id}</td>
                                    {#if searchMode === "cards"}
                                        <td
                                            >{card?.deck_id ||
                                                "Loading..."}</td
                                        >
                                        <td>{card?.due || "-"}</td>
                                        <td>{card?.interval || "-"}</td>
                                    {/if}
                                    <td>
                                        <button
                                            class="btn-tiny"
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
                        <div class="pagination-note">
                            Showing first 100 of {results.length}
                            {searchMode}
                        </div>
                    {/if}
                {/if}
            </div>
        </div>
    </main>
</div>

<style>
    .browse-page {
        min-height: 100vh;
        background: #f5f5f5;
    }

    .page-header {
        background: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        padding: 1.5rem 2rem;
    }

    .header-content {
        max-width: 1400px;
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

    .page-content {
        max-width: 1400px;
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

    .search-bar {
        background: white;
        padding: 1.5rem;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        margin-bottom: 1.5rem;
    }

    .mode-selector {
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1rem;
    }

    .mode-btn {
        padding: 0.5rem 1rem;
        border: 2px solid #ddd;
        background: white;
        border-radius: 4px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .mode-btn.active {
        background: #0a84ff;
        border-color: #0a84ff;
        color: white;
    }

    .search-input-container {
        display: flex;
        gap: 0.5rem;
    }

    .search-input {
        flex: 1;
        padding: 0.75rem;
        border: 2px solid #ddd;
        border-radius: 4px;
        font-size: 14px;
    }

    .search-input:focus {
        outline: none;
        border-color: #0a84ff;
    }

    .btn-search {
        padding: 0.75rem 2rem;
        background: #0a84ff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
    }

    .btn-search:hover:not(:disabled) {
        background: #0066cc;
    }

    .btn-search:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .results-section {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        overflow: hidden;
    }

    .results-header {
        padding: 1rem 1.5rem;
        border-bottom: 1px solid #eee;
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-wrap: wrap;
        gap: 1rem;
    }

    .results-info {
        font-weight: 600;
        color: #333;
    }

    .bulk-actions {
        display: flex;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .btn-small,
    .btn-tiny {
        padding: 0.5rem 1rem;
        background: #f0f0f0;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 13px;
    }

    .btn-tiny {
        padding: 0.25rem 0.5rem;
        font-size: 12px;
    }

    .btn-small:hover,
    .btn-tiny:hover {
        background: #e0e0e0;
    }

    .btn-warning {
        background: #ff9800;
        color: white;
    }

    .btn-warning:hover {
        background: #f57c00;
    }

    .btn-danger {
        background: #f44336;
        color: white;
    }

    .btn-danger:hover {
        background: #d32f2f;
    }

    .btn-secondary {
        padding: 0.5rem 1rem;
        background: #f0f0f0;
        color: #333;
        text-decoration: none;
        border-radius: 4px;
        display: inline-block;
    }

    .results-table {
        overflow-x: auto;
    }

    table {
        width: 100%;
        border-collapse: collapse;
    }

    th,
    td {
        padding: 0.75rem;
        text-align: left;
        border-bottom: 1px solid #eee;
    }

    th {
        background: #f9f9f9;
        font-weight: 600;
        color: #666;
        font-size: 13px;
        text-transform: uppercase;
    }

    tr.selected {
        background: #e3f2fd;
    }

    tr:hover {
        background: #f5f5f5;
    }

    tr.selected:hover {
        background: #bbdefb;
    }

    .col-checkbox {
        width: 40px;
        text-align: center;
    }

    .loading,
    .no-results {
        text-align: center;
        padding: 3rem;
        color: #666;
    }

    .pagination-note {
        padding: 1rem;
        text-align: center;
        color: #666;
        font-size: 14px;
        border-top: 1px solid #eee;
    }
</style>
