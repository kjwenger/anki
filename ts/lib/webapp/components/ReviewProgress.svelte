<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { reviewerStore } from "$lib/webapp/stores/reviewer";

    $: counts = $reviewerStore.currentCard?.counts || {
        new: 0,
        learning: 0,
        review: 0,
    };
    $: total = counts.new + counts.learning + counts.review;
</script>

<div class="progress-container">
    <div class="progress-item new">
        <span class="count">{counts.new}</span>
        <span class="label">New</span>
    </div>
    <div class="progress-item learning">
        <span class="count">{counts.learning}</span>
        <span class="label">Learning</span>
    </div>
    <div class="progress-item review">
        <span class="count">{counts.review}</span>
        <span class="label">Review</span>
    </div>
    <div class="progress-item total">
        <span class="count">{total}</span>
        <span class="label">Total</span>
    </div>
</div>

<style>
    .progress-container {
        display: flex;
        gap: 20px;
        align-items: center;
    }

    .progress-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 8px 12px;
        border-radius: 6px;
        min-width: 60px;
    }

    .progress-item.new {
        background: #e3f2fd;
        color: #1976d2;
    }

    .progress-item.learning {
        background: #fff3e0;
        color: #f57c00;
    }

    .progress-item.review {
        background: #e8f5e9;
        color: #388e3c;
    }

    .progress-item.total {
        background: #f5f5f5;
        color: #666;
        font-weight: 600;
    }

    .count {
        font-size: 24px;
        font-weight: 700;
        line-height: 1;
    }

    .label {
        font-size: 11px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        margin-top: 4px;
    }

    @media (max-width: 480px) {
        .progress-container {
            gap: 8px;
        }

        .progress-item {
            min-width: 50px;
            padding: 6px 8px;
        }

        .count {
            font-size: 20px;
        }

        .label {
            font-size: 10px;
        }
    }
</style>
