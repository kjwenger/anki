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

    // PWA Install logic
    let installPrompt: any = null;
    let isInstallable = false;

    onMount(() => {
        loadSettings();

        window.addEventListener("beforeinstallprompt", (e) => {
            // Prevent Chrome 67 and earlier from automatically showing the prompt
            e.preventDefault();
            // Stash the event so it can be triggered later.
            installPrompt = e;
            isInstallable = true;
        });

        window.addEventListener("appinstalled", () => {
            isInstallable = false;
            installPrompt = null;
            console.log("PWA was installed");
        });
    });

    async function handleInstallClick() {
        if (!installPrompt) return;
        
        // Show the install prompt
        installPrompt.prompt();
        
        // Wait for the user to respond to the prompt
        const { outcome } = await installPrompt.userChoice;
        console.log(`User response to the install prompt: ${outcome}`);
        
        // We've used the prompt, and can't use it again
        installPrompt = null;
        isInstallable = false;
    }

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
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }

        setTimeout(() => {
            saved = false;
        }, 2000);
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

<div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-xs py-6 px-8">
        <div class="max-w-[900px] mx-auto flex justify-between items-center">
            <h1 class="m-0 text-2xl text-gray-800 dark:text-gray-100 font-bold">
                Settings
            </h1>
            <a
                href="/webapp"
                class="px-5 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 no-underline rounded-lg text-sm font-medium hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
            >
                ← Back
            </a>
        </div>
    </header>

    <main class="max-w-[900px] mx-auto p-8">
        {#if saved}
            <div
                class="bg-green-50 dark:bg-green-900/20 border border-green-300 dark:border-green-800 rounded-lg text-green-700 dark:text-green-400 p-4 mb-6 text-center font-medium"
            >
                ✓ Settings saved successfully!
            </div>
        {/if}

        <div class="flex flex-col gap-6">
            <section class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-8">
                <h2
                    class="m-0 mb-6 text-xl text-gray-800 dark:text-gray-100 pb-3 border-b-2 border-gray-100 dark:border-gray-700"
                >
                    Appearance
                </h2>
                <div>
                    <label
                        for="theme"
                        class="block font-semibold text-gray-700 dark:text-gray-200 mb-2 text-sm"
                    >
                        Theme
                    </label>
                    <select
                        id="theme"
                        bind:value={theme}
                        class="w-full max-w-[300px] p-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg text-sm bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-hidden focus:border-indigo-500 dark:focus:border-indigo-400"
                    >
                        <option value="light">Light</option>
                        <option value="dark">Dark</option>
                    </select>
                    <p
                        class="mt-2 mb-0 text-gray-500 dark:text-gray-400 text-[13px] leading-relaxed"
                    >
                        Choose between light and dark color schemes
                    </p>
                </div>
            </section>

            <section class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-8">
                <h2
                    class="m-0 mb-6 text-xl text-gray-800 dark:text-gray-100 pb-3 border-b-2 border-gray-100 dark:border-gray-700"
                >
                    Study Limits
                </h2>
                <div class="mb-6">
                    <label
                        for="cards-per-day"
                        class="block font-semibold text-gray-700 dark:text-gray-200 mb-2 text-sm"
                    >
                        New Cards Per Day
                    </label>
                    <input
                        type="number"
                        id="cards-per-day"
                        bind:value={cardsPerDay}
                        min="0"
                        max="999"
                        class="w-full max-w-[300px] p-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg text-sm bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-hidden focus:border-indigo-500 dark:focus:border-indigo-400"
                    />
                    <p
                        class="mt-2 mb-0 text-gray-500 dark:text-gray-400 text-[13px] leading-relaxed"
                    >
                        Maximum new cards to introduce per day
                    </p>
                </div>

                <div>
                    <label
                        for="reviews-per-day"
                        class="block font-semibold text-gray-700 dark:text-gray-200 mb-2 text-sm"
                    >
                        Maximum Reviews Per Day
                    </label>
                    <input
                        type="number"
                        id="reviews-per-day"
                        bind:value={reviewsPerDay}
                        min="0"
                        max="9999"
                        class="w-full max-w-[300px] p-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg text-sm bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-hidden focus:border-indigo-500 dark:focus:border-indigo-400"
                    />
                    <p
                        class="mt-2 mb-0 text-gray-500 dark:text-gray-400 text-[13px] leading-relaxed"
                    >
                        Maximum review cards to show per day
                    </p>
                </div>
            </section>

            <section class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-8">
                <h2
                    class="m-0 mb-6 text-xl text-gray-800 dark:text-gray-100 pb-3 border-b-2 border-gray-100 dark:border-gray-700"
                >
                    Study Interface
                </h2>
                <div class="mb-5">
                    <label class="flex items-center gap-3 cursor-pointer">
                        <input
                            type="checkbox"
                            bind:checked={showAnswerTime}
                            class="w-5 h-5 cursor-pointer accent-indigo-500"
                        />
                        <span class="font-semibold text-gray-700 dark:text-gray-200">
                            Show Answer Time
                        </span>
                    </label>
                    <p
                        class="mt-2 mb-0 text-gray-500 dark:text-gray-400 text-[13px] leading-relaxed ml-8"
                    >
                        Display how long you took to answer each card
                    </p>
                </div>

                <div class="mb-5">
                    <label class="flex items-center gap-3 cursor-pointer">
                        <input
                            type="checkbox"
                            bind:checked={autoPlayAudio}
                            class="w-5 h-5 cursor-pointer accent-indigo-500"
                        />
                        <span class="font-semibold text-gray-700 dark:text-gray-200">
                            Auto-play Audio
                        </span>
                    </label>
                    <p
                        class="mt-2 mb-0 text-gray-500 dark:text-gray-400 text-[13px] leading-relaxed ml-8"
                    >
                        Automatically play audio when showing cards
                    </p>
                </div>

                <div>
                    <label class="flex items-center gap-3 cursor-pointer">
                        <input
                            type="checkbox"
                            bind:checked={enableKeyboardShortcuts}
                            class="w-5 h-5 cursor-pointer accent-indigo-500"
                        />
                        <span class="font-semibold text-gray-700 dark:text-gray-200">
                            Enable Keyboard Shortcuts
                        </span>
                    </label>
                    <p
                        class="mt-2 mb-0 text-gray-500 dark:text-gray-400 text-[13px] leading-relaxed ml-8"
                    >
                        Use keyboard to answer cards (1-4, Space, etc.)
                    </p>
                </div>
            </section>

            {#if isInstallable}
                <section class="bg-indigo-50 dark:bg-indigo-900/20 rounded-lg shadow-md p-8 border-2 border-indigo-200 dark:border-indigo-800">
                    <h2 class="m-0 mb-4 text-xl text-indigo-900 dark:text-indigo-100 font-bold flex items-center gap-2">
                        <span>📱</span> Install Anki Web
                    </h2>
                    <p class="text-indigo-800 dark:text-indigo-300 mb-6 leading-relaxed">
                        Install Anki Web on your device for a better experience, including a dedicated app icon and faster access.
                    </p>
                    <button
                        class="px-8 py-3 bg-indigo-600 hover:bg-indigo-700 text-white border-none rounded-lg text-base font-bold cursor-pointer transition-all shadow-md hover:shadow-lg active:scale-95"
                        on:click={handleInstallClick}
                    >
                        Install App
                    </button>
                </section>
            {/if}

            <section
                class="bg-gradient-to-br from-indigo-500 to-purple-600 text-white rounded-lg shadow-md p-8"
            >
                <h2 class="m-0 mb-6 text-xl text-white pb-3 border-b-2 border-white/30">
                    About Settings
                </h2>
                <p class="my-3 leading-relaxed opacity-95">
                    Settings are stored locally in your browser. They will persist
                    across sessions but are not synced between devices.
                </p>
                <p class="my-3 leading-relaxed opacity-95">
                    Some settings (like study limits) may require reloading the study
                    interface to take effect.
                </p>
            </section>

            <div class="flex gap-4 p-6 bg-white dark:bg-gray-800 rounded-lg shadow-md">
                <button
                    class="px-8 py-3 bg-indigo-500 hover:bg-indigo-600 text-white border-none rounded-lg text-base font-medium cursor-pointer transition-colors"
                    on:click={saveSettings}
                >
                    Save Settings
                </button>
                <button
                    class="px-8 py-3 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-200 border-none rounded-lg text-base font-medium cursor-pointer transition-colors"
                    on:click={resetSettings}
                >
                    Reset to Defaults
                </button>
            </div>
        </div>
    </main>
</div>
