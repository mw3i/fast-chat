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

<div class="welcome-overlay" data-tauri-drag-region>
  <!-- Back Arrow - Upper Left (only on second screen) -->
  {#if currentPage === 1}
    <button 
      class="welcome-back-button"
      on:click={handleBack}
      data-tauri-drag-region="false"
      aria-label="Back"
    >
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M19 12H5M12 19l-7-7 7-7"/>
      </svg>
    </button>
  {/if}

  <!-- Page Content -->
  <div class="flex items-center justify-center h-full p-8" data-tauri-drag-region>
    {#if currentPage === 0}
      <div class="w-full max-w-2xl text-center welcome-text">
        <h1 class="text-4xl font-semibold mb-8">Welcome to the app</h1>
        <button 
          class="welcome-button"
          on:click={handleGetStarted}
          data-tauri-drag-region="false"
          aria-label="Get started"
        >
          Get started
        </button>
      </div>
    {:else if currentPage === 1}
      <div class="w-full max-w-2xl welcome-text">
        <h1 class="text-4xl font-semibold mb-8 text-center">Local LLM Setup</h1>
        <div class="text-lg leading-relaxed welcome-text-secondary space-y-6">
          <p>
            To set up a local model, the easiest approach is going to be installing Ollama. 
            Go to <a href="https://ollama.com" on:click={handleLinkClick} data-tauri-drag-region="false" class="welcome-link">ollama.com</a> and download it, install it, and run it.
          </p>
          <p>
            Once you've loaded up Ollama, download an LLM. <code class="welcome-code">gemma3:4b</code> is a good one to start with.
          </p>
        </div>
      </div>
    {/if}
  </div>

  <!-- Finish Button - Bottom Right (only on second screen) -->
  {#if currentPage === 1}
    <button 
      class="welcome-button welcome-finish-button"
      on:click={handleFinish}
      data-tauri-drag-region="false"
      aria-label="Finish setup"
    >
      Finish Setup. Get started.
    </button>
  {/if}
</div>

<style>
  .welcome-overlay {
    position: fixed;
    inset: 0;
    background: var(--bg-primary);
    backdrop-filter: blur(24px) saturate(180%);
    -webkit-backdrop-filter: blur(24px) saturate(180%);
    z-index: 1000;
    transition: background 0.3s ease;
  }

  .welcome-text {
    color: var(--text-primary);
  }

  .welcome-text-secondary {
    color: var(--text-secondary);
  }

  .welcome-back-button {
    position: absolute;
    top: 3rem;
    left: 2rem;
    color: var(--text-primary);
    transition: opacity 0.2s;
    cursor: pointer;
    background: transparent;
    border: none;
    padding: 0;
  }

  .welcome-back-button:hover {
    opacity: 0.7;
  }

  .welcome-button {
    padding: 0.75rem 1.5rem;
    border-radius: 0.5rem;
    background: var(--bg-button);
    border: 1px solid var(--border-secondary);
    color: var(--text-primary);
    font-weight: 500;
    transition: all 0.2s;
    cursor: pointer;
  }

  .welcome-button:hover {
    background: var(--bg-button-hover);
    border-color: var(--border-secondary);
  }

  .welcome-finish-button {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    z-index: 1001;
  }

  .welcome-code {
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.875rem;
    background: var(--bg-input);
    color: var(--text-primary);
  }

  .welcome-link {
    color: var(--link-color);
    text-decoration: underline;
    transition: opacity 0.2s;
    cursor: pointer;
  }

  .welcome-link:hover {
    opacity: 0.8;
  }

</style>

