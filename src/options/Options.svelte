<script lang="ts">
    import { toasts } from "svelte-toasts";
    import { MessageType, SecureDiscordConfig } from "../types";
    import { onMount } from "svelte";
    import { bytesToBase64DataUrl, dataUrlToBytes } from "./utils";

    let config: SecureDiscordConfig;
    let port: chrome.runtime.Port;
    let changed = false;
    let saved = false;
    const resetConfig = () => {
        changed = false;
        saved = false;
        chrome.storage.sync.get('config', (data) => {
            if (data.config) {
                config = data.config;
            } else {
                config = {
                    enabled: false,
                    identity: {
                        username: '',
                    },
                };
            }
        });
    };

    const onConfigChange = (e: Event) => {
        changed = true;
    };
    const applyConfig = () => {
        chrome.storage.sync.set({ config }, () => {
            if (!chrome.runtime.lastError) {
                changed = false;
                saved = true;
                toasts.success('Config saved');
            } else {
                toasts.error('Failed to save config');
            }
        });
    };

    const reload = () => {
        port?.postMessage({ type: MessageType.CONFIG });
    };

    onMount(() => {
        resetConfig();
        port = chrome.runtime.connect({ name: 'optionsPage' });
    });
</script>

<main class="md:p-12 p-8">
    <h1 class="my-4 text-center text-2xl font-bold">
        Configure Secure Discord
    </h1>
    {#if !config}
        <span class="loading loading-spinner loading-lg"></span>
    {:else}
        <div class="w-96 min-w-fit mx-auto">
            <h2 class="font-bold text-xl">Global settings</h2>
            <div class="form-control mb-10">
                <label class="label">
                    <span class="label-text">Enable Secure Discord</span>
                    <input type="checkbox" class="toggle" bind:checked={config.enabled} on:change={onConfigChange}/>
                </label>
            </div>
            <h2 class="font-bold text-xl">Identity Settings</h2>
            <div class="mb-10">
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">Username</span>
                        <input
                                type="text"
                                class="input input-bordered input-sm"
                                bind:value={config.identity.username}
                                disabled={!config.enabled}
                                on:change={onConfigChange}
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">User ID</span>
                        <input
                                type="text"
                                class="input input-bordered input-sm"
                                bind:value={config.identity.identityKeyPair}
                                disabled={!config.enabled}
                                on:change={onConfigChange}
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">Generated Secret Key</span>
                        <input type="checkbox" checked={!!config.identity.identityKeyPair} disabled class="checkbox checkbox-sm">
                    </label>
                    <div class="tooltip" data-tip="Click to generate a new secret key">
                        <button
                                class="btn btn-sm btn-secondary"
                                on:click={() => {
                                        config.identity.identityKeyPair = crypto.getRandomValues(new Uint8Array(32));
                                        changed = true;
                                    }}
                                disabled={!config.identity.identityKeyPair}
                        >
                            Generate
                        </button>
                    </div>
                </div>
            </div>
            <div class="flex md:flex-row flex-col md:justify-between md:items-center w-100">
                <div class="mb-4 md:mb-0">
                    <button class="btn btn-primary mr-4" on:click={applyConfig} disabled={!changed}>Save</button>
                    <div class="tooltip" data-tip="Reload the extension to apply new configuration">
                        <button class="btn btn-secondary" on:click={reload} disabled={!saved}>Reload Extension</button>
                    </div>
                </div>
                <button class="btn btn-error max-w-fit" on:click={resetConfig} disabled={!changed}>Reset</button>
            </div>
        </div>
    {/if}
</main>
