<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { onMount } from "svelte";
    import { api } from "$lib/webapp/api/client";

    let loading = true;
    let error = "";
    let todayStats: any = null;
    let collectionStats: any = null;

    onMount(async () => {
        await loadStats();
    });

    async function loadStats() {
        loading = true;
        error = "";
        try {
            const [today, collection] = await Promise.all([
                api.getTodayStats(),
                api.getCollectionStats(),
            ]);
            todayStats = today;
            collectionStats = collection;
        } catch (e: any) {
            error = e.message || "Failed to load statistics";
        } finally {
            loading = false;
        }
    }

    function formatTime(millis: number): string {
        const seconds = Math.floor(millis / 1000);
        const minutes = Math.floor(seconds / 60);
        const hours = Math.floor(minutes / 60);

        if (hours > 0) {
            return `${hours}h ${minutes % 60}m`;
        } else if (minutes > 0) {
            return `${minutes}m ${seconds % 60}s`;
        } else {
            return `${seconds}s`;
        }
    }

    function formatPercent(numerator: number, denominator: number): string {
        if (denominator === 0) return "0%";
        return `${Math.round((numerator / denominator) * 100)}%`;
    }
</script>

<div class="stats-page">
    <header class="page-header">
        <div class="header-content">
            <h1>Statistics</h1>
            <div class="header-actions">
                <a href="/webapp/decks" class="btn-secondary"> ← Back </a>
            </div>
        </div>
    </header>

    <main class="page-content">
        {#if error}
            <div class="error-banner">{error}</div>
        {/if}

        {#if loading}
            <div class="loading">Loading statistics...</div>
        {:else}
            <div class="stats-grid">
                <section class="stats-card">
                    <h2>Today's Stats</h2>
                    <div class="stats-row">
                        <div class="stat-item primary">
                            <div class="stat-value">
                                {todayStats?.answer_count || 0}
                            </div>
                            <div class="stat-label">Cards Answered</div>
                        </div>
                        <div class="stat-item success">
                            <div class="stat-value">
                                {todayStats?.correct_count || 0}
                            </div>
                            <div class="stat-label">Correct</div>
                        </div>
                        <div class="stat-item info">
                            <div class="stat-value">
                                {formatTime(todayStats?.answer_millis || 0)}
                            </div>
                            <div class="stat-label">Study Time</div>
                        </div>
                    </div>

                    {#if todayStats && todayStats.answer_count > 0}
                        <div class="stats-detail">
                            <div class="detail-row">
                                <span>Accuracy:</span>
                                <strong
                                    >{formatPercent(
                                        todayStats.correct_count,
                                        todayStats.answer_count,
                                    )}</strong
                                >
                            </div>
                            <div class="detail-row">
                                <span>Learn:</span>
                                <strong>{todayStats.learn_count}</strong>
                            </div>
                            <div class="detail-row">
                                <span>Review:</span>
                                <strong>{todayStats.review_count}</strong>
                            </div>
                            <div class="detail-row">
                                <span>Relearn:</span>
                                <strong>{todayStats.relearn_count}</strong>
                            </div>
                        </div>
                    {/if}
                </section>

                <section class="stats-card">
                    <h2>Collection Overview</h2>
                    <div class="stats-row">
                        <div class="stat-item">
                            <div class="stat-value">
                                {collectionStats?.total_cards || 0}
                            </div>
                            <div class="stat-label">Total Cards</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-value">
                                {collectionStats?.total_notes || 0}
                            </div>
                            <div class="stat-label">Total Notes</div>
                        </div>
                    </div>

                    <div class="stats-detail">
                        <div class="detail-row">
                            <span>New:</span>
                            <strong
                                >{collectionStats?.new_cards || 0}</strong
                            >
                        </div>
                        <div class="detail-row">
                            <span>Young:</span>
                            <strong
                                >{collectionStats?.young_cards || 0}</strong
                            >
                        </div>
                        <div class="detail-row">
                            <span>Mature:</span>
                            <strong
                                >{collectionStats?.mature_cards || 0}</strong
                            >
                        </div>
                        <div class="detail-row">
                            <span>Suspended:</span>
                            <strong
                                >{collectionStats?.suspended_cards || 0}</strong
                            >
                        </div>
                        <div class="detail-row">
                            <span>Buried:</span>
                            <strong
                                >{collectionStats?.buried_cards || 0}</strong
                            >
                        </div>
                    </div>
                </section>

                {#if todayStats && todayStats.mature_count > 0}
                    <section class="stats-card">
                        <h2>Mature Cards</h2>
                        <div class="stats-row">
                            <div class="stat-item">
                                <div class="stat-value">
                                    {todayStats.mature_count}
                                </div>
                                <div class="stat-label">Reviewed</div>
                            </div>
                            <div class="stat-item success">
                                <div class="stat-value">
                                    {formatPercent(
                                        todayStats.mature_correct,
                                        todayStats.mature_count,
                                    )}
                                </div>
                                <div class="stat-label">Retention</div>
                            </div>
                        </div>
                    </section>
                {/if}

                <section class="stats-card info-card">
                    <h2>📊 Statistics</h2>
                    <p>
                        Track your learning progress with detailed statistics.
                    </p>
                    <p>
                        Study consistently to improve retention and master your
                        flashcards!
                    </p>
                </section>
            </div>
        {/if}
    </main>
</div>

<style>
    .stats-page {
        min-height: 100vh;
        background: #f5f5f5;
    }

    .page-header {
        background: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        padding: 1.5rem 2rem;
    }

    .header-content {
        max-width: 1200px;
        margin: 0 auto;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    h1 {
        margin: 0;
        font-size: 1.75rem;
        color: #333;
    }

    .page-content {
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem;
    }

    .error-banner {
        background: #fee;
        border: 1px solid #fcc;
        border-radius: 8px;
        color: #c33;
        padding: 1rem;
        margin-bottom: 1.5rem;
    }

    .loading {
        text-align: center;
        padding: 4rem;
        color: #666;
        font-size: 18px;
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 1.5rem;
    }

    .stats-card {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        padding: 1.5rem;
    }

    .stats-card h2 {
        margin: 0 0 1.5rem 0;
        font-size: 1.25rem;
        color: #333;
        padding-bottom: 0.75rem;
        border-bottom: 2px solid #f0f0f0;
    }

    .stats-row {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .stat-item {
        text-align: center;
        padding: 1rem;
        border-radius: 6px;
        background: #f9f9f9;
    }

    .stat-item.primary {
        background: #e3f2fd;
        color: #1976d2;
    }

    .stat-item.success {
        background: #e8f5e9;
        color: #388e3c;
    }

    .stat-item.info {
        background: #fff3e0;
        color: #f57c00;
    }

    .stat-value {
        font-size: 2rem;
        font-weight: 700;
        line-height: 1;
        margin-bottom: 0.5rem;
    }

    .stat-label {
        font-size: 0.875rem;
        font-weight: 500;
        opacity: 0.8;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .stats-detail {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .detail-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem 0;
        border-bottom: 1px solid #f0f0f0;
    }

    .detail-row:last-child {
        border-bottom: none;
    }

    .detail-row span {
        color: #666;
        font-size: 0.95rem;
    }

    .detail-row strong {
        color: #333;
        font-weight: 600;
    }

    .info-card {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
    }

    .info-card h2 {
        color: white;
        border-color: rgba(255, 255, 255, 0.3);
    }

    .info-card p {
        margin: 0.75rem 0;
        line-height: 1.6;
        opacity: 0.95;
    }

    .btn-secondary {
        padding: 0.5rem 1rem;
        background: #f0f0f0;
        color: #333;
        text-decoration: none;
        border-radius: 4px;
        display: inline-block;
    }

    .btn-secondary:hover {
        background: #e0e0e0;
    }

    @media (max-width: 768px) {
        .stats-grid {
            grid-template-columns: 1fr;
        }

        .stats-row {
            grid-template-columns: 1fr;
        }

        .stat-value {
            font-size: 1.5rem;
        }
    }
</style>
