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

<div class="flex gap-5 items-center">
    <div
        class="flex flex-col items-center px-3 py-2 rounded-lg min-w-[60px] bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400"
    >
        <span class="text-2xl font-bold leading-none">{counts.new}</span>
        <span class="text-[11px] font-semibold uppercase tracking-wider mt-1">New</span>
    </div>
    <div
        class="flex flex-col items-center px-3 py-2 rounded-lg min-w-[60px] bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400"
    >
        <span class="text-2xl font-bold leading-none">{counts.learning}</span>
        <span class="text-[11px] font-semibold uppercase tracking-wider mt-1">
            Learning
        </span>
    </div>
    <div
        class="flex flex-col items-center px-3 py-2 rounded-lg min-w-[60px] bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400"
    >
        <span class="text-2xl font-bold leading-none">{counts.review}</span>
        <span class="text-[11px] font-semibold uppercase tracking-wider mt-1">
            Review
        </span>
    </div>
    <div
        class="flex flex-col items-center px-3 py-2 rounded-lg min-w-[60px] bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 font-semibold"
    >
        <span class="text-2xl font-bold leading-none">{total}</span>
        <span class="text-[11px] font-semibold uppercase tracking-wider mt-1">
            Total
        </span>
    </div>
</div>
