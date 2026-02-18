<script lang="ts">
    export let tags: string[] = [];

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    let inputValue = "";

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            addTag();
        } else if (event.key === "Backspace" && inputValue === "" && tags.length > 0) {
            removeTag(tags.length - 1);
        }
    }

    function addTag() {
        const tag = inputValue.trim();
        if (tag && !tags.includes(tag)) {
            tags = [...tags, tag];
            dispatch("change", tags);
        }
        inputValue = "";
    }

    function removeTag(index: number) {
        tags = tags.filter((_, i) => i !== index);
        dispatch("change", tags);
    }
</script>

<div class="mb-4">
    <label
        for="tag-input-field"
        class="block font-semibold text-gray-800 dark:text-gray-200 mb-1.5 text-sm"
    >
        Tags
    </label>
    <div
        class="flex flex-wrap gap-1.5 p-2 border-2 border-gray-300 dark:border-gray-600 rounded-lg min-h-[42px] items-center transition-colors duration-200 focus-within:border-indigo-500 focus-within:ring-2 focus-within:ring-indigo-500/20 bg-white dark:bg-gray-700"
    >
        {#each tags as tag, index}
            <span
                class="inline-flex items-center gap-1 bg-indigo-100 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400 px-2 py-1 rounded-full text-sm font-medium"
            >
                {tag}
                <button
                    type="button"
                    class="bg-transparent border-none text-indigo-700 dark:text-indigo-400 text-lg leading-none cursor-pointer p-0 w-4 h-4 flex items-center justify-center rounded-full hover:bg-indigo-200 dark:hover:bg-indigo-800/50 transition-colors"
                    on:click={() => removeTag(index)}
                    aria-label="Remove tag"
                >
                    &times;
                </button>
            </span>
        {/each}
        <input
            type="text"
            id="tag-input-field"
            bind:value={inputValue}
            on:keydown={handleKeydown}
            on:blur={addTag}
            placeholder="Add tags (press Enter or Space)"
            class="flex-1 border-none outline-hidden text-sm p-1 min-w-[120px] bg-transparent dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500"
        />
    </div>
</div>
