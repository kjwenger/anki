<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { onMount } from "svelte";
    import { api } from "$lib/webapp/api/client";

    let loading = true;
    let error = "";
    let todayStats: any = null;
    let collectionStats: any = null;

    onMount(async () => {
        await loadStats();
    });

    async function loadStats() {
        loading = true;
        error = "";
        try {
            const [today, collection] = await Promise.all([
                api.getTodayStats(),
                api.getCollectionStats(),
            ]);
            todayStats = today;
            collectionStats = collection;
        } catch (e: any) {
            error = e.message || "Failed to load statistics";
        } finally {
            loading = false;
        }
    }

    function formatTime(millis: number): string {
        const seconds = Math.floor(millis / 1000);
        const minutes = Math.floor(seconds / 60);
        const hours = Math.floor(minutes / 60);

        if (hours > 0) {
            return `${hours}h ${minutes % 60}m`;
        } else if (minutes > 0) {
            return `${minutes}m ${seconds % 60}s`;
        } else {
            return `${seconds}s`;
        }
    }

    function formatPercent(numerator: number, denominator: number): string {
        if (denominator === 0) return "0%";
        return `${Math.round((numerator / denominator) * 100)}%`;
    }
</script>

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md px-8 py-6">
        <div class="max-w-7xl mx-auto flex justify-between items-center">
            <h1 class="m-0 text-3xl text-gray-800 dark:text-gray-100 font-bold">
                Statistics
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

        {#if loading}
            <div class="text-center py-16 text-gray-500 dark:text-gray-400 text-lg">
                Loading statistics...
            </div>
        {:else}
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <section class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                    <h2
                        class="m-0 mb-6 text-xl text-gray-800 dark:text-gray-100 font-semibold pb-3 border-b-2 border-gray-100 dark:border-gray-700"
                    >
                        Today's Stats
                    </h2>
                    <div class="grid grid-cols-3 gap-4 mb-6">
                        <div
                            class="text-center p-4 rounded-lg bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400"
                        >
                            <div class="text-3xl font-bold leading-none mb-2">
                                {todayStats?.answer_count || 0}
                            </div>
                            <div
                                class="text-xs font-medium uppercase tracking-wider opacity-80"
                            >
                                Cards Answered
                            </div>
                        </div>
                        <div
                            class="text-center p-4 rounded-lg bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400"
                        >
                            <div class="text-3xl font-bold leading-none mb-2">
                                {todayStats?.correct_count || 0}
                            </div>
                            <div
                                class="text-xs font-medium uppercase tracking-wider opacity-80"
                            >
                                Correct
                            </div>
                        </div>
                        <div
                            class="text-center p-4 rounded-lg bg-orange-50 dark:bg-orange-900/20 text-orange-700 dark:text-orange-400"
                        >
                            <div class="text-3xl font-bold leading-none mb-2">
                                {formatTime(todayStats?.answer_millis || 0)}
                            </div>
                            <div
                                class="text-xs font-medium uppercase tracking-wider opacity-80"
                            >
                                Study Time
                            </div>
                        </div>
                    </div>

                    {#if todayStats && todayStats.answer_count > 0}
                        <div class="flex flex-col gap-3">
                            <div
                                class="flex justify-between items-center py-2 border-b border-gray-100 dark:border-gray-700"
                            >
                                <span class="text-gray-500 dark:text-gray-400">
                                    Accuracy:
                                </span>
                                <strong class="text-gray-800 dark:text-gray-200">
                                    {formatPercent(
                                        todayStats.correct_count,
                                        todayStats.answer_count,
                                    )}
                                </strong>
                            </div>
                            <div
                                class="flex justify-between items-center py-2 border-b border-gray-100 dark:border-gray-700"
                            >
                                <span class="text-gray-500 dark:text-gray-400">
                                    Learn:
                                </span>
                                <strong class="text-gray-800 dark:text-gray-200">
                                    {todayStats.learn_count}
                                </strong>
                            </div>
                            <div
                                class="flex justify-between items-center py-2 border-b border-gray-100 dark:border-gray-700"
                            >
                                <span class="text-gray-500 dark:text-gray-400">
                                    Review:
                                </span>
                                <strong class="text-gray-800 dark:text-gray-200">
                                    {todayStats.review_count}
                                </strong>
                            </div>
                            <div class="flex justify-between items-center py-2">
                                <span class="text-gray-500 dark:text-gray-400">
                                    Relearn:
                                </span>
                                <strong class="text-gray-800 dark:text-gray-200">
                                    {todayStats.relearn_count}
                                </strong>
                            </div>
                        </div>
                    {/if}
                </section>

                <section class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                    <h2
                        class="m-0 mb-6 text-xl text-gray-800 dark:text-gray-100 font-semibold pb-3 border-b-2 border-gray-100 dark:border-gray-700"
                    >
                        Collection Overview
                    </h2>
                    <div class="grid grid-cols-2 gap-4 mb-6">
                        <div
                            class="text-center p-4 rounded-lg bg-gray-50 dark:bg-gray-700/50"
                        >
                            <div
                                class="text-3xl font-bold leading-none mb-2 text-gray-800 dark:text-gray-200"
                            >
                                {collectionStats?.total_cards || 0}
                            </div>
                            <div
                                class="text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400"
                            >
                                Total Cards
                            </div>
                        </div>
                        <div
                            class="text-center p-4 rounded-lg bg-gray-50 dark:bg-gray-700/50"
                        >
                            <div
                                class="text-3xl font-bold leading-none mb-2 text-gray-800 dark:text-gray-200"
                            >
                                {collectionStats?.total_notes || 0}
                            </div>
                            <div
                                class="text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400"
                            >
                                Total Notes
                            </div>
                        </div>
                    </div>

                    <div class="flex flex-col gap-3">
                        <div
                            class="flex justify-between items-center py-2 border-b border-gray-100 dark:border-gray-700"
                        >
                            <span class="text-gray-500 dark:text-gray-400">New:</span>
                            <strong class="text-gray-800 dark:text-gray-200">
                                {collectionStats?.new_cards || 0}
                            </strong>
                        </div>
                        <div
                            class="flex justify-between items-center py-2 border-b border-gray-100 dark:border-gray-700"
                        >
                            <span class="text-gray-500 dark:text-gray-400">Young:</span>
                            <strong class="text-gray-800 dark:text-gray-200">
                                {collectionStats?.young_cards || 0}
                            </strong>
                        </div>
                        <div
                            class="flex justify-between items-center py-2 border-b border-gray-100 dark:border-gray-700"
                        >
                            <span class="text-gray-500 dark:text-gray-400">
                                Mature:
                            </span>
                            <strong class="text-gray-800 dark:text-gray-200">
                                {collectionStats?.mature_cards || 0}
                            </strong>
                        </div>
                        <div
                            class="flex justify-between items-center py-2 border-b border-gray-100 dark:border-gray-700"
                        >
                            <span class="text-gray-500 dark:text-gray-400">
                                Suspended:
                            </span>
                            <strong class="text-gray-800 dark:text-gray-200">
                                {collectionStats?.suspended_cards || 0}
                            </strong>
                        </div>
                        <div class="flex justify-between items-center py-2">
                            <span class="text-gray-500 dark:text-gray-400">
                                Buried:
                            </span>
                            <strong class="text-gray-800 dark:text-gray-200">
                                {collectionStats?.buried_cards || 0}
                            </strong>
                        </div>
                    </div>
                </section>

                {#if todayStats && todayStats.mature_count > 0}
                    <section class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                        <h2
                            class="m-0 mb-6 text-xl text-gray-800 dark:text-gray-100 font-semibold pb-3 border-b-2 border-gray-100 dark:border-gray-700"
                        >
                            Mature Cards
                        </h2>
                        <div class="grid grid-cols-2 gap-4">
                            <div
                                class="text-center p-4 rounded-lg bg-gray-50 dark:bg-gray-700/50"
                            >
                                <div
                                    class="text-3xl font-bold leading-none mb-2 text-gray-800 dark:text-gray-200"
                                >
                                    {todayStats.mature_count}
                                </div>
                                <div
                                    class="text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400"
                                >
                                    Reviewed
                                </div>
                            </div>
                            <div
                                class="text-center p-4 rounded-lg bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400"
                            >
                                <div class="text-3xl font-bold leading-none mb-2">
                                    {formatPercent(
                                        todayStats.mature_correct,
                                        todayStats.mature_count,
                                    )}
                                </div>
                                <div
                                    class="text-xs font-medium uppercase tracking-wider opacity-80"
                                >
                                    Retention
                                </div>
                            </div>
                        </div>
                    </section>
                {/if}

                <section
                    class="bg-gradient-to-br from-indigo-500 to-purple-600 rounded-lg shadow-lg p-6 text-white"
                >
                    <h2
                        class="m-0 mb-6 text-xl font-semibold pb-3 border-b-2 border-white/30"
                    >
                        Statistics
                    </h2>
                    <p class="m-0 my-3 leading-relaxed opacity-95">
                        Track your learning progress with detailed statistics.
                    </p>
                    <p class="m-0 my-3 leading-relaxed opacity-95">
                        Study consistently to improve retention and master your
                        flashcards!
                    </p>
                </section>
            </div>
        {/if}
    </main>
</div>
