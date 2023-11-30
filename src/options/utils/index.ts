export function base64ToUint8Array(base64: string) {
    return Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
}

export function uint8ArrayToBase64(array: Uint8Array) {
    return btoa(String.fromCharCode(...array));
}
