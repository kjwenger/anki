<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { createEventDispatcher } from "svelte";
    import { api } from "$lib/webapp/api/client";
    import FieldEditor from "./FieldEditor.svelte";
    import TagInput from "./TagInput.svelte";

    /** Whether the dialog is visible. */
    export let open: boolean = false;
    /** The note ID to load and edit. */
    export let noteId: number | null = null;

    const dispatch = createEventDispatcher<{ close: void; saved: void }>();

    type Notetype = {
        id: number;
        name: string;
        fields: { name: string; ord: number }[];
        is_cloze: boolean;
    };

    let loading = false;
    let saving = false;
    let error = "";
    let success = "";

    let notetype: Notetype | null = null;
    let fields: string[] = [];
    let tags: string[] = [];
    let currentNoteId: number | null = null;

    // Load note whenever noteId changes and dialog opens
    $: if (open && noteId !== null && noteId !== currentNoteId) {
        loadNote(noteId);
    }

    // Reset state when dialog closes
    $: if (!open) {
        error = "";
        success = "";
    }

    async function loadNote(id: number) {
        loading = true;
        error = "";
        success = "";
        notetype = null;
        try {
            const note = await api.getNote(id);
            const nt = await api.getNotetype(note.notetype_id);
            fields = [...note.fields];
            tags = [...note.tags];
            notetype = nt;
            currentNoteId = id;
        } catch (e: any) {
            error = e.message || "Failed to load note";
        } finally {
            loading = false;
        }
    }

    function handleFieldChange(event: CustomEvent) {
        const { index, value } = event.detail;
        fields[index] = value;
        fields = fields;
    }

    function handleTagsChange(event: CustomEvent) {
        tags = event.detail;
    }

    async function handleSave() {
        if (currentNoteId === null) return;
        saving = true;
        error = "";
        success = "";
        try {
            await api.updateNote(currentNoteId, fields, tags);
            success = "Saved successfully.";
            dispatch("saved");
            setTimeout(() => {
                handleClose();
            }, 800);
        } catch (e: any) {
            error = e.message || "Failed to save note";
        } finally {
            saving = false;
        }
    }

    function handleClose() {
        open = false;
        dispatch("close");
    }

    function handleBackdropClick(event: MouseEvent) {
        if (event.target === event.currentTarget) {
            handleClose();
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Escape") {
            handleClose();
        } else if (event.ctrlKey && event.key === "Enter") {
            event.preventDefault();
            handleSave();
        }
    }
</script>

{#if open}
    <!-- Backdrop -->
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
        on:click={handleBackdropClick}
        on:keydown={handleKeydown}
    >
        <!-- Dialog panel -->
        <div
            class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col"
            role="dialog"
            aria-modal="true"
            aria-label="Edit Note"
        >
            <!-- Header -->
            <div
                class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700"
            >
                <h2 class="text-xl font-bold text-gray-800 dark:text-gray-100 m-0">
                    {notetype ? notetype.name : "Edit Note"}
                </h2>
                <button
                    class="p-2 rounded-lg text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors border-none cursor-pointer bg-transparent"
                    on:click={handleClose}
                    aria-label="Close"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>

            <!-- Body -->
            <div class="flex-1 overflow-y-auto px-6 py-4">
                {#if error}
                    <div
                        class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-3 mb-4 text-sm"
                    >
                        {error}
                    </div>
                {/if}

                {#if success}
                    <div
                        class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg text-green-600 dark:text-green-400 p-3 mb-4 text-sm"
                    >
                        {success}
                    </div>
                {/if}

                {#if loading}
                    <div class="text-center py-12 text-gray-500 dark:text-gray-400">
                        Loading...
                    </div>
                {:else if notetype}
                    {#each notetype.fields as field, index}
                        <FieldEditor
                            label={field.name}
                            value={fields[index] ?? ""}
                            {index}
                            isCloze={notetype.is_cloze}
                            isSticky={false}
                            on:change={handleFieldChange}
                        />
                    {/each}

                    <TagInput {tags} on:change={handleTagsChange} />
                {/if}
            </div>

            <!-- Footer -->
            <div
                class="flex justify-end gap-3 px-6 py-4 border-t border-gray-200 dark:border-gray-700"
            >
                <button
                    class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg cursor-pointer text-sm font-medium hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                    on:click={handleClose}
                >
                    Cancel
                </button>
                <button
                    class="px-5 py-2 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg cursor-pointer text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click={handleSave}
                    disabled={saving || loading || !notetype}
                    title="Save (Ctrl+Enter)"
                >
                    {saving ? "Saving..." : "Save"}
                </button>
            </div>
        </div>
    </div>
{/if}
