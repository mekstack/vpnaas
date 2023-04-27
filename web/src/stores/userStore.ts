import { writable } from 'svelte/store';

interface UserStoreState {
    username: string | null;
    accessToken: string | null;
}

const initialState: UserStoreState = {
    username: null,
    accessToken: null,
};

const userStore = writable<UserStoreState>(initialState);

export function setUsername(username: string): void {
    userStore.update((state) => ({
        ...state,
        username,
    }));
}

export function setAccessToken(accessToken: string): void {
    userStore.update((state) => ({
        ...state,
        accessToken,
    }));
}

export default userStore;
