<script lang="ts">
    const openOptions = () => chrome.runtime.openOptionsPage();
</script>

<template>
    <main class="w-56 h-32 p-6 flex flex-col items-center justify-center">
        <h3 class="font-bold w-full text-center text-lg">Secure Discord</h3>
        <button class="btn btn-primary btn-sm" on:click={openOptions}>Open Options</button>
    </main>
</template>
