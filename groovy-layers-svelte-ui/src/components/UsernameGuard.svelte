<script lang="ts">
        import { registration_state_store } from "$lib/user_store";
</script>

{#if $registration_state_store.email != ""}
        <slot />
{:else}
<p class="text-error my-10">Please enter email first.</p>
{/if}
