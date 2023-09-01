<script lang="ts">
    import "../app.css";
    import LoginGuard from "../components/LoginGuard.svelte";
</script>

<LoginGuard>
    <div class="min-h-screen fles flex-col">
        <slot />
    </div>
</LoginGuard>
