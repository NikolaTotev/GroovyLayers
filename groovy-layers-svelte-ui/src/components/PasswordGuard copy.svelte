<script lang="ts">
    import { registration_state_store } from "$lib/user_store";
</script>

{#if $registration_state_store.email != "" && $registration_state_store.username != ""}
    <slot />
{:else}
    <p class="text-error my-10">Please enter email and username first.</p>
{/if}
