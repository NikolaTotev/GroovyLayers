<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<div class="flex items-center justify-center">
    <div class="flex justify-center items-center h-screen w-full">
        <div class="container mx-auto p-6 text-center">
            <h1 class="text-3xl font-semibold mb-8 text-center">Welcome</h1>
            <div class="overflow-y-scroll max-h-96 w-4/5 mx-auto">
                <!-- Order Cards Go Here -->
                <!-- For each order, create a card using the following structure -->
                <a
                    href="order-details-1.html"
                    class="flex bg-white p-4 shadow rounded mb-4"
                >
                    <img
                        src="https://images.unsplash.com/photo-1682685797818-c9dc151d241e?ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1974&q=80"
                        alt="Order Image 1"
                        class="w-16 h-16 mr-4"
                    />
                    <div class="flex flex-col">
                        <h2 class="text-lg font-semibold mb-2">
                            Order Title 1
                        </h2>
                        <div
                            class="bg-blue-500 text-white text-sm py-1 px-3 rounded"
                        >
                            Status 1
                        </div>
                    </div>
                </a>

                <a
                    href="order-details-2.html"
                    class="flex bg-white p-4 shadow rounded mb-4"
                >
                    <img
                        src="https://images.unsplash.com/photo-1682685797818-c9dc151d241e?ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1974&q=80"
                        alt="Order Image 2"
                        class="w-16 h-16 mr-4"
                    />
                    <div class="flex flex-col">
                        <h2 class="text-lg font-semibold mb-2">
                            Order Title 2
                        </h2>
                        <div
                            class="bg-blue-500 text-white text-sm py-1 px-3 rounded"
                        >
                            Status 2
                        </div>
                    </div>
                </a>
                <!-- Add more order cards as needed -->
            </div>
            <button
                class="mt-4 bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600"
            >
                New Order
            </button>
        </div>
    </div>
</div>
