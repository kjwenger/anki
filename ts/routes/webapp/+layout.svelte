<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { isAuthenticated } from "$lib/webapp/stores/auth";
    import { onMount } from "svelte";

    // Public routes that don't require authentication
    const publicRoutes = [
        "/webapp/auth/login",
        "/webapp/auth/register",
    ];

    onMount(() => {
        const unsubscribe = isAuthenticated.subscribe((authenticated) => {
            const currentPath = $page.url.pathname;
            const isPublicRoute = publicRoutes.some((route) =>
                currentPath.startsWith(route)
            );

            if (!authenticated && !isPublicRoute) {
                goto("/webapp/auth/login");
            }
        });

        return unsubscribe;
    });
</script>

<div class="webapp-layout">
    <slot />
</div>

<style>
    .webapp-layout {
        width: 100%;
        min-height: 100vh;
    }

    :global(body) {
        margin: 0;
        padding: 0;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
            "Helvetica Neue", Arial, sans-serif;
    }

    :global(*) {
        box-sizing: border-box;
    }
</style>
