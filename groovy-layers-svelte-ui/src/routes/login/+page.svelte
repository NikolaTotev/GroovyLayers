<script lang="ts">
    import { goto } from "$app/navigation";
    import { login_state_store, current_user_store } from "$lib/user_store";
    import { stringify } from "postcss";
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
            class:btn-disabled={$login_state_store.email == "" ||
                $login_state_store.password == ""}
            on:click={() => {
                var jsonData = {
                    username: $login_state_store.email,
                    pwd: $login_state_store.password,
                };

                fetch("http://localhost:8080/api/login", {
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
                            console.log(response);
                            $current_user_store.is_logged_in = true;
                        }
                        return response.json();
                    })
                    .then((data) => {
                        // Process the data
                        console.log(data);
                        
                        if ($current_user_store.is_logged_in) {
                            goto("/home");
                        }
                    })
                    .catch((error) => {
                        console.error("Fetch error:", error);
                    });
            }}
        >
            Login
        </button>
    </div>
</div>
