<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { api } from "$lib/webapp/api/client";
    import { editorStore } from "$lib/webapp/stores/editor";
    import FieldEditor from "$lib/webapp/components/FieldEditor.svelte";
    import TagInput from "$lib/webapp/components/TagInput.svelte";

    let decks: Array<{ id: number; name: string }> = [];
    let notetypes: Array<{ id: number; name: string }> = [];
    let loading = true;
    let saving = false;
    let error = "";
    let success = "";

    onMount(async () => {
        await loadDecksAndNotetypes();
    });

    async function loadDecksAndNotetypes() {
        loading = true;
        error = "";
        try {
            const [decksResponse, notetypesResponse] = await Promise.all([
                api.getDecks(),
                api.getNotetypes(),
            ]);

            decks = decksResponse.decks;
            notetypes = notetypesResponse.notetypes;

            // Set default deck and notetype
            if (decks.length > 0 && !$editorStore.deckId) {
                editorStore.setDeck(decks[0].id);
            }
            if (notetypes.length > 0 && !$editorStore.notetypeId) {
                await selectNotetype(notetypes[0].id);
            }
        } catch (e: any) {
            error = e.message || "Failed to load decks and notetypes";
        } finally {
            loading = false;
        }
    }

    async function selectNotetype(notetypeId: number) {
        try {
            const notetype = await api.getNotetype(notetypeId);
            editorStore.setNotetype(notetypeId, notetype);
        } catch (e: any) {
            error = e.message || "Failed to load notetype";
        }
    }

    function handleFieldChange(event: CustomEvent) {
        const { index, value } = event.detail;
        editorStore.setField(index, value);
    }

    function handleTagsChange(event: CustomEvent) {
        editorStore.setTags(event.detail);
    }

    async function handleSubmit() {
        const state = $editorStore;

        if (!state.deckId || !state.notetypeId || !state.notetype) {
            error = "Please select a deck and notetype";
            return;
        }

        // Check if all fields are filled
        const emptyFields = state.fields.filter((f) => !f.trim());
        if (emptyFields.length === state.fields.length) {
            error = "Please fill in at least one field";
            return;
        }

        saving = true;
        error = "";
        success = "";

        try {
            const result = await api.createNote(
                state.deckId,
                state.notetypeId,
                state.fields,
                state.tags,
            );

            success = `Card created successfully! (ID: ${result.note_id})`;
            
            // Reset form for next card
            editorStore.resetFields();

            // Auto-clear success message after 3 seconds
            setTimeout(() => {
                success = "";
            }, 3000);
        } catch (e: any) {
            error = e.message || "Failed to create card";
        } finally {
            saving = false;
        }
    }

    function handleDeckChange(event: Event) {
        const select = event.target as HTMLSelectElement;
        editorStore.setDeck(parseInt(select.value));
    }

    async function handleNotetypeChange(event: Event) {
        const select = event.target as HTMLSelectElement;
        await selectNotetype(parseInt(select.value));
    }
</script>

<div class="editor-page">
    <header class="page-header">
        <div class="header-content">
            <h1>Add Cards</h1>
            <div class="header-actions">
                <a href="/webapp/decks" class="btn-secondary"> ← Back to Decks </a>
            </div>
        </div>
    </header>

    <main class="page-content">
        {#if error}
            <div class="error-banner">{error}</div>
        {/if}

        {#if success}
            <div class="success-banner">{success}</div>
        {/if}

        {#if loading}
            <div class="loading">Loading...</div>
        {:else}
            <div class="editor-container">
                <div class="editor-sidebar">
                    <div class="selector-group">
                        <label for="notetype-select">Note Type</label>
                        <select
                            id="notetype-select"
                            on:change={handleNotetypeChange}
                            value={$editorStore.notetypeId || ""}
                        >
                            {#each notetypes as notetype}
                                <option value={notetype.id}>{notetype.name}</option>
                            {/each}
                        </select>
                    </div>

                    <div class="selector-group">
                        <label for="deck-select">Deck</label>
                        <select
                            id="deck-select"
                            on:change={handleDeckChange}
                            value={$editorStore.deckId || ""}
                        >
                            {#each decks as deck}
                                <option value={deck.id}>{deck.name}</option>
                            {/each}
                        </select>
                    </div>
                </div>

                <div class="editor-main">
                    {#if $editorStore.notetype}
                        <form on:submit|preventDefault={handleSubmit}>
                            <div class="fields-container">
                                {#each $editorStore.notetype.fields as field, index}
                                    <FieldEditor
                                        label={field.name}
                                        value={$editorStore.fields[index] || ""}
                                        {index}
                                        on:change={handleFieldChange}
                                    />
                                {/each}
                            </div>

                            <TagInput
                                tags={$editorStore.tags}
                                on:change={handleTagsChange}
                            />

                            <div class="form-actions">
                                <button
                                    type="submit"
                                    class="btn-primary"
                                    disabled={saving}
                                >
                                    {saving ? "Adding..." : "Add Card"}
                                </button>
                                <button
                                    type="button"
                                    class="btn-secondary"
                                    on:click={() => editorStore.resetFields()}
                                    disabled={saving}
                                >
                                    Clear
                                </button>
                            </div>
                        </form>
                    {:else}
                        <p class="no-notetype">Please select a note type to begin.</p>
                    {/if}
                </div>
            </div>
        {/if}
    </main>
</div>

<style>
    .editor-page {
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

    .success-banner {
        background: #efe;
        border: 1px solid #cfc;
        border-radius: 8px;
        color: #3c3;
        padding: 1rem;
        margin-bottom: 1.5rem;
    }

    .loading {
        text-align: center;
        padding: 60px 20px;
        color: #666;
        font-size: 18px;
    }

    .editor-container {
        display: grid;
        grid-template-columns: 250px 1fr;
        gap: 2rem;
    }

    .editor-sidebar {
        background: white;
        padding: 1.5rem;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        height: fit-content;
    }

    .selector-group {
        margin-bottom: 1.5rem;
    }

    .selector-group:last-child {
        margin-bottom: 0;
    }

    .selector-group label {
        display: block;
        font-weight: 600;
        color: #333;
        margin-bottom: 6px;
        font-size: 14px;
    }

    .selector-group select {
        width: 100%;
        padding: 8px;
        border: 2px solid #ddd;
        border-radius: 4px;
        font-size: 14px;
        background: white;
        cursor: pointer;
    }

    .selector-group select:focus {
        outline: none;
        border-color: #0a84ff;
    }

    .editor-main {
        background: white;
        padding: 2rem;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .fields-container {
        margin-bottom: 20px;
    }

    .form-actions {
        display: flex;
        gap: 1rem;
        margin-top: 2rem;
    }

    .btn-primary,
    .btn-secondary {
        padding: 12px 24px;
        border: none;
        border-radius: 4px;
        font-size: 16px;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-primary {
        background: #0a84ff;
        color: white;
    }

    .btn-primary:hover:not(:disabled) {
        background: #0066cc;
    }

    .btn-primary:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .btn-secondary {
        background: #f0f0f0;
        color: #333;
        text-decoration: none;
        display: inline-block;
    }

    .btn-secondary:hover:not(:disabled) {
        background: #e0e0e0;
    }

    .btn-secondary:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .no-notetype {
        text-align: center;
        color: #666;
        padding: 40px;
    }

    @media (max-width: 768px) {
        .editor-container {
            grid-template-columns: 1fr;
        }
    }
</style>
