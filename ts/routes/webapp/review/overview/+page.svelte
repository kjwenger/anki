<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { onMount } from "svelte";
    import { api } from "$lib/webapp/api/client";
    import { goto } from "$app/navigation";

    export let data: { deckId: number };

    let loading = true;
    let error = "";
    let deckName = "";
    let counts = { new: 0, learning: 0, review: 0 };

    onMount(async () => {
        await loadOverview();
    });

    async function loadOverview() {
        loading = true;
        error = "";
        try {
            const [deck, deckCounts] = await Promise.all([
                api.getDeck(data.deckId),
                api.getDeckCounts(data.deckId),
            ]);
            deckName = deck.name;
            counts = deckCounts;
        } catch (e: any) {
            error = e.message || "Failed to load deck overview";
        } finally {
            loading = false;
        }
    }

    function startStudy() {
        goto(`/webapp/review?deck=${data.deckId}`);
    }

    function getDisplayName(fullName: string): string {
        const parts = fullName.split("::");
        return parts[parts.length - 1];
    }
</script>

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md px-8 py-6">
        <div class="max-w-4xl mx-auto flex justify-between items-center">
            <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">
                Deck Overview
            </h1>
            <a
                href="/webapp/decks"
                class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 no-underline rounded-lg text-sm hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
            >
                &larr; Back to Decks
            </a>
        </div>
    </header>

    <main class="max-w-4xl mx-auto p-8">
        {#if error}
            <div
                class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-4 mb-6"
            >
                {error}
            </div>
        {/if}

        {#if loading}
            <div class="text-center py-16 text-gray-500 dark:text-gray-400 text-lg">
                Loading deck details...
            </div>
        {:else}
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-md p-10 text-center">
                <h2 class="text-4xl text-gray-800 dark:text-gray-100 font-bold mb-8">
                    {getDisplayName(deckName)}
                </h2>

                <div class="grid grid-cols-3 gap-6 max-w-2xl mx-auto mb-12">
                    <div class="p-6 bg-blue-50 dark:bg-blue-900/20 rounded-xl border-2 border-blue-100 dark:border-blue-800">
                        <div class="text-3xl font-bold text-blue-600 dark:text-blue-400 mb-1">
                            {counts.new}
                        </div>
                        <div class="text-sm font-medium text-blue-500 dark:text-blue-500 uppercase tracking-wider">
                            New
                        </div>
                    </div>
                    <div class="p-6 bg-orange-50 dark:bg-orange-900/20 rounded-xl border-2 border-orange-100 dark:border-orange-800">
                        <div class="text-3xl font-bold text-orange-600 dark:text-orange-400 mb-1">
                            {counts.learning}
                        </div>
                        <div class="text-sm font-medium text-orange-500 dark:text-orange-500 uppercase tracking-wider">
                            Learn
                        </div>
                    </div>
                    <div class="p-6 bg-green-50 dark:bg-green-900/20 rounded-xl border-2 border-green-100 dark:border-green-800">
                        <div class="text-3xl font-bold text-green-600 dark:text-green-400 mb-1">
                            {counts.review}
                        </div>
                        <div class="text-sm font-medium text-green-500 dark:text-green-500 uppercase tracking-wider">
                            Review
                        </div>
                    </div>
                </div>

                <div class="flex flex-col items-center gap-4">
                    <button
                        class="px-12 py-4 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-xl text-xl font-bold cursor-pointer shadow-lg shadow-indigo-500/30 transition-all hover:-translate-y-0.5 active:translate-y-0 disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click={startStudy}
                        disabled={counts.new + counts.learning + counts.review === 0}
                    >
                        Study Now
                    </button>
                    
                    {#if counts.new + counts.learning + counts.review === 0}
                        <p class="text-gray-500 dark:text-gray-400 italic">
                            Congratulations! You've finished this deck for now.
                        </p>
                    {/if}
                </div>
            </div>
        {/if}
    </main>
</div>
