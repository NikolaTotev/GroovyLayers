<script lang="ts">
    import { goto } from "$app/navigation";
    import { registration_state_store } from "$lib/user_store";
    import UsernameGuard from "../../../components/UsernameGuard.svelte";
</script>

<UsernameGuard>
    <h2>Username</h2>
    <input
        type="text"
        class="input input-bordered input-primary w-full max-w-xs"
        bind:value={$registration_state_store.username}
    />
    <button
        class="btn btn-primary btn-active"
        class:btn-disabled={$registration_state_store.username == ""}
        on:click={() => {
            goto("/register/password");
        }}
    >
        Next
    </button>
</UsernameGuard>
