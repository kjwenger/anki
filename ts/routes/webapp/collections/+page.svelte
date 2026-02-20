<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    onMount(() => {
        goto("/webapp/decks", { replaceState: true });
    });
</script>
