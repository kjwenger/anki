<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let show = false;

    let name = "";
    let error = "";
    let loading = false;

    const dispatch = createEventDispatcher();

    function handleClose() {
        name = "";
        error = "";
        show = false;
        dispatch("close");
    }

    function handleSubmit() {
        if (!name.trim()) {
            error = "Collection name is required";
            return;
        }

        if (name.length < 2) {
            error = "Collection name must be at least 2 characters";
            return;
        }

        dispatch("create", { name: name.trim() });
    }

    function handleKeyPress(event: KeyboardEvent) {
        if (event.key === "Enter") {
            handleSubmit();
        } else if (event.key === "Escape") {
            handleClose();
        }
    }

    $: if (show) {
        name = "";
        error = "";
        loading = false;
    }
</script>

{#if show}
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
        on:click={handleClose}
        role="presentation"
    >
        <div
            class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-[90%] max-w-lg max-h-[90vh] overflow-auto"
            on:click|stopPropagation
            role="dialog"
            aria-modal="true"
        >
            <div
                class="flex justify-between items-center p-6 border-b border-gray-200 dark:border-gray-700"
            >
                <h2 class="m-0 text-xl text-gray-800 dark:text-gray-100 font-semibold">
                    Create New Collection
                </h2>
                <button
                    class="bg-transparent border-none text-3xl text-gray-400 dark:text-gray-500 cursor-pointer p-0 w-8 h-8 flex items-center justify-center leading-none hover:text-gray-600 dark:hover:text-gray-300"
                    on:click={handleClose}
                    aria-label="Close"
                >
                    &times;
                </button>
            </div>

            <div class="p-6">
                {#if error}
                    <div
                        class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-3 mb-4 text-sm"
                        role="alert"
                    >
                        {error}
                    </div>
                {/if}

                <div class="mb-4">
                    <label
                        for="collection-name"
                        class="block mb-2 text-gray-700 dark:text-gray-300 font-medium text-sm"
                    >
                        Collection Name
                    </label>
                    <input
                        id="collection-name"
                        type="text"
                        bind:value={name}
                        on:keydown={handleKeyPress}
                        placeholder="Enter collection name"
                        disabled={loading}
                        autofocus
                        class="w-full px-3 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-base transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 disabled:bg-gray-100 dark:disabled:bg-gray-700 disabled:cursor-not-allowed bg-white dark:bg-gray-700 dark:text-gray-100"
                    />
                </div>
            </div>

            <div
                class="flex justify-end gap-4 p-6 border-t border-gray-200 dark:border-gray-700"
            >
                <button
                    class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:bg-gray-300 disabled:cursor-not-allowed"
                    on:click={handleClose}
                    disabled={loading}
                >
                    Cancel
                </button>
                <button
                    class="px-6 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 disabled:bg-gray-400 disabled:cursor-not-allowed"
                    on:click={handleSubmit}
                    disabled={loading}
                >
                    {loading ? "Creating..." : "Create"}
                </button>
            </div>
        </div>
    </div>
{/if}
