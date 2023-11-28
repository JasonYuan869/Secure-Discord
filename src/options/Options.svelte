<script>
  import { onMount } from 'svelte'

  let countSync = 0
  const link = 'https://github.com/guocaoyi/create-chrome-ext'

  onMount(() => {
    chrome.storage.sync.get(['count'], (result) => {
      countSync = result.count || 0
    })

    chrome.runtime.onMessage.addListener((request) => {
      if (request.type === 'COUNT') {
        countSync = request.count || 0
      }
    })
  })
</script>

<main>

</main>
