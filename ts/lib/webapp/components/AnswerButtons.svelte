<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    const buttons = [
        { label: "Again", rating: 0, color: "#e74c3c", key: "1" },
        { label: "Hard", rating: 1, color: "#e67e22", key: "2" },
        { label: "Good", rating: 2, color: "#27ae60", key: "3" },
        { label: "Easy", rating: 3, color: "#3498db", key: "4" },
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
            <span class="text-lg">{button.label}</span>
            <span class="text-sm opacity-80 bg-black/20 px-2 py-1 rounded">{button.key}</span>
        </button>
    {/each}
</div>
