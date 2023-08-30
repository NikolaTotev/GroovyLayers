<script lang="ts">
    import { goto } from "$app/navigation";
    import { login_state_store } from "$lib/user_store";
</script>

<div class="flex justify-center m-10 flex-col">
    <div class="self-center">
        <h1 class="text-3xl">Login</h1>
    </div>
    <div class="m-10 flex flex-col self-center">
        <input
            class="input input-bordered input-primary w-full max-w-xs m-4"
            placeholder="E-mail"
            bind:value={$login_state_store.email}
        />

        <input
            class="input input-bordered input-primary w-full max-w-xs m-4"
            placeholder="Password"
            bind:value={$login_state_store.password}
        />
        <button
        class="btn btn-primary btn-active m-4 w-full"
        class:btn-disabled={($login_state_store.email == "" || $login_state_store.password == "")}
        on:click={() => {
            goto("/home");
        }}
    >
        Login
    </button>
    </div>
    
</div>
