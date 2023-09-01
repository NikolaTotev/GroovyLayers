import { writable } from "svelte/store";
import { current_user_store } from "$lib/user_store";
import { onDestroy } from "svelte";
const order_creation_state = {
    material_id: 0,
    quality_setting:0,
    uploaded_file:"", 
}


export const order_store = writable([]);
export const order_creation_store = writable(order_creation_state);

export function load_orders() {

    let current_user_id=0;

	const unsubscribe = current_user_store.subscribe(value => {
		current_user_id = value.user_id;
	});
    
    var jsonData = {
        id: 1,
        method: "get_order",
        params:{
            id: Number(current_user_id)
        }
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
            }
            else{
                console.log("ORDERS ===================")
                console.log(response);
            }
            return response.json();
        })
        .then((data) => {
            // Process the data
            console.log("ORDERS DATA ===================")
            console.log(data.result);   
            order_store.set(data.result);
     
        })
        .catch((error) => {
            console.error("Fetch error:", error);
        });

        unsubscribe()
}