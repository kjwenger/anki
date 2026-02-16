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

<div class="tag-input-container">
    <label for="tag-input-field">Tags</label>
    <div class="tag-input">
        {#each tags as tag, index}
            <span class="tag">
                {tag}
                <button
                    type="button"
                    class="tag-remove"
                    on:click={() => removeTag(index)}
                    aria-label="Remove tag"
                >
                    ×
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
            class="tag-input-field"
        />
    </div>
</div>

<style>
    .tag-input-container {
        margin-bottom: 16px;
    }

    label {
        display: block;
        font-weight: 600;
        color: #333;
        margin-bottom: 6px;
        font-size: 14px;
    }

    .tag-input {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
        padding: 8px;
        border: 2px solid #ddd;
        border-radius: 4px;
        min-height: 42px;
        align-items: center;
        transition: border-color 0.2s;
    }

    .tag-input:focus-within {
        border-color: #0a84ff;
    }

    .tag {
        display: inline-flex;
        align-items: center;
        gap: 4px;
        background: #e8f4fd;
        color: #0066cc;
        padding: 4px 8px;
        border-radius: 12px;
        font-size: 13px;
        font-weight: 500;
    }

    .tag-remove {
        background: none;
        border: none;
        color: #0066cc;
        font-size: 18px;
        line-height: 1;
        cursor: pointer;
        padding: 0;
        width: 16px;
        height: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        transition: background 0.2s;
    }

    .tag-remove:hover {
        background: rgba(0, 102, 204, 0.15);
    }

    .tag-input-field {
        flex: 1;
        border: none;
        outline: none;
        font-size: 14px;
        padding: 4px;
        min-width: 120px;
    }

    .tag-input-field::placeholder {
        color: #999;
    }
</style>
