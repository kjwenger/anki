<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { reviewerStore } from "$lib/webapp/stores/reviewer";
    import { authStore } from "$lib/webapp/stores/auth";
    import { onMount, onDestroy } from "svelte";

    $: card = $reviewerStore.currentCard;
    $: showingAnswer = $reviewerStore.showingAnswer;
    $: token = $authStore?.token;

    let styleElement: HTMLStyleElement | null = null;
    let autoPlayAudio = true;

    onMount(() => {
        const stored = localStorage.getItem("anki-webapp-settings");
        if (stored) {
            try {
                const settings = JSON.parse(stored);
                autoPlayAudio = settings.autoPlayAudio ?? true;
            } catch (e) {
                console.error("Failed to load settings", e);
            }
        }
    });

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

    function processHtml(html: string, isAnswer: boolean = false): string {
        if (!html) return "";

        let firstAudio = true;

        // 1. Replace [sound:filename.mp3] with <audio> tags
        let processed = html.replace(/\[sound:(.+?)\]/g, (match, filename) => {
            const src = `/api/v1/media/${encodeURIComponent(filename)}?token=${token}`;
            const autoplay = autoPlayAudio && firstAudio ? "autoplay" : "";
            if (autoplay) firstAudio = false;

            return `<div class="anki-audio-wrapper my-4">
                <audio controls ${autoplay} class="w-full h-10 max-w-md mx-auto">
                    <source src="${src}" type="audio/mpeg">
                    Your browser does not support the audio element.
                </audio>
            </div>`;
        });

        // 2. Fix <img> tags: prefix src with media API path if it's just a filename
        processed = processed.replace(
            /<img([^>]+)src=["']([^"':]+)["']([^>]*)>/g,
            (match, prefix, filename, suffix) => {
                const src = `/api/v1/media/${encodeURIComponent(filename)}?token=${token}`;
                return `<img${prefix}src="${src}"${suffix}>`;
            },
        );

        return processed;
    }

    $: processedQuestion = card ? processHtml(card.question_html) : "";
    $: processedAnswer = card ? processHtml(card.answer_html, true) : "";
</script>

{#if card}
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-10 my-5 min-h-[300px]">
        <div class="text-xl leading-relaxed text-gray-800 dark:text-gray-200">
            <div class="mb-5">
                {@html processedQuestion}
            </div>

            {#if showingAnswer}
                <hr
                    class="my-8 border-none border-t-2 border-gray-200 dark:border-gray-600"
                />
                <div class="mt-5">
                    {@html processedAnswer}
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
        display: block;
        margin: 1rem auto;
        border-radius: 0.5rem;
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

    .bg-white :global(audio),
    .dark\:bg-gray-800 :global(audio) {
        filter: sepia(20%) saturate(70%) grayscale(100%) contrast(99%) invert(12%);
    }

    :global(.dark) .bg-white :global(audio),
    :global(.dark) .dark\:bg-gray-800 :global(audio) {
        filter: invert(100%) hue-rotate(180deg) brightness(1.5);
    }
</style>
