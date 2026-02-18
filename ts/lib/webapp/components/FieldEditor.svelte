<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    export let label: string;
    export let value: string;
    export let index: number;
    export let isCloze: boolean = false;
    export let isSticky: boolean = false;

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    let textarea: HTMLTextAreaElement;

    function handleInput(event: Event) {
        const target = event.target as HTMLTextAreaElement;
        dispatch("change", { index, value: target.value });
    }

    function toggleSticky() {
        dispatch("stickyChange", { index, sticky: !isSticky });
    }

    function insertCloze() {
        if (!textarea) return;

        const start = textarea.selectionStart;
        const end = textarea.selectionEnd;
        const text = textarea.value;

        // Find the next cloze index
        const clozes = text.match(/\{\{c(\d+)::/g) || [];
        const maxCloze = clozes.reduce((max, c) => {
            const num = parseInt(c.match(/\d+/)![0]);
            return Math.max(max, num);
        }, 0);
        const nextCloze = maxCloze + 1;

        const selectedText = text.substring(start, end);
        const newText =
            text.substring(0, start) +
            `{{c${nextCloze}::${selectedText}}}` +
            text.substring(end);

        dispatch("change", { index, value: newText });

        // Update value immediately to set selection
        value = newText;

        // Restore focus and selection
        setTimeout(() => {
            textarea.focus();
            if (selectedText.length > 0) {
                // Keep the original text selected inside the cloze
                const newStart = start + `{{c${nextCloze}::`.length;
                textarea.setSelectionRange(newStart, newStart + selectedText.length);
            } else {
                // Place cursor between :: and }}
                const newPos = start + `{{c${nextCloze}::`.length;
                textarea.setSelectionRange(newPos, newPos);
            }
        }, 0);
    }

    function handleKeydown(event: KeyboardEvent) {
        // Ctrl+Shift+C for cloze deletion
        if (event.ctrlKey && event.shiftKey && event.key.toLowerCase() === "c") {
            event.preventDefault();
            insertCloze();
        }
    }
</script>

<div class="mb-6">
    <div class="flex justify-between items-center mb-1.5">
        <label
            for="field-{index}"
            class="block font-semibold text-gray-800 dark:text-gray-200 text-sm"
        >
            {label}
        </label>
        <div class="flex gap-1.5 items-center">
            <button
                type="button"
                on:click={toggleSticky}
                class="p-1 rounded transition-colors duration-200 {isSticky
                    ? 'text-indigo-500 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 border border-indigo-200 dark:border-indigo-800'
                    : 'text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-300 border border-transparent'}"
                title={isSticky ? "Pinned (retains value)" : "Pin field"}
                aria-label={isSticky ? "Unpin field" : "Pin field"}
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <line x1="12" y1="17" x2="12" y2="22"></line>
                    <path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z"></path>
                </svg>
            </button>
            {#if isCloze}
                <button
                    type="button"
                    on:click={insertCloze}
                    class="px-2 py-1 bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded text-xs font-mono text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                    title="Insert cloze deletion (Ctrl+Shift+C)"
                >
                    [c1]
                </button>
            {/if}
        </div>
    </div>
    <textarea
        id="field-{index}"
        bind:this={textarea}
        {value}
        on:input={handleInput}
        on:keydown={handleKeydown}
        rows="3"
        placeholder="Enter {label}"
        class="w-full p-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg font-sans text-sm leading-relaxed resize-y transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 bg-white dark:bg-gray-700 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500"
    ></textarea>
</div>
