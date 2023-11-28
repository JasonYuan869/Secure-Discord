<script lang="ts">
    import { toasts } from "svelte-toasts";
    import { offset, shift } from "svelte-floating-ui/dom";
    import { createFloatingActions } from "svelte-floating-ui";
    import { currentChat, currentUser } from "../stores";
    import FloatingWindow from "./Tooltip.svelte";
    import { getChatBoxText } from "../utils/domHelpers";

    const [floatingRef, floatingContent] = createFloatingActions({
        strategy: "fixed",
        placement: "top-start",
        middleware: [
            offset({ mainAxis: 50, crossAxis: -115 }),
            shift(),
        ]
    });

    let isWindowOpen = false;

    const secureButtonOnClick = () => {
        console.log("clicked in chat", $currentChat);
        console.log("logged in as", $currentUser);
        const txt = getChatBoxText();
        console.log("txt", txt);
        navigator.clipboard.writeText(txt?.trim() || "");
        toasts.success("Copied to clipboard!", { placement: "top-right", duration: 2000 });
    };
</script>

<div>
    {#if isWindowOpen}
        <div use:floatingContent>
            <FloatingWindow/>
        </div>
    {/if}
    <div
            tabindex="0"
            role="button"
            on:click={secureButtonOnClick}
            on:keypress={secureButtonOnClick}
            on:mouseenter={() => isWindowOpen = true}
            on:mouseleave={() => isWindowOpen = false}
            use:floatingRef
    >
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-lock-fill"
             viewBox="0 0 16 16">
            <path d="M8 1a2 2 0 0 1 2 2v4H6V3a2 2 0 0 1 2-2m3 6V3a3 3 0 0 0-6 0v4a2 2 0 0 0-2 2v5a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2"/>
        </svg>
    </div>
</div>

