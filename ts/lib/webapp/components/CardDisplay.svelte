<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { reviewerStore } from "$lib/webapp/stores/reviewer";
    import { onMount, onDestroy } from "svelte";

    $: card = $reviewerStore.currentCard;
    $: showingAnswer = $reviewerStore.showingAnswer;

    let styleElement: HTMLStyleElement | null = null;

    $: {
        if (typeof document !== "undefined") {
            if (styleElement) {
                styleElement.remove();
            }
            if (card && card.css) {
                styleElement = document.createElement("style");
                styleElement.textContent = card.css;
                document.head.appendChild(styleElement);
            }
        }
    }

    onDestroy(() => {
        if (styleElement) {
            styleElement.remove();
            styleElement = null;
        }
    });
</script>

{#if card}
    <div class="card-container">
        <div class="card-content">
            <div class="question">
                {@html card.question_html}
            </div>

            {#if showingAnswer}
                <hr class="divider" />
                <div class="answer">
                    {@html card.answer_html}
                </div>
            {/if}
        </div>
    </div>
{/if}

<style>
    .card-container {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        padding: 40px;
        margin: 20px 0;
        min-height: 300px;
    }

    .card-content {
        font-size: 20px;
        line-height: 1.6;
    }

    .question {
        margin-bottom: 20px;
    }

    .divider {
        margin: 30px 0;
        border: none;
        border-top: 2px solid #e0e0e0;
    }

    .answer {
        margin-top: 20px;
    }

    /* Override global styles for card content */
    .card-container :global(img) {
        max-width: 100%;
        height: auto;
    }

    .card-container :global(pre) {
        background: #f5f5f5;
        padding: 15px;
        border-radius: 4px;
        overflow-x: auto;
    }

    .card-container :global(code) {
        background: #f5f5f5;
        padding: 2px 6px;
        border-radius: 3px;
        font-family: "Courier New", monospace;
    }

    .card-container :global(table) {
        border-collapse: collapse;
        width: 100%;
        margin: 15px 0;
    }

    .card-container :global(th),
    .card-container :global(td) {
        border: 1px solid #ddd;
        padding: 8px;
        text-align: left;
    }

    .card-container :global(th) {
        background: #f5f5f5;
        font-weight: bold;
    }
</style>
