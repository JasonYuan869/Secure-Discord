import { writable } from 'svelte/store';
const currentChat = writable("");
const currentUser = writable("");


export { currentChat, currentUser };
