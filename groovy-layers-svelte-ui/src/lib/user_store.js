import { writable } from "svelte/store";

const registration_state = {
    username: "",
    email:"",
    password:"", 
}

const login_state = {
    email:"",
    password:""
}
const current_user={
    username:"",
    email:"",
    is_logged_in:"false"
}

export const registration_state_store = writable(registration_state);
export const login_state_store = writable(login_state);
export const current_user_store = writable(current_user);
