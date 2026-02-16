<script lang="ts">
    // Copyright: Ankitects Pty Ltd and contributors
    // License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

    import { onMount } from "svelte";

    let theme: "light" | "dark" = "light";
    let cardsPerDay = 20;
    let reviewsPerDay = 200;
    let showAnswerTime = true;
    let autoPlayAudio = true;
    let enableKeyboardShortcuts = true;
    let saved = false;

    onMount(() => {
        loadSettings();
    });

    function loadSettings() {
        const stored = localStorage.getItem("anki-webapp-settings");
        if (stored) {
            try {
                const settings = JSON.parse(stored);
                theme = settings.theme || "light";
                cardsPerDay = settings.cardsPerDay || 20;
                reviewsPerDay = settings.reviewsPerDay || 200;
                showAnswerTime = settings.showAnswerTime ?? true;
                autoPlayAudio = settings.autoPlayAudio ?? true;
                enableKeyboardShortcuts = settings.enableKeyboardShortcuts ?? true;
            } catch (e) {
                console.error("Failed to load settings", e);
            }
        }
    }

    function saveSettings() {
        const settings = {
            theme,
            cardsPerDay,
            reviewsPerDay,
            showAnswerTime,
            autoPlayAudio,
            enableKeyboardShortcuts,
        };

        localStorage.setItem("anki-webapp-settings", JSON.stringify(settings));
        saved = true;

        if (theme === "dark") {
            document.documentElement.classList.add("dark-theme");
        } else {
            document.documentElement.classList.remove("dark-theme");
        }

        setTimeout(() => { saved = false; }, 2000);
    }

    function resetSettings() {
        if (confirm("Reset all settings to defaults?")) {
            theme = "light";
            cardsPerDay = 20;
            reviewsPerDay = 200;
            showAnswerTime = true;
            autoPlayAudio = true;
            enableKeyboardShortcuts = true;
            saveSettings();
        }
    }
</script>

<div class="settings-page">
    <header class="page-header">
        <div class="header-content">
            <h1>Settings</h1>
            <div class="header-actions">
                <a href="/webapp/decks" class="btn-secondary"> ← Back </a>
            </div>
        </div>
    </header>

    <main class="page-content">
        {#if saved}
            <div class="success-banner">✓ Settings saved successfully!</div>
        {/if}

        <div class="settings-container">
            <section class="settings-section">
                <h2>Appearance</h2>
                <div class="setting-item">
                    <label for="theme">Theme</label>
                    <select id="theme" bind:value={theme}>
                        <option value="light">Light</option>
                        <option value="dark">Dark</option>
                    </select>
                    <p class="setting-description">Choose between light and dark color schemes</p>
                </div>
            </section>

            <section class="settings-section">
                <h2>Study Limits</h2>
                <div class="setting-item">
                    <label for="cards-per-day">New Cards Per Day</label>
                    <input type="number" id="cards-per-day" bind:value={cardsPerDay} min="0" max="999" />
                    <p class="setting-description">Maximum new cards to introduce per day</p>
                </div>

                <div class="setting-item">
                    <label for="reviews-per-day">Maximum Reviews Per Day</label>
                    <input type="number" id="reviews-per-day" bind:value={reviewsPerDay} min="0" max="9999" />
                    <p class="setting-description">Maximum review cards to show per day</p>
                </div>
            </section>

            <section class="settings-section">
                <h2>Study Interface</h2>
                <div class="setting-item checkbox-item">
                    <label>
                        <input type="checkbox" bind:checked={showAnswerTime} />
                        <span>Show Answer Time</span>
                    </label>
                    <p class="setting-description">Display how long you took to answer each card</p>
                </div>

                <div class="setting-item checkbox-item">
                    <label>
                        <input type="checkbox" bind:checked={autoPlayAudio} />
                        <span>Auto-play Audio</span>
                    </label>
                    <p class="setting-description">Automatically play audio when showing cards</p>
                </div>

                <div class="setting-item checkbox-item">
                    <label>
                        <input type="checkbox" bind:checked={enableKeyboardShortcuts} />
                        <span>Enable Keyboard Shortcuts</span>
                    </label>
                    <p class="setting-description">Use keyboard to answer cards (1-4, Space, etc.)</p>
                </div>
            </section>

            <section class="settings-section info-section">
                <h2>💡 About Settings</h2>
                <p>Settings are stored locally in your browser. They will persist across sessions but are not synced between devices.</p>
                <p>Some settings (like study limits) may require reloading the study interface to take effect.</p>
            </section>

            <div class="settings-actions">
                <button class="btn-primary" on:click={saveSettings}>Save Settings</button>
                <button class="btn-secondary" on:click={resetSettings}>Reset to Defaults</button>
            </div>
        </div>
    </main>
</div>

<style>
    .settings-page { min-height: 100vh; background: #f5f5f5; }
    .page-header { background: white; box-shadow: 0 2px 4px rgba(0,0,0,0.1); padding: 1.5rem 2rem; }
    .header-content { max-width: 900px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center; }
    h1 { margin: 0; font-size: 1.75rem; color: #333; }
    .page-content { max-width: 900px; margin: 0 auto; padding: 2rem; }
    .success-banner { background: #e8f5e9; border: 1px solid #81c784; border-radius: 8px; color: #2e7d32; padding: 1rem; margin-bottom: 1.5rem; text-align: center; font-weight: 500; }
    .settings-container { display: flex; flex-direction: column; gap: 1.5rem; }
    .settings-section { background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); padding: 2rem; }
    .settings-section h2 { margin: 0 0 1.5rem 0; font-size: 1.25rem; color: #333; padding-bottom: 0.75rem; border-bottom: 2px solid #f0f0f0; }
    .setting-item { margin-bottom: 1.5rem; }
    .setting-item:last-child { margin-bottom: 0; }
    .setting-item label { display: block; font-weight: 600; color: #333; margin-bottom: 0.5rem; font-size: 14px; }
    .checkbox-item label { display: flex; align-items: center; gap: 0.75rem; cursor: pointer; }
    .checkbox-item input[type="checkbox"] { width: 20px; height: 20px; cursor: pointer; }
    .checkbox-item span { font-weight: 600; color: #333; }
    input[type="number"], select { width: 100%; max-width: 300px; padding: 0.75rem; border: 2px solid #ddd; border-radius: 4px; font-size: 14px; background: white; }
    input[type="number"]:focus, select:focus { outline: none; border-color: #0a84ff; }
    .setting-description { margin: 0.5rem 0 0 0; color: #666; font-size: 13px; line-height: 1.5; }
    .info-section { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; }
    .info-section h2 { color: white; border-color: rgba(255,255,255,0.3); }
    .info-section p { margin: 0.75rem 0; line-height: 1.6; opacity: 0.95; }
    .settings-actions { display: flex; gap: 1rem; padding: 1.5rem; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }
    .btn-primary, .btn-secondary { padding: 0.75rem 2rem; border: none; border-radius: 4px; font-size: 16px; font-weight: 500; cursor: pointer; transition: background 0.2s; }
    .btn-primary { background: #0a84ff; color: white; }
    .btn-primary:hover { background: #0066cc; }
    .btn-secondary { background: #f0f0f0; color: #333; text-decoration: none; display: inline-block; }
    .btn-secondary:hover { background: #e0e0e0; }
    @media (max-width: 768px) { .settings-actions { flex-direction: column; } .btn-primary, .btn-secondary { width: 100%; } }
</style>
