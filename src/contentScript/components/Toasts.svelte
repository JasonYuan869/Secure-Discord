<script lang="ts">
    import { ToastContainer, FlatToast } from "svelte-toasts";
</script>


<div class="fixed top-0 left-0 z-[150] w-full h-full pointer-events-none svelte-toasts-container">
    <ToastContainer placement="bottom-right" let:data={data}>
        <FlatToast {data}/>
    </ToastContainer>
</div>

