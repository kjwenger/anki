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
    let duplicateWarning = "";
    let checkTimeout: any;

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

        // Check for duplicates when fields change (debounced)
        clearTimeout(checkTimeout);
        checkTimeout = setTimeout(() => {
            checkForDuplicates();
        }, 500);
    }

    async function checkForDuplicates() {
        const state = $editorStore;
        if (!state.notetypeId || state.fields.length === 0) return;

        // Don't check if the first field is empty
        if (!state.fields[0].trim()) {
            duplicateWarning = "";
            return;
        }

        try {
            const result = await api.checkNoteFields(state.notetypeId, state.fields);
            // State 2 is DUPLICATE in anki_proto
            if (result.state === 2) {
                duplicateWarning = "Warning: This card appears to be a duplicate.";
            } else {
                duplicateWarning = "";
            }
        } catch (e) {
            console.error("Duplicate check failed:", e);
        }
    }

    function handleStickyChange(event: CustomEvent) {
        const { index, sticky } = event.detail;
        editorStore.setSticky(index, sticky);
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

    function handleKeydown(event: KeyboardEvent) {
        // Ctrl+Enter to submit
        if (event.ctrlKey && event.key === "Enter") {
            event.preventDefault();
            handleSubmit();
        }
    }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md px-8 py-6">
        <div class="max-w-7xl mx-auto flex justify-between items-center">
            <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">
                Add Cards
            </h1>
            <div class="flex gap-4">
                <a
                    href="/webapp"
                    class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 no-underline rounded-lg inline-block text-sm hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                >
                    &larr; Back
                </a>
            </div>
        </div>
    </header>

    <main class="max-w-7xl mx-auto p-8">
        {#if error}
            <div
                class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-4 mb-6"
            >
                {error}
            </div>
        {/if}

        {#if success}
            <div
                class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg text-green-600 dark:text-green-400 p-4 mb-6"
            >
                {success}
            </div>
        {/if}

        {#if duplicateWarning}
            <div
                class="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg text-amber-700 dark:text-amber-400 p-4 mb-6 flex items-center gap-3"
            >
                <span class="text-xl">⚠️</span>
                {duplicateWarning}
            </div>
        {/if}

        {#if loading}
            <div class="text-center py-16 text-gray-500 dark:text-gray-400 text-lg">
                Loading...
            </div>
        {:else}
            <div class="grid grid-cols-1 md:grid-cols-[250px_1fr] gap-8">
                <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md h-fit">
                    <div class="mb-6">
                        <label
                            for="notetype-select"
                            class="block font-semibold text-gray-800 dark:text-gray-200 mb-2 text-sm"
                        >
                            Note Type
                        </label>
                        <select
                            id="notetype-select"
                            on:change={handleNotetypeChange}
                            value={$editorStore.notetypeId || ""}
                            class="w-full p-2 border-2 border-gray-300 dark:border-gray-600 rounded-lg text-sm bg-white dark:bg-gray-700 dark:text-gray-100 cursor-pointer focus:outline-hidden focus:border-indigo-500 transition-colors"
                        >
                            {#each notetypes as notetype}
                                <option value={notetype.id}>{notetype.name}</option>
                            {/each}
                        </select>
                    </div>

                    <div>
                        <label
                            for="deck-select"
                            class="block font-semibold text-gray-800 dark:text-gray-200 mb-2 text-sm"
                        >
                            Deck
                        </label>
                        <select
                            id="deck-select"
                            on:change={handleDeckChange}
                            value={$editorStore.deckId || ""}
                            class="w-full p-2 border-2 border-gray-300 dark:border-gray-600 rounded-lg text-sm bg-white dark:bg-gray-700 dark:text-gray-100 cursor-pointer focus:outline-hidden focus:border-indigo-500 transition-colors"
                        >
                            {#each decks as deck}
                                <option value={deck.id}>{deck.name}</option>
                            {/each}
                        </select>
                    </div>
                </div>

                <div class="bg-white dark:bg-gray-800 p-8 rounded-lg shadow-md">
                    {#if $editorStore.notetype}
                        <form on:submit|preventDefault={handleSubmit}>
                            <div class="mb-5">
                                {#each $editorStore.notetype.fields as field, index}
                                    <FieldEditor
                                        label={field.name}
                                        value={$editorStore.fields[index] || ""}
                                        {index}
                                        isCloze={$editorStore.notetype.is_cloze}
                                        isSticky={$editorStore.stickyFields[index]}
                                        on:change={handleFieldChange}
                                        on:stickyChange={handleStickyChange}
                                    />
                                {/each}
                            </div>

                            <TagInput
                                tags={$editorStore.tags}
                                on:change={handleTagsChange}
                            />

                            <div class="flex gap-4 mt-8">
                                <button
                                    type="submit"
                                    class="px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-base font-medium cursor-pointer transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                                    disabled={saving}
                                >
                                    {saving ? "Adding..." : "Add Card"}
                                </button>
                                <button
                                    type="button"
                                    class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-base font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
                                    on:click={() => editorStore.resetFields()}
                                    disabled={saving}
                                >
                                    Clear
                                </button>
                            </div>
                        </form>
                    {:else}
                        <p class="text-center text-gray-500 dark:text-gray-400 py-10">
                            Please select a note type to begin.
                        </p>
                    {/if}
                </div>
            </div>
        {/if}
    </main>
</div>
