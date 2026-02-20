<script lang="ts">
    import { api } from "$lib/webapp/api/client";
    import { goto } from "$app/navigation";
    
    let fileInput: HTMLInputElement;
    let importing = false;
    let error = "";
    let success = "";
    let results: { notes_new: number; notes_updated: number; message: string } | null = null;

    async function handleFileChange(event: Event) {
        const target = event.target as HTMLInputElement;
        if (target.files && target.files.length > 0) {
            await importFile(target.files[0]);
        }
    }

    async function importFile(file: File) {
        if (!file.name.endsWith(".apkg") && !file.name.endsWith(".colpkg")) {
            error = "Please select an Anki package (.apkg or .colpkg)";
            success = "";
            results = null;
            return;
        }

        importing = true;
        error = "";
        success = "";
        results = null;
        try {
            results = await api.importApkg(file);
            success = "Import complete!";
        } catch (e: any) {
            error = e.message || "Import failed. The file might be corrupted or incompatible.";
        } finally {
            importing = false;
        }
    }

    function handleDrop(event: DragEvent) {
        event.preventDefault();
        if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
            importFile(event.dataTransfer.files[0]);
        }
    }

    function handleDragOver(event: DragEvent) {
        event.preventDefault();
    }

    function goToDashboard() {
        goto("/webapp");
    }
</script>

<svelte:head>
    <title>Import - Anki Web</title>
</svelte:head>

<div class="w-full">
    <header class="bg-white dark:bg-gray-800 shadow-xs px-8 py-6">
        <div class="max-w-7xl mx-auto flex justify-between items-center">
            <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">
                Import Package
            </h1>
            <button
                class="px-6 py-3 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 border-none rounded-lg text-sm font-medium cursor-pointer transition-colors duration-200 hover:bg-gray-200 dark:hover:bg-gray-600"
                on:click={goToDashboard}
            >
                &larr; Dashboard
            </button>
        </div>
    </header>

    <main class="max-w-7xl mx-auto p-8">
        {#if error}
            <div class="mb-6 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-700 dark:text-red-400 flex items-start gap-3">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mt-0.5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
                <p>{error}</p>
            </div>
        {/if}

        {#if success}
            <div class="mb-6 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg text-green-700 dark:text-green-400 flex items-start gap-3">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mt-0.5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                </svg>
                <p>{success}</p>
            </div>
        {/if}

        <div
            class="border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-lg p-12 text-center transition-all hover:border-indigo-500 dark:hover:border-indigo-400 bg-white dark:bg-gray-800 shadow-sm"
            role="button"
            tabindex="0"
            on:drop={handleDrop}
            on:dragover={handleDragOver}
        >
            {#if importing}
                <div class="flex flex-col items-center gap-4 py-4">
                    <div class="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
                    <div class="animate-pulse">
                        <p class="text-lg font-semibold text-gray-900 dark:text-white">Importing your data...</p>
                        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">This may take a minute for large packages.</p>
                    </div>
                </div>
            {:else}
                <div class="flex flex-col items-center gap-6">
                    <div class="w-20 h-20 bg-indigo-50 dark:bg-indigo-900/30 rounded-full flex items-center justify-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="w-10 h-10 text-indigo-600 dark:text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                        </svg>
                    </div>
                    <div>
                        <p class="text-xl font-bold text-gray-900 dark:text-white mb-2">
                            Click or drag Anki package to upload
                        </p>
                        <p class="text-gray-500 dark:text-gray-400 max-w-sm mx-auto text-sm">
                            Compatible with .apkg (decks) and .colpkg (collection) formats from Anki Desktop and AnkiMobile.
                        </p>
                    </div>
                    <button
                        class="px-8 py-3 bg-indigo-500 hover:bg-indigo-600 text-white rounded-lg font-medium transition-all shadow-md hover:shadow-lg active:scale-95 cursor-pointer border-none"
                        on:click={() => fileInput.click()}
                    >
                        Choose File
                    </button>
                    <input
                        type="file"
                        accept=".apkg,.colpkg"
                        class="hidden"
                        bind:this={fileInput}
                        on:change={handleFileChange}
                    />
                </div>
            {/if}
        </div>

        {#if results}
            <div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
                <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-sm border border-gray-100 dark:border-gray-700 flex flex-col items-center justify-center text-center">
                    <p class="text-sm font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2">Notes Added</p>
                    <p class="text-4xl font-black text-indigo-600 dark:text-indigo-400">{results.notes_new}</p>
                </div>
                <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-sm border border-gray-100 dark:border-gray-700 flex flex-col items-center justify-center text-center">
                    <p class="text-sm font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2">Notes Updated</p>
                    <p class="text-4xl font-black text-gray-900 dark:text-white">{results.notes_updated}</p>
                </div>
                <div class="md:col-span-2 bg-indigo-500 p-6 rounded-lg shadow-lg flex items-center justify-between">
                    <div class="text-white">
                        <p class="text-lg font-bold">Import Finished Successfully</p>
                        <p class="text-indigo-100 opacity-90">{results.message}</p>
                    </div>
                    <button
                        on:click={() => goto("/webapp/decks")}
                        class="px-6 py-3 bg-white text-indigo-600 rounded-lg font-medium hover:bg-indigo-50 transition-colors shadow-sm border-none cursor-pointer"
                    >
                        View Decks
                    </button>
                </div>
            </div>
        {/if}

        <div class="mt-12 bg-gray-50 dark:bg-gray-800/50 p-8 rounded-lg border border-gray-200 dark:border-gray-700 shadow-xs">
            <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4 text-center md:text-left">Important Information</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center md:items-start text-center md:text-left gap-3">
                    <div class="w-8 h-8 bg-indigo-100 dark:bg-indigo-900/40 rounded-full flex items-center justify-center shrink-0 text-indigo-600 dark:text-indigo-400 text-sm font-bold">1</div>
                    <p class="text-sm text-gray-600 dark:text-gray-400"><strong>Merging:</strong> Importing a package will merge its contents with your current collection. Your existing cards are safe.</p>
                </div>
                <div class="flex flex-col items-center md:items-start text-center md:text-left gap-3">
                    <div class="w-8 h-8 bg-indigo-100 dark:bg-indigo-900/40 rounded-full flex items-center justify-center shrink-0 text-indigo-600 dark:text-indigo-400 text-sm font-bold">2</div>
                    <p class="text-sm text-gray-600 dark:text-gray-400"><strong>Duplicates:</strong> If a note in the package already exists in your collection, it will be updated if the version in the package is newer.</p>
                </div>
                <div class="flex flex-col items-center md:items-start text-center md:text-left gap-3">
                    <div class="w-8 h-8 bg-indigo-100 dark:bg-indigo-900/40 rounded-full flex items-center justify-center shrink-0 text-indigo-600 dark:text-indigo-400 text-sm font-bold">3</div>
                    <p class="text-sm text-gray-600 dark:text-gray-400"><strong>Media:</strong> Images and audio files included in the package will be automatically added to your media library.</p>
                </div>
            </div>
        </div>
    </main>
</div>
