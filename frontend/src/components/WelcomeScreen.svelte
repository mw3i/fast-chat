<script>
  export let onContinue;

  let currentPage = 0;

  function handleGetStarted() {
    currentPage = 1;
  }

  function handleBack() {
    currentPage = 0;
  }

  function handleFinish() {
    if (onContinue) {
      onContinue();
    }
  }

  function handleLinkClick(event) {
    event.preventDefault();
    const url = event.currentTarget.href;
    // Open in default browser
    window.open(url, '_blank', 'noopener,noreferrer');
  }
</script>

<div class="fixed inset-0 bg-black/95 backdrop-blur-2xl backdrop-saturate-150 z-[1000]">
  <!-- Back Arrow - Upper Left (only on second screen) -->
  {#if currentPage === 1}
    <button 
      class="absolute top-8 left-8 text-white hover:opacity-70 transition-opacity cursor-pointer"
      on:click={handleBack}
      aria-label="Back"
    >
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M19 12H5M12 19l-7-7 7-7"/>
      </svg>
    </button>
  {/if}

  <!-- Page Content -->
  <div class="flex items-center justify-center h-full p-8">
    {#if currentPage === 0}
      <div class="w-full max-w-2xl text-center text-white">
        <h1 class="text-4xl font-semibold mb-8">Welcome to the app</h1>
        <button 
          class="px-6 py-3 rounded-lg bg-white/10 border border-white/20 text-white font-medium hover:bg-white/20 hover:border-white/30 transition-all"
          on:click={handleGetStarted}
          aria-label="Get started"
        >
          Get started
        </button>
      </div>
    {:else if currentPage === 1}
      <div class="w-full max-w-2xl text-white">
        <h1 class="text-4xl font-semibold mb-8 text-center">Local LLM Setup</h1>
        <div class="text-lg leading-relaxed text-white/90 space-y-6">
          <p>
            To set up a local model, the easiest approach is going to be installing Ollama. 
            Go to <a href="https://ollama.com" on:click={handleLinkClick} class="text-blue-400 underline hover:text-blue-300 transition-colors cursor-pointer">ollama.com</a> and download it, install it, and run it.
          </p>
          <p>
            Once you've loaded up Ollama, download an LLM. <code class="bg-white/10 px-2 py-1 rounded font-mono text-sm text-yellow-400">gemma3:4b</code> is a good one to start with.
          </p>
        </div>
      </div>
    {/if}
  </div>

  <!-- Finish Button - Bottom Right (only on second screen) -->
  {#if currentPage === 1}
    <button 
      class="fixed bottom-8 right-8 px-6 py-3 rounded-lg bg-white/10 border border-white/20 text-white font-medium hover:bg-white/20 hover:border-white/30 transition-all z-[1001]"
      on:click={handleFinish}
      aria-label="Finish setup"
    >
      Finish Setup. Get started.
    </button>
  {/if}
</div>

<style>
</style>

