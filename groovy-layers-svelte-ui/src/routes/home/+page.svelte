<script lang="ts">
    import { goto } from "$app/navigation";
    import { load_orders } from "$lib/order_store";
    import { current_user_store } from "$lib/user_store";
    import { onMount } from "svelte";
    import OrdersList from "../../components/OrdersList.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;
    //onMount(load_orders)

</script>

<div class="navbar bg-base-100">
    <div class="navbar-start">
        <div class="dropdown">
            <label tabindex="0" class="btn btn-ghost btn-circle">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    ><path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M4 6h16M4 12h16M4 18h7"
                    /></svg
                >
            </label>
            <ul
                tabindex="0"
                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
            >
                <li><a>Homepage</a></li>
                <li><a>Portfolio</a></li>
                <li><a>About</a></li>
            </ul>
        </div>
    </div>
    <div class="navbar-center">
        <a class="btn btn-ghost normal-case text-xl">Groovy Layers</a>
    </div>
    <div class="navbar-end">
        <button class="btn btn-ghost btn-circle">
            <div class="indicator">
                <span class="material-symbols-outlined"> notifications </span>
                <span class="badge badge-xs badge-secondary indicator-item" />
            </div>
        </button>
        <button
            class="btn btn-ghost btn-circle"
            on:click={() => {
                var jsonData = {
                    logoff: true,
                };

                fetch("http://localhost:8080/api/logout", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json", // Set JSON content type
                    },
                    body: JSON.stringify(jsonData), // Convert JSON object to a string
                })
                    .then((response) => {
                        if (!response.ok) {
                            throw new Error(
                                `Network response was not ok: ${response.status}`
                            );
                        } else {
                            console.log(JSON.stringify(response));
                            $current_user_store.is_logged_in = false;
                        }
                        return response.json();
                    })
                    .then((data) => {
                        // Process the data
                        console.log(data);

                        if (!$current_user_store.is_logged_in) {
                            goto("/login");
                        }
                    })
                    .catch((error) => {
                        console.error("Fetch error:", error);
                    });
            }}
        >
            <span class="material-symbols-outlined"> logout </span>
        </button>
    </div>
</div>

<div class="flex items-center justify-center">
    <div class="flex justify-center items-center h-screen w-full">
        <div class="container mx-auto p-6 text-center">
            <h1 class="text-3xl font-semibold mb-8 text-center">Welcome</h1>
            <OrdersList />
            <button
                class="mt-4 bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600"
                on:click={() => {
                    goto("/create_order");
                }}
            >
                New Order
            </button>
        </div>
    </div>
</div>
