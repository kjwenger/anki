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

<div class="answer-buttons">
    {#each buttons as button}
        <button
            class="answer-btn"
            style="background-color: {button.color}"
            on:click={() => handleAnswer(button.rating)}
            title="{button.label} ({button.key})"
        >
            <span class="label">{button.label}</span>
            <span class="key">{button.key}</span>
        </button>
    {/each}
</div>

<style>
    .answer-buttons {
        display: flex;
        gap: 12px;
        justify-content: center;
        margin-top: 40px;
        padding: 20px 0;
    }

    .answer-btn {
        flex: 1;
        max-width: 200px;
        padding: 20px;
        border: none;
        border-radius: 8px;
        color: white;
        font-size: 18px;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
    }

    .answer-btn:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
    }

    .answer-btn:active {
        transform: translateY(0);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    .label {
        font-size: 18px;
    }

    .key {
        font-size: 14px;
        opacity: 0.8;
        background: rgba(0, 0, 0, 0.2);
        padding: 4px 8px;
        border-radius: 4px;
    }

    @media (max-width: 768px) {
        .answer-buttons {
            flex-wrap: wrap;
        }

        .answer-btn {
            max-width: calc(50% - 6px);
        }
    }

    @media (max-width: 480px) {
        .answer-btn {
            max-width: 100%;
        }
    }
</style>
