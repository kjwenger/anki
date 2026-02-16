<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { onMount } from "svelte";
    import { api } from "$lib/webapp/api/client";
    import { reviewerStore } from "$lib/webapp/stores/reviewer";
    import CardDisplay from "$lib/webapp/components/CardDisplay.svelte";
    import AnswerButtons from "$lib/webapp/components/AnswerButtons.svelte";
    import ReviewProgress from "$lib/webapp/components/ReviewProgress.svelte";

    export let data: { deckId: number };

    let loading = true;
    let error = "";

    onMount(async () => {
        reviewerStore.setDeckId(data.deckId);
        await loadNextCard();
    });

    async function loadNextCard() {
        loading = true;
        error = "";
        try {
            const response = await api.getNextCard(data.deckId);
            reviewerStore.setCard(response.card, response.finished);
        } catch (e: any) {
            error = e.message || "Failed to load card";
        } finally {
            loading = false;
        }
    }

    async function answerCard(rating: number) {
        const state = $reviewerStore;
        if (!state.currentCard) return;

        try {
            await api.answerCard(
                data.deckId,
                state.currentCard.card_id,
                rating,
            );
            reviewerStore.setUndoRedo(true, false);
            await loadNextCard();
        } catch (e: any) {
            error = e.message || "Failed to answer card";
        }
    }

    async function handleUndo() {
        try {
            await api.undo();
            reviewerStore.setUndoRedo(false, true);
            await loadNextCard();
        } catch (e: any) {
            error = e.message || "Nothing to undo";
        }
    }

    async function handleRedo() {
        try {
            await api.redo();
            await loadNextCard();
        } catch (e: any) {
            error = e.message || "Nothing to redo";
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        const state = $reviewerStore;
        if (loading) return;

        // Space or Enter to show answer
        if (
            !state.showingAnswer &&
            (event.key === " " || event.key === "Enter")
        ) {
            event.preventDefault();
            reviewerStore.showAnswer();
            return;
        }

        // Number keys for rating (1=Again, 2=Hard, 3=Good, 4=Easy)
        if (state.showingAnswer && event.key >= "1" && event.key <= "4") {
            event.preventDefault();
            const rating = parseInt(event.key) - 1;
            answerCard(rating);
            return;
        }

        // Ctrl+Z for undo
        if (event.ctrlKey && event.key === "z" && state.canUndo) {
            event.preventDefault();
            handleUndo();
            return;
        }

        // Ctrl+Shift+Z or Ctrl+Y for redo
        if (
            ((event.ctrlKey && event.shiftKey && event.key === "Z") ||
                (event.ctrlKey && event.key === "y")) &&
            state.canRedo
        ) {
            event.preventDefault();
            handleRedo();
        }
    }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="max-w-4xl mx-auto p-5">
    <div class="flex justify-between items-center mb-5 pb-3 border-b border-gray-200 dark:border-gray-700">
        <ReviewProgress />
        <div class="flex gap-3">
            <button
                class="px-4 py-2 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg cursor-pointer text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                on:click={handleUndo}
                disabled={!$reviewerStore.canUndo}
                title="Undo (Ctrl+Z)"
            >
                ↶ Undo
            </button>
            <button
                class="px-4 py-2 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg cursor-pointer text-sm text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                on:click={handleRedo}
                disabled={!$reviewerStore.canRedo}
                title="Redo (Ctrl+Shift+Z)"
            >
                ↷ Redo
            </button>
        </div>
    </div>

    {#if error}
        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-600 dark:text-red-400 p-4 mb-5">{error}</div>
    {/if}

    {#if loading}
        <div class="text-center py-16 text-gray-500 dark:text-gray-400 text-lg">Loading card...</div>
    {:else if $reviewerStore.finished}
        <div class="text-center py-20">
            <h2 class="text-3xl mb-5 text-indigo-500 dark:text-indigo-400 font-bold">Study Complete!</h2>
            <p class="text-lg text-gray-500 dark:text-gray-400 mb-8">You've finished reviewing this deck for now.</p>
            <a href="/webapp/decks" class="inline-block px-8 py-3 bg-indigo-500 hover:bg-indigo-600 text-white no-underline rounded-lg text-base font-medium transition-colors">Back to Decks</a>
        </div>
    {:else if $reviewerStore.currentCard}
        <CardDisplay />
        {#if $reviewerStore.showingAnswer}
            <AnswerButtons on:answer={(e) => answerCard(e.detail)} />
        {:else}
            <div class="text-center mt-10">
                <button
                    class="px-12 py-4 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-lg cursor-pointer transition-colors"
                    on:click={() => reviewerStore.showAnswer()}
                >
                    Show Answer (Space)
                </button>
            </div>
        {/if}
    {/if}
</div>
