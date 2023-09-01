<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { current_user_store } from "$lib/user_store";
</script>

{#if $current_user_store.is_logged_in || $page.route.id?.match(/register|login/g)}
    <slot />
{:else}
    <div class="flex justify-center m-10 flex-col">
        <div class="self-center">
            <h1 class="text-5xl">Oh no!</h1>
        </div>
        <div class="m-10 flex flex-col self-center">
            <p class="text-error my-10 text-2xl">Looks like you aren't logged in!</p>
            <button
                class="btn btn-primary btn-active"    
                on:click={() => {
                    goto("/login");
                }}
            >To Login</button>
        </div>
    </div>
{/if}
