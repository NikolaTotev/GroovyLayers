<script>
    import { order_store } from "$lib/order_store";
    import Order from "./Order.svelte";
</script>

<ul>
    {#each $order_store as order}
        <Order
            id={order.id}
            text={order.file_location}
            material_id={order.material_id}
            quality_setting={order.quality_setting}
            status={order.status.status}
        />
    {/each}
</ul>
