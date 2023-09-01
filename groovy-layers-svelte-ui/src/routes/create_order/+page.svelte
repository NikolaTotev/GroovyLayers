<script lang="ts">
    import { goto } from "$app/navigation";
    import { load_orders, order_creation_store } from "$lib/order_store";
    import { current_user_store } from "$lib/user_store";
</script>

<div class="flex justify-center m-10 flex-col">
    <div class="self-center">
        <h1 class="text-3xl">New Order</h1>
    </div>
    <div class="m-10 flex flex-col self-center">
        <select class="select select-info w-full max-w-xs m-4">
            <option disabled selected>Select Material</option>
            <option>English</option>
            <option>Japanese</option>
            <option>Italian</option>
          </select>

        <input
            class="input input-bordered input-primary w-full max-w-xs m-4"
            placeholder="Quality Setting"
            bind:value={$order_creation_store.quality_setting}
        />
        <input
            class="input input-bordered input-primary w-full max-w-xs m-4"
            placeholder="Uploaded file"
            bind:value={$order_creation_store.uploaded_file}
        />
        <button
            class="btn btn-primary btn-active m-4 w-full"
            on:click={() => {
                var jsonData = {
                    id: 1,
                    method: "create_order",
                    params: {
                        data: {
                            user_id: Number($current_user_store.user_id),
                            quality_setting: Number($order_creation_store.quality_setting),
                            material_id: Number($order_creation_store.material_id),
                            file_location: $order_creation_store.uploaded_file,
                            print_job_file: "",
                        },
                    },
                };

                fetch("http://localhost:8080/api/rpc", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json", // Set JSON content type
                    },
                    credentials: "include",
                    body: JSON.stringify(jsonData), // Convert JSON object to a string
                })
                    .then((response) => {
                        if (!response.ok) {
                            throw new Error(
                                `Network response was not ok: ${response.status}`
                            );
                        } else {
                            console.log(JSON.stringify(response));
                            $current_user_store.is_logged_in = true;
                        }
                        return response.json();
                    })
                    .then((data) => {
                        // Process the data
                        console.log(data);
                        load_orders();
                        goto("/home")                   
                    })
                    .catch((error) => {
                        console.error("Fetch error:", error);
                    });
            }}
        >
            Place order
        </button>
    </div>
</div>
