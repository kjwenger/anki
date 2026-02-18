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
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-10 my-5 min-h-[300px]">
        <div class="text-xl leading-relaxed text-gray-800 dark:text-gray-200">
            <div class="mb-5">
                {@html card.question_html}
            </div>

            {#if showingAnswer}
                <hr
                    class="my-8 border-none border-t-2 border-gray-200 dark:border-gray-600"
                />
                <div class="mt-5">
                    {@html card.answer_html}
                </div>
            {/if}
        </div>
    </div>
{/if}

<style>
    /* Keep :global() styles for user-generated HTML that can't use Tailwind */
    .bg-white :global(img),
    .dark\:bg-gray-800 :global(img) {
        max-width: 100%;
        height: auto;
    }

    .bg-white :global(pre),
    .dark\:bg-gray-800 :global(pre) {
        background: #f5f5f5;
        padding: 15px;
        border-radius: 4px;
        overflow-x: auto;
    }

    :global(.dark) .bg-white :global(pre),
    :global(.dark) .dark\:bg-gray-800 :global(pre) {
        background: #1f2937;
    }

    .bg-white :global(code),
    .dark\:bg-gray-800 :global(code) {
        background: #f5f5f5;
        padding: 2px 6px;
        border-radius: 3px;
        font-family: "Courier New", monospace;
    }

    :global(.dark) .bg-white :global(code),
    :global(.dark) .dark\:bg-gray-800 :global(code) {
        background: #1f2937;
    }

    .bg-white :global(table),
    .dark\:bg-gray-800 :global(table) {
        border-collapse: collapse;
        width: 100%;
        margin: 15px 0;
    }

    .bg-white :global(th),
    .bg-white :global(td),
    .dark\:bg-gray-800 :global(th),
    .dark\:bg-gray-800 :global(td) {
        border: 1px solid #ddd;
        padding: 8px;
        text-align: left;
    }

    :global(.dark) .bg-white :global(th),
    :global(.dark) .bg-white :global(td),
    :global(.dark) .dark\:bg-gray-800 :global(th),
    :global(.dark) .dark\:bg-gray-800 :global(td) {
        border-color: #4b5563;
    }

    .bg-white :global(th),
    .dark\:bg-gray-800 :global(th) {
        background: #f5f5f5;
        font-weight: bold;
    }

    :global(.dark) .bg-white :global(th),
    :global(.dark) .dark\:bg-gray-800 :global(th) {
        background: #1f2937;
    }
</style>
