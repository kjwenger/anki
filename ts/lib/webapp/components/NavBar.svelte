<script lang="ts">
    import { goto } from "$app/navigation";
    import { currentUser, authStore } from "$lib/webapp/stores/auth";
    import { page } from "$app/stores";

    let showUserMenu = false;
    let showMobileMenu = false;

    function handleLogout() {
        authStore.logout();
        goto("/webapp/auth/login");
    }

    function toggleUserMenu() {
        showUserMenu = !showUserMenu;
    }

    function toggleMobileMenu() {
        showMobileMenu = !showMobileMenu;
    }

    function navigateTo(path: string) {
        goto(path);
        showMobileMenu = false;
        showUserMenu = false;
    }

    $: currentPath = $page.url.pathname;
    $: isActive = (path: string) => currentPath.startsWith(path);
</script>

<nav class="navbar">
    <div class="navbar-container">
        <div class="navbar-brand">
            <button class="brand-link" on:click={() => navigateTo("/webapp")}>
                <span class="brand-icon">🎴</span>
                <span class="brand-text">Anki Web</span>
            </button>
            <button class="mobile-menu-toggle" on:click={toggleMobileMenu}>
                {showMobileMenu ? "✕" : "☰"}
            </button>
        </div>

        <div class="navbar-menu" class:active={showMobileMenu}>
            <div class="navbar-links">
                <button
                    class="nav-link"
                    class:active={isActive("/webapp/decks")}
                    on:click={() => navigateTo("/webapp/decks")}
                >
                    Decks
                </button>
                <button
                    class="nav-link"
                    class:active={isActive("/webapp/editor")}
                    on:click={() => navigateTo("/webapp/editor")}
                >
                    Add
                </button>
                <button
                    class="nav-link"
                    class:active={isActive("/webapp/browse")}
                    on:click={() => navigateTo("/webapp/browse")}
                >
                    Browse
                </button>
                <button
                    class="nav-link"
                    class:active={isActive("/webapp/stats")}
                    on:click={() => navigateTo("/webapp/stats")}
                >
                    Stats
                </button>
            </div>

            <div class="navbar-user">
                {#if $currentUser}
                    <button class="user-button" on:click={toggleUserMenu}>
                        <span class="user-icon">👤</span>
                        <span class="user-name">{$currentUser.username}</span>
                        <span class="user-arrow">{showUserMenu ? "▲" : "▼"}</span>
                    </button>

                    {#if showUserMenu}
                        <div class="user-menu">
                            <button
                                class="menu-item"
                                on:click={() => navigateTo("/webapp/auth/profile")}
                            >
                                Profile
                            </button>
                            <button
                                class="menu-item"
                                on:click={() => navigateTo("/webapp/settings")}
                            >
                                Settings
                            </button>
                            <button
                                class="menu-item"
                                on:click={() => navigateTo("/webapp/collections")}
                            >
                                Collections
                            </button>
                            <div class="menu-divider"></div>
                            <button class="menu-item logout" on:click={handleLogout}>
                                Logout
                            </button>
                        </div>
                    {/if}
                {/if}
            </div>
        </div>
    </div>
</nav>

<style>
    .navbar {
        background: white;
        border-bottom: 1px solid #e0e0e0;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        position: sticky;
        top: 0;
        z-index: 1000;
    }

    .navbar-container {
        max-width: 1400px;
        margin: 0 auto;
        padding: 0 1rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        height: 60px;
    }

    .navbar-brand {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .brand-link {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: none;
        border: none;
        cursor: pointer;
        font-size: 1.25rem;
        font-weight: 600;
        color: #333;
        padding: 0;
    }

    .brand-link:hover {
        color: #667eea;
    }

    .brand-icon {
        font-size: 1.5rem;
    }

    .mobile-menu-toggle {
        display: none;
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0.5rem;
    }

    .navbar-menu {
        display: flex;
        align-items: center;
        gap: 2rem;
    }

    .navbar-links {
        display: flex;
        gap: 0.5rem;
    }

    .nav-link {
        background: none;
        border: none;
        padding: 0.5rem 1rem;
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        color: #666;
        border-radius: 4px;
        transition: all 0.2s;
    }

    .nav-link:hover {
        background: #f5f5f5;
        color: #333;
    }

    .nav-link.active {
        background: #667eea;
        color: white;
    }

    .navbar-user {
        position: relative;
    }

    .user-button {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: #f5f5f5;
        border: 1px solid #ddd;
        border-radius: 4px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        font-size: 14px;
        transition: all 0.2s;
    }

    .user-button:hover {
        background: #e8e8e8;
    }

    .user-icon {
        font-size: 16px;
    }

    .user-name {
        font-weight: 500;
        color: #333;
    }

    .user-arrow {
        font-size: 10px;
        color: #999;
    }

    .user-menu {
        position: absolute;
        top: calc(100% + 0.5rem);
        right: 0;
        background: white;
        border: 1px solid #ddd;
        border-radius: 4px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        min-width: 180px;
        z-index: 1001;
    }

    .menu-item {
        display: block;
        width: 100%;
        padding: 0.75rem 1rem;
        background: none;
        border: none;
        text-align: left;
        cursor: pointer;
        font-size: 14px;
        color: #333;
        transition: background 0.2s;
    }

    .menu-item:hover {
        background: #f5f5f5;
    }

    .menu-item.logout {
        color: #d32f2f;
    }

    .menu-divider {
        height: 1px;
        background: #e0e0e0;
        margin: 0.5rem 0;
    }

    @media (max-width: 768px) {
        .mobile-menu-toggle {
            display: block;
        }

        .navbar-menu {
            position: absolute;
            top: 60px;
            left: 0;
            right: 0;
            background: white;
            border-bottom: 1px solid #e0e0e0;
            flex-direction: column;
            gap: 0;
            padding: 1rem;
            display: none;
        }

        .navbar-menu.active {
            display: flex;
        }

        .navbar-links {
            flex-direction: column;
            width: 100%;
            gap: 0.5rem;
        }

        .nav-link {
            width: 100%;
            text-align: left;
        }

        .navbar-user {
            width: 100%;
            margin-top: 0.5rem;
        }

        .user-button {
            width: 100%;
            justify-content: space-between;
        }

        .user-menu {
            position: static;
            margin-top: 0.5rem;
            box-shadow: none;
            border: none;
            background: #f9f9f9;
        }
    }
</style>
