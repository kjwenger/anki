<script lang="ts">
    export let label: string;
    export let value: string;
    export let index: number;

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    function handleInput(event: Event) {
        const target = event.target as HTMLTextAreaElement;
        dispatch("change", { index, value: target.value });
    }
</script>

<div class="mb-4">
    <label
        for="field-{index}"
        class="block font-semibold text-gray-800 dark:text-gray-200 mb-1.5 text-sm"
    >
        {label}
    </label>
    <textarea
        id="field-{index}"
        {value}
        on:input={handleInput}
        rows="3"
        placeholder="Enter {label}"
        class="w-full p-3 border-2 border-gray-300 dark:border-gray-600 rounded-lg font-sans text-sm leading-relaxed resize-y transition-colors duration-200 focus:outline-hidden focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 bg-white dark:bg-gray-700 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500"
    ></textarea>
</div>
