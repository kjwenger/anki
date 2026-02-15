<script lang="ts">
    import { currentUser } from "$lib/webapp/stores/auth";
    import { goto } from "$app/navigation";

    function handleProfile() {
        goto("/webapp/auth/profile");
    }

    function handleCollections() {
        goto("/webapp/collections");
    }

    function handleDecks() {
        goto("/webapp/decks");
    }
</script>

<div class="dashboard-container">
    <header class="dashboard-header">
        <h1>Anki Web Dashboard</h1>
        <div class="user-section">
            {#if $currentUser}
                <span class="welcome">Welcome, {$currentUser.username}!</span>
                <button class="btn-profile" on:click={handleProfile}>
                    Profile
                </button>
            {/if}
        </div>
    </header>

    <main class="dashboard-main">
        <div class="dashboard-grid">
            <div class="card">
                <h2>Collections</h2>
                <p>Manage your flashcard collections</p>
                <button class="btn-primary" on:click={handleCollections}>View Collections</button>
            </div>

            <div class="card">
                <h2>Study</h2>
                <p>Start your study session</p>
                <button class="btn-primary" on:click={handleDecks}>Browse Decks</button>
            </div>

            <div class="card">
                <h2>Statistics</h2>
                <p>View your learning progress</p>
                <button class="btn-primary">View Stats</button>
            </div>

            <div class="card">
                <h2>Settings</h2>
                <p>Configure your preferences</p>
                <button class="btn-primary">Settings</button>
            </div>
        </div>

        <div class="info-box">
            <h3>🎉 Welcome to Anki Web!</h3>
            <p>
                This is your central hub for managing flashcards, studying, and tracking your progress.
            </p>
            <p>
                Phase 3.1 (Authentication UI) is now complete! More features coming soon.
            </p>
        </div>
    </main>
</div>

<style>
    .dashboard-container {
        min-height: 100vh;
        background: #f5f5f5;
    }

    .dashboard-header {
        background: white;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        padding: 1rem 2rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    h1 {
        margin: 0;
        font-size: 1.5rem;
        color: #333;
    }

    .user-section {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .welcome {
        color: #666;
        font-size: 0.9rem;
    }

    .btn-profile {
        padding: 0.5rem 1rem;
        background: #667eea;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.9rem;
        transition: background 0.2s;
    }

    .btn-profile:hover {
        background: #5568d3;
    }

    .dashboard-main {
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    }

    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .card {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        padding: 1.5rem;
        transition: transform 0.2s, box-shadow 0.2s;
    }

    .card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .card h2 {
        margin: 0 0 0.5rem 0;
        font-size: 1.25rem;
        color: #333;
    }

    .card p {
        margin: 0 0 1rem 0;
        color: #666;
        font-size: 0.9rem;
    }

    .btn-primary {
        width: 100%;
        padding: 0.75rem;
        background: #667eea;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: background 0.2s;
    }

    .btn-primary:hover {
        background: #5568d3;
    }

    .info-box {
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        padding: 1.5rem;
        border-left: 4px solid #667eea;
    }

    .info-box h3 {
        margin: 0 0 1rem 0;
        color: #333;
    }

    .info-box p {
        margin: 0.5rem 0;
        color: #666;
        line-height: 1.6;
    }
</style>
