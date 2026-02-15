<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let show = false;
    
    let name = "";
    let error = "";
    let loading = false;

    const dispatch = createEventDispatcher();

    function handleClose() {
        name = "";
        error = "";
        show = false;
        dispatch("close");
    }

    function handleSubmit() {
        if (!name.trim()) {
            error = "Collection name is required";
            return;
        }

        if (name.length < 2) {
            error = "Collection name must be at least 2 characters";
            return;
        }

        dispatch("create", { name: name.trim() });
    }

    function handleKeyPress(event: KeyboardEvent) {
        if (event.key === "Enter") {
            handleSubmit();
        } else if (event.key === "Escape") {
            handleClose();
        }
    }

    $: if (show) {
        name = "";
        error = "";
        loading = false;
    }
</script>

{#if show}
    <div class="modal-overlay" on:click={handleClose} role="presentation">
        <div class="modal-content" on:click|stopPropagation role="dialog" aria-modal="true">
            <div class="modal-header">
                <h2>Create New Collection</h2>
                <button class="close-button" on:click={handleClose} aria-label="Close">
                    ×
                </button>
            </div>

            <div class="modal-body">
                {#if error}
                    <div class="error-message" role="alert">
                        {error}
                    </div>
                {/if}

                <div class="form-group">
                    <label for="collection-name">Collection Name</label>
                    <input
                        id="collection-name"
                        type="text"
                        bind:value={name}
                        on:keydown={handleKeyPress}
                        placeholder="Enter collection name"
                        disabled={loading}
                        autofocus
                    />
                </div>
            </div>

            <div class="modal-footer">
                <button class="btn-secondary" on:click={handleClose} disabled={loading}>
                    Cancel
                </button>
                <button class="btn-primary" on:click={handleSubmit} disabled={loading}>
                    {loading ? "Creating..." : "Create"}
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .modal-content {
        background: white;
        border-radius: 8px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
        width: 90%;
        max-width: 500px;
        max-height: 90vh;
        overflow: auto;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem;
        border-bottom: 1px solid #eee;
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.25rem;
        color: #333;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 2rem;
        color: #999;
        cursor: pointer;
        padding: 0;
        width: 2rem;
        height: 2rem;
        display: flex;
        align-items: center;
        justify-content: center;
        line-height: 1;
    }

    .close-button:hover {
        color: #666;
    }

    .modal-body {
        padding: 1.5rem;
    }

    .error-message {
        background: #fee;
        border: 1px solid #fcc;
        border-radius: 4px;
        color: #c33;
        padding: 0.75rem;
        margin-bottom: 1rem;
        font-size: 0.9rem;
    }

    .form-group {
        margin-bottom: 1rem;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
        color: #333;
        font-weight: 500;
        font-size: 0.9rem;
    }

    input {
        width: 100%;
        padding: 0.75rem;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 1rem;
        transition: border-color 0.2s;
        box-sizing: border-box;
    }

    input:focus {
        outline: none;
        border-color: #667eea;
    }

    input:disabled {
        background: #f5f5f5;
        cursor: not-allowed;
    }

    .modal-footer {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
        padding: 1.5rem;
        border-top: 1px solid #eee;
    }

    button {
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 4px;
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-primary {
        background: #667eea;
        color: white;
    }

    .btn-primary:hover:not(:disabled) {
        background: #5568d3;
    }

    .btn-secondary {
        background: #f0f0f0;
        color: #333;
    }

    .btn-secondary:hover:not(:disabled) {
        background: #e0e0e0;
    }

    button:disabled {
        background: #ccc;
        cursor: not-allowed;
    }
</style>
