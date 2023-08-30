<script lang="ts">
    import { goto } from "$app/navigation";
    import { registration_state_store } from "$lib/user_store";
</script>

<h2>Email</h2>
<input class="input input-bordered input-primary w-full max-w-xs" bind:value={$registration_state_store.email} />

<button
    class="btn btn-primary btn-active"
    class:btn-disabled={$registration_state_store.email == ""}
    on:click={() => {
        goto("/register/username");
    }}
>
    Next
</button>
