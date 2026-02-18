<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { createEventDispatcher } from "svelte";

    /** Interval descriptions from the backend.
     *  - `undefined` = still loading (show skeleton)
     *  - `null`      = failed to load (show buttons without intervals)
     *  - object      = loaded successfully
     */
    export let intervals:
        | { again: string; hard: string; good: string; easy: string }
        | null
        | undefined = undefined;

    const dispatch = createEventDispatcher();

    const buttons = [
        {
            label: "Again",
            rating: 0,
            color: "#e74c3c",
            key: "1",
            intervalKey: "again" as const,
        },
        {
            label: "Hard",
            rating: 1,
            color: "#e67e22",
            key: "2",
            intervalKey: "hard" as const,
        },
        {
            label: "Good",
            rating: 2,
            color: "#27ae60",
            key: "3",
            intervalKey: "good" as const,
        },
        {
            label: "Easy",
            rating: 3,
            color: "#3498db",
            key: "4",
            intervalKey: "easy" as const,
        },
    ];

    function handleAnswer(rating: number) {
        dispatch("answer", rating);
    }
</script>

<div class="flex gap-3 justify-center mt-10 py-5 flex-wrap">
    {#each buttons as button}
        <button
            class="flex-1 max-w-[200px] p-5 border-none rounded-lg text-white text-lg font-semibold cursor-pointer transition-all duration-200 shadow-md hover:-translate-y-0.5 hover:shadow-lg active:translate-y-0 active:shadow-md flex flex-col items-center gap-2"
            style="background-color: {button.color}"
            on:click={() => handleAnswer(button.rating)}
            title="{button.label} ({button.key})"
        >
            {#if intervals === undefined}
                <!-- Loading skeleton while intervals are being fetched -->
                <span class="w-12 h-4 rounded bg-white/30 animate-pulse"></span>
            {:else if intervals !== null}
                <span class="text-sm font-normal opacity-90">
                    {intervals[button.intervalKey]}
                </span>
            {/if}
            <span class="text-lg">{button.label}</span>
            <span class="text-sm opacity-80 bg-black/20 px-2 py-1 rounded">
                {button.key}
            </span>
        </button>
    {/each}
</div>
