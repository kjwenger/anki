<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { isAuthenticated } from "$lib/webapp/stores/auth";
    import { onMount } from "svelte";
    import NavBar from "$lib/webapp/components/NavBar.svelte";
    import "./app.css";

    // Public routes that don't require authentication
    const publicRoutes = [
        "/webapp/auth/login",
        "/webapp/auth/register",
    ];

    let showNav = false;

    // Initialize dark mode from localStorage before render
    if (typeof document !== "undefined") {
        try {
            const settings = JSON.parse(
                localStorage.getItem("anki-webapp-settings") || "{}",
            );
            if (settings.theme === "dark") {
                document.documentElement.classList.add("dark");
            } else {
                document.documentElement.classList.remove("dark");
            }
        } catch {
            // ignore
        }
    }

    onMount(() => {
        const unsubscribe = isAuthenticated.subscribe((authenticated) => {
            const currentPath = $page.url.pathname;
            const isPublicRoute = publicRoutes.some((route) =>
                currentPath.startsWith(route)
            );

            showNav = authenticated && !isPublicRoute;

            if (!authenticated && !isPublicRoute) {
                goto("/webapp/auth/login");
            }
        });

        return unsubscribe;
    });
</script>

<div id="webapp-root" class="w-full min-h-screen">
    {#if showNav}
        <NavBar />
    {/if}
    <slot />
</div>
