<script lang="ts">
    import { registration_state_store } from "$lib/user_store";
</script>

<h2>Login</h2>
<input bind:value={$registration_state_store.email} />
<h2>Name is {$registration_state_store}</h2>
