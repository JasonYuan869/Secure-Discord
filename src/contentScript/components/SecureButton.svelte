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
    <i
            tabindex="0"
            role="button"
            on:click={secureButtonOnClick}
            on:keypress={secureButtonOnClick}
            on:mouseenter={() => isWindowOpen = true}
            on:mouseleave={() => isWindowOpen = false}
            class="text-[1.2rem] bi-lock-fill"
            use:floatingRef
    />
</div>

