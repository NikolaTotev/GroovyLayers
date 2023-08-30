<script lang="ts">
    import { page } from "$app/stores";
</script>

<nav class="flex justify-center my-6">
    <ul class="steps">
        <a href="/register/email" class="step step-primary">Email</a>               
        <a
            href="/register/username"
            class="step"
            class:step-primary={$page.route.id?.match(/username|password/g)}
            >Choose Username</a
        >
        <a
            href="/register/password"
            class="step"
            class:step-primary={$page.route.id?.match(/password/g)}
            >Password</a
        >
    </ul>
</nav>

<main class="card w-4/6 bg-neutral text-neutral-content mx-auto">
    <div class="card-body items-center text-center">
        <slot />
    </div>
</main>
