<script lang="ts">
  import { FlatToast, ToastContainer, toasts } from "svelte-toasts";
  import { MessageType, SecureDiscordConfig } from "../types";
  import { onMount } from "svelte";
  import { Identity } from "websignal-lib";
  import { base64ToUint8Array, uint8ArrayToBase64 } from "./utils";

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
    console.log("applyConfig", config);
    chrome.storage.sync.set({ config }, () => {
      if (!chrome.runtime.lastError) {
        changed = false;
        saved = true;
        toasts.success('Config saved');
      } else {
        toasts.error('Failed to save config');
        console.error(chrome.runtime.lastError);
      }
    });
  };

  const generateIdentity = () => {
    const identity = new Identity();
    const keypair = identity.serialize();
    config.identity.identityKeyPair = uint8ArrayToBase64(keypair);
    toasts.success('New identity generated');
    changed = true;
  };

  const exportIdentity = () => {
    if (!config.identity.identityKeyPair) {
      return;
    }
    navigator.clipboard.writeText(config.identity.identityKeyPair).then(() => {
      toasts.success('Identity copied to clipboard');
    }, () => {
      toasts.error('Failed to copy identity to clipboard');
    });
  };

  const importIdentity = () => {
    navigator.clipboard.readText().then((text) => {
      const keypair = base64ToUint8Array(text);
      try {
        const identity = Identity.deserialize(keypair);
        config.identity.identityKeyPair = uint8ArrayToBase64(identity.serialize());
      } catch (e) {
        toasts.error('Invalid identity key');
        return;
      }

      toasts.success('Identity imported');
      changed = true;
    }, () => {
      toasts.error('Failed to import identity');
    });
  }

  const reload = () => {
    port?.postMessage({ type: MessageType.CONFIG });
    toasts.success('Reloading extension');
    setTimeout(() => {
      window.location.reload();
    }, 1000);
  };

  onMount(() => {
    resetConfig();
    port = chrome.runtime.connect({ name: 'optionsPage' });
  });
</script>

<main>
  <div class="fixed top-0 left-0 z-[150] w-full h-full pointer-events-none svelte-toasts-container">
    <ToastContainer placement="bottom-right" let:data={data}>
      <FlatToast {data}/>
    </ToastContainer>
  </div>
  <div class="md:p-12 p-8">
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
                bind:value={config.identity.userId}
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
            <div class="flex justify-center gap-4">
              <div class="tooltip" data-tip="Click to generate a new secret key">
                <button
                  class="btn btn-sm btn-secondary"
                  on:click={generateIdentity}
                >
                  Generate
                </button>
              </div>
              <div class="tooltip" data-tip="Copies the generated key to the clipboard">
                <button
                  class="btn btn-sm btn-primary"
                  on:click={exportIdentity}
                  disabled={!config.identity.identityKeyPair}
                >
                  Export
                </button>
              </div>
              <div class="tooltip" data-tip="Reads an exported key from the clipboard">
                <button
                  class="btn btn-sm btn-accent"
                  on:click={importIdentity}
                  disabled={!config.identity.identityKeyPair}
                >
                  Import
                </button>
              </div>
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
  </div>
</main>
