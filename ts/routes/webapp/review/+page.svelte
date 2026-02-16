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

<div class="reviewer-container">
    <div class="reviewer-header">
        <ReviewProgress />
        <div class="actions">
            <button
                on:click={handleUndo}
                disabled={!$reviewerStore.canUndo}
                title="Undo (Ctrl+Z)"
            >
                ↶ Undo
            </button>
            <button
                on:click={handleRedo}
                disabled={!$reviewerStore.canRedo}
                title="Redo (Ctrl+Shift+Z)"
            >
                ↷ Redo
            </button>
        </div>
    </div>

    {#if error}
        <div class="error-message">{error}</div>
    {/if}

    {#if loading}
        <div class="loading">Loading card...</div>
    {:else if $reviewerStore.finished}
        <div class="completion-screen">
            <h2>🎉 Study Complete!</h2>
            <p>You've finished reviewing this deck for now.</p>
            <a href="/webapp/decks" class="btn-primary">Back to Decks</a>
        </div>
    {:else if $reviewerStore.currentCard}
        <CardDisplay />
        {#if $reviewerStore.showingAnswer}
            <AnswerButtons on:answer={(e) => answerCard(e.detail)} />
        {:else}
            <div class="show-answer-container">
                <button
                    class="btn-show-answer"
                    on:click={() => reviewerStore.showAnswer()}
                >
                    Show Answer (Space)
                </button>
            </div>
        {/if}
    {/if}
</div>

<style>
    .reviewer-container {
        max-width: 900px;
        margin: 0 auto;
        padding: 20px;
    }

    .reviewer-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
        padding-bottom: 10px;
        border-bottom: 1px solid #e0e0e0;
    }

    .actions {
        display: flex;
        gap: 10px;
    }

    .actions button {
        padding: 8px 16px;
        background: #f5f5f5;
        border: 1px solid #ddd;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
    }

    .actions button:hover:not(:disabled) {
        background: #e8e8e8;
    }

    .actions button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .error-message {
        background: #fee;
        color: #c33;
        padding: 15px;
        border-radius: 4px;
        margin-bottom: 20px;
        border: 1px solid #fcc;
    }

    .loading {
        text-align: center;
        padding: 60px 20px;
        color: #666;
        font-size: 18px;
    }

    .completion-screen {
        text-align: center;
        padding: 80px 20px;
    }

    .completion-screen h2 {
        font-size: 32px;
        margin-bottom: 20px;
        color: #0a84ff;
    }

    .completion-screen p {
        font-size: 18px;
        color: #666;
        margin-bottom: 30px;
    }

    .btn-primary {
        display: inline-block;
        padding: 12px 32px;
        background: #0a84ff;
        color: white;
        text-decoration: none;
        border-radius: 6px;
        font-size: 16px;
        transition: background 0.2s;
    }

    .btn-primary:hover {
        background: #0066cc;
    }

    .show-answer-container {
        text-align: center;
        margin-top: 40px;
    }

    .btn-show-answer {
        padding: 16px 48px;
        background: #0a84ff;
        color: white;
        border: none;
        border-radius: 8px;
        font-size: 18px;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-show-answer:hover {
        background: #0066cc;
    }
</style>
