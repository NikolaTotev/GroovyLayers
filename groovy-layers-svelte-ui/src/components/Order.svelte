<script lang="ts">
    import { fly } from "svelte/transition";

    export let id = "";
    export let text = "";
    export let file_location = "";
    export let material_id = 0;
    export let quality_setting = 0;
    export let status = {};
</script>

<li in:fly={{ x: 900, duration: 500 }}>
    <a
        href="order-details-2.html"
        class="flex bg-zinc-700 -4 shadow rounded m-4 p-2 justify-center items-center"
    >
        <img
            src="https://images.unsplash.com/photo-1682685797818-c9dc151d241e?ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1974&q=80"
            alt="Order Image 2"
            class="w-16 h-16 mr-4"
        />
        <div class="flex flex-row p-4 items-center">
            <h2 class="text-lg font-semibold m-5 ">
                Order Id: {id}
            </h2>
            <div class="bg-blue-500 text-white text-sm py-1 px-3 rounded m-4">
                <p>Status: {status}</p>
            </div>
            <div class="bg-blue-500 text-white text-sm py-1 px-3 rounded m-4">
                <p>Quality Setting: {quality_setting}</p>            
            </div>

            <div class="bg-blue-500 text-white text-sm py-1 px-3 rounded m-4">
                <p>File Name: {file_location}</p>            
            </div>

            <div class="bg-blue-500 text-white text-sm py-1 px-3 rounded m-4">
                <p>Material: {material_id}</p>            
            </div>                
        </div>
    </a>
</li>
