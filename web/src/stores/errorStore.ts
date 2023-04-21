import { writable } from "svelte/store";

interface ErrorMessage {
    id: number;
    message: string;
}

const errorStore = writable<ErrorMessage[]>([]);

export function addError(message: string, duration: number = 5000) {
    const id = new Date().getTime();
    errorStore.update((errors) => [...errors, { id, message }]);

    setTimeout(() => {
        errorStore.update((errors) =>
            errors.filter((error) => error.id !== id)
        );
    }, duration);
}

export default errorStore;
