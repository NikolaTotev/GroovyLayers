<script lang="ts">
    import { goto } from "$app/navigation";
    import { registration_state_store } from "$lib/user_store";
    import PasswordGuard from "../../../components/PasswordGuard.svelte";
</script>

<PasswordGuard>
    <h2>Password</h2>
    <input class="input input-bordered input-primary w-full max-w-xs" bind:value={$registration_state_store.password} />
    <button
        class="btn btn-primary btn-active"
        class:btn-disabled={$registration_state_store.email == ""}
        on:click={() => {
            goto("/login");
        }}
    >
        Finish
    </button>
</PasswordGuard>
