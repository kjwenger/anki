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

<div class="field-editor">
    <label for="field-{index}">{label}</label>
    <textarea
        id="field-{index}"
        {value}
        on:input={handleInput}
        rows="3"
        placeholder="Enter {label}"
    ></textarea>
</div>

<style>
    .field-editor {
        margin-bottom: 16px;
    }

    label {
        display: block;
        font-weight: 600;
        color: #333;
        margin-bottom: 6px;
        font-size: 14px;
    }

    textarea {
        width: 100%;
        padding: 10px;
        border: 2px solid #ddd;
        border-radius: 4px;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
        font-size: 14px;
        line-height: 1.5;
        resize: vertical;
        transition: border-color 0.2s;
    }

    textarea:focus {
        outline: none;
        border-color: #0a84ff;
    }

    textarea::placeholder {
        color: #999;
    }
</style>
