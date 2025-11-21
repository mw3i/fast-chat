<script>
  import { X } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import ShortcutPicker from '../ShortcutPicker.svelte';
  
  export let show = false;
  export let closing = false;
  export let loading = false;
  export let settings = {};
  export let shortcutError = '';
  export let onClose;
  export let onSettingChange;
  export let onSave;
  export let onRedoSetup;

  let ollamaModels = [];
  let loadingModels = false;
  let modelError = '';
  let hasFetchedModels = false;
  let lastFetchedUrl = '';

  async function fetchOllamaModels() {
    if (settings.provider !== 'ollama') {
      ollamaModels = [];
      hasFetchedModels = false;
      return;
    }

    const url = settings['provider-params']?.ollama?.url || 'http://localhost:11434';
    
    // Don't fetch if we already fetched for this URL
    if (hasFetchedModels && lastFetchedUrl === url) {
      return;
    }

    loadingModels = true;
    modelError = '';

    try {
      const models = await invoke('list_ollama_models', { url });
      ollamaModels = models || [];
      hasFetchedModels = true;
      lastFetchedUrl = url;
      if (ollamaModels.length === 0) {
        modelError = 'No models found. Make sure Ollama is running and has models installed.';
      } else {
        // Auto-select the first model if no model is currently selected or if URL changed
        const currentModel = settings['provider-params']?.ollama?.model;
        if (!currentModel || !ollamaModels.includes(currentModel)) {
          settings['provider-params'].ollama.model = ollamaModels[0];
          settings = settings;
          await onSave();
        }
      }
    } catch (error) {
      console.error('Error fetching Ollama models:', error);
      modelError = `Failed to fetch models: ${error}`;
      ollamaModels = [];
      hasFetchedModels = false;
    } finally {
      loadingModels = false;
    }
  }

  // Fetch models when settings panel opens and Ollama is selected
  $: if (show && settings.provider === 'ollama' && !loading && !hasFetchedModels) {
    fetchOllamaModels();
  }

  // Fetch models when provider changes to ollama
  let previousProvider = '';
  $: {
    if (settings.provider !== previousProvider) {
      previousProvider = settings.provider;
      if (settings.provider === 'ollama' && show) {
        hasFetchedModels = false; // Reset to allow fetching
        fetchOllamaModels();
      } else {
        ollamaModels = [];
        modelError = '';
        hasFetchedModels = false;
      }
    }
  }

  // Reset fetch flag when panel closes
  $: if (!show) {
    hasFetchedModels = false;
  }
</script>

{#if show}
  <div class="settings-panel" class:closing={closing}>
    <div class="settings-header">
      <h2 class="settings-title">Settings</h2>
      <button
        on:click={onClose}
        class="settings-close-button"
        title="Close settings"
      >
        <X size={20} />
      </button>
    </div>
    
    <div class="settings-content">
      {#if loading}
        <div class="settings-loading">Loading settings...</div>
      {:else}
        <!-- Keyboard Shortcut Section -->
        <div class="settings-section">
          <h3 class="settings-section-heading">KEYBOARD SHORTCUT</h3>
          
          <div class="setting-item">
            <label class="setting-label">
              <span class="setting-label-text">Toggle Launcher</span>
              <ShortcutPicker
                value={settings['keyboard-shortcut'] || 'Ctrl+Space'}
                onSave={async (shortcut) => {
                  const previousShortcut = settings['keyboard-shortcut'] || 'Ctrl+Space';
                  settings['keyboard-shortcut'] = shortcut;
                  settings = settings;
                  try {
                    await onSave();
                  } catch (error) {
                    // If save fails, revert in UI (backend already reverted)
                    settings['keyboard-shortcut'] = previousShortcut;
                    settings = settings;
                  }
                }}
              />
              {#if shortcutError}
                <div class="shortcut-error">
                  {shortcutError}
                </div>
              {/if}
            </label>
          </div>
        </div>
        
        <!-- Model Section -->
        <div class="settings-section">
          <h3 class="settings-section-heading">MODEL</h3>
        
        <!-- Provider Selection -->
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">Provider</span>
            <select
              value={settings.provider}
              on:change={(e) => onSettingChange('provider', e.target.value)}
              class="setting-select"
            >
              <option value="ollama">Ollama</option>
              <option value="openai">ChatGPT (OpenAI)</option>
              <option value="google">Google</option>
              <option value="lmstudio">LM Studio</option>
              <option value="custom">Custom</option>
            </select>
          </label>
        </div>
        
        <!-- Provider-specific settings -->
        <div class="provider-dependent-settings">
        {#if settings.provider === 'ollama'}
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">Ollama URL</span>
            <input
              type="text"
              value={settings['provider-params'].ollama.url}
              on:input={(e) => {
                settings['provider-params'].ollama.url = e.target.value;
                settings = settings;
              }}
              on:blur={async () => {
                await onSave();
                // Fetch models after URL changes
                fetchOllamaModels();
              }}
              placeholder="http://localhost:11434"
              class="setting-input"
            />
          </label>
        </div>
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">Model Name</span>
            {#if loadingModels}
              <div class="relative">
                <select disabled class="setting-select opacity-50">
                  <option>Select a model</option>
                </select>
                <div class="absolute right-3 top-1/2 -translate-y-1/2">
                  <div class="w-4 h-4 border-2 border-white/30 border-t-white/80 rounded-full animate-spin"></div>
                </div>
              </div>
            {:else if ollamaModels.length > 0}
              <select
                value={settings['provider-params'].ollama.model || ''}
                on:change={(e) => {
                  settings['provider-params'].ollama.model = e.target.value;
                  settings = settings;
                  onSave();
                }}
                class="setting-select"
              >
                <option value="" disabled>Select a model</option>
                {#each ollamaModels as model}
                  <option value={model}>{model}</option>
                {/each}
              </select>
            {:else}
              <div class="model-select-container">
                <select
                  value={settings['provider-params'].ollama.model}
                  on:change={(e) => {
                    settings['provider-params'].ollama.model = e.target.value;
                    settings = settings;
                    onSave();
                  }}
                  class="setting-select"
                >
                  <option value={settings['provider-params'].ollama.model || ''}>
                    {settings['provider-params'].ollama.model || 'No models available'}
                  </option>
                </select>
                <button
                  type="button"
                  on:click={fetchOllamaModels}
                  class="refresh-models-button"
                  title="Refresh models"
                >
                  ↻
                </button>
              </div>
            {/if}
            {#if modelError}
              <div class="model-error">
                {modelError}
              </div>
            {/if}
          </label>
        </div>
      {:else if settings.provider === 'openai'}
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">OpenAI API Key</span>
            <input
              type="password"
              value={settings['provider-params'].openai.api_key}
              on:input={(e) => {
                settings['provider-params'].openai.api_key = e.target.value;
                settings = settings;
              }}
              on:blur={onSave}
              placeholder="sk-..."
              class="setting-input"
            />
          </label>
        </div>
      {:else if settings.provider === 'google'}
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">Google API Key</span>
            <input
              type="password"
              value={settings['provider-params'].google.api_key}
              on:input={(e) => {
                settings['provider-params'].google.api_key = e.target.value;
                settings = settings;
              }}
              on:blur={onSave}
              placeholder="API key..."
              class="setting-input"
            />
          </label>
        </div>
      {:else if settings.provider === 'lmstudio'}
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">LM Studio URL</span>
            <input
              type="text"
              value={settings['provider-params'].lmstudio.url}
              on:input={(e) => {
                settings['provider-params'].lmstudio.url = e.target.value;
                settings = settings;
              }}
              on:blur={onSave}
              placeholder="http://localhost:1234"
              class="setting-input"
            />
          </label>
        </div>
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">Model Name</span>
            <input
              type="text"
              value={settings['provider-params'].lmstudio.model}
              on:input={(e) => {
                settings['provider-params'].lmstudio.model = e.target.value;
                settings = settings;
              }}
              on:blur={onSave}
              placeholder="local-model"
              class="setting-input"
            />
          </label>
        </div>
      {:else if settings.provider === 'custom'}
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">Custom API URL</span>
            <input
              type="text"
              value={settings['provider-params'].custom.url}
              on:input={(e) => {
                settings['provider-params'].custom.url = e.target.value;
                settings = settings;
              }}
              on:blur={onSave}
              placeholder="https://api.example.com"
              class="setting-input"
            />
          </label>
        </div>
      {/if}
        </div>
      </div>
      
      <!-- Reset Section -->
      <div class="settings-section">
        <h3 class="settings-section-heading">Reset</h3>
        <div class="setting-item">
          <button
            on:click={() => {
              if (onRedoSetup) {
                onRedoSetup();
                onClose();
              }
            }}
            class="redo-setup-button"
          >
            Redo the setup?
          </button>
        </div>
      </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .settings-panel {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(20, 20, 20, 0.9);
    backdrop-filter: blur(24px) saturate(180%);
    -webkit-backdrop-filter: blur(24px) saturate(180%);
    border-radius: 1rem;
    z-index: 100;
    display: flex;
    flex-direction: column;
    animation: slideInRight 0.3s ease;
    overflow: hidden;
  }

  .settings-panel.closing {
    animation: slideOutRight 0.3s ease;
  }

  @keyframes slideInRight {
    from {
      transform: translateX(100%);
    }
    to {
      transform: translateX(0);
    }
  }

  @keyframes slideOutRight {
    from {
      transform: translateX(0);
    }
    to {
      transform: translateX(100%);
    }
  }

  .settings-header {
    @apply flex items-center justify-between;
    @apply p-6;
    @apply border-b;
    border-color: rgba(255, 255, 255, 0.1);
  }

  .settings-title {
    @apply text-2xl;
    @apply font-semibold;
    color: rgba(255, 255, 255, 0.9);
    margin: 0;
  }

  .settings-close-button {
    @apply flex items-center justify-center;
    @apply p-2;
    @apply rounded-lg;
    @apply transition-colors;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
  }

  .settings-close-button:hover {
    color: rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.1);
  }

  .settings-content {
    @apply flex flex-col;
    @apply p-6;
    @apply gap-6;
    @apply overflow-y-auto;
    flex: 1;
  }

  .settings-loading {
    @apply text-center;
    @apply py-8;
    color: rgba(255, 255, 255, 0.5);
  }

  .setting-item {
    @apply flex flex-col;
    @apply gap-3;
    margin-bottom: 1rem;
  }

  .setting-label {
    @apply flex flex-col;
    @apply gap-3;
  }

  .setting-label-text {
    @apply text-sm;
    @apply font-medium;
    color: rgba(255, 255, 255, 0.8);
  }

  .setting-select,
  .setting-input {
    @apply px-4 py-2;
    @apply rounded-lg;
    @apply bg-transparent;
    @apply border;
    @apply outline-none;
    @apply transition-colors;
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
  }

  .setting-select:focus,
  .setting-input:focus {
    border-color: rgba(59, 130, 246, 0.5);
    background: rgba(255, 255, 255, 0.08);
  }

  .setting-select {
    @apply cursor-pointer;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='rgba(255,255,255,0.6)' d='M6 9L1 4h10z'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    padding-right: 2.5rem;
  }
  
  .settings-section {
    @apply flex flex-col;
    @apply gap-4;
    margin-bottom: 1rem;
  }
  
  .settings-section-heading {
    @apply text-4xl;
    @apply font-bold;
    color: rgba(255, 255, 255, 0.95);
    margin-bottom: 1rem;
  }
  
  .provider-dependent-settings {
    width: 50%;
  }

  .setting-input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .model-select-container {
    @apply flex;
    @apply gap-2;
    @apply items-center;
  }

  .model-select-container .setting-select {
    flex: 1;
  }

  .refresh-models-button {
    @apply px-3 py-2;
    @apply rounded-lg;
    @apply bg-transparent;
    @apply border;
    @apply transition-colors;
    @apply cursor-pointer;
    color: rgba(255, 255, 255, 0.7);
    border-color: rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
    font-size: 1.2rem;
    line-height: 1;
  }

  .refresh-models-button:hover {
    color: rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
  }

  .model-error {
    @apply text-sm;
    @apply mt-2;
    color: #ef4444;
  }

  /* Shortcut Error Message */
  .shortcut-error {
    @apply text-sm;
    @apply mt-2;
    color: #ef4444;
    animation: fadeInError 0.3s ease, fadeOutError 0.3s ease 3.7s forwards;
  }

  @keyframes fadeInError {
    from {
      opacity: 0;
      transform: translateY(-5px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes fadeOutError {
    from {
      opacity: 1;
    }
    to {
      opacity: 0;
    }
  }

  .redo-setup-button {
    @apply px-4 py-2;
    @apply rounded-lg;
    @apply bg-transparent;
    @apply border;
    @apply transition-colors;
    @apply cursor-pointer;
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
    font-size: 0.9rem;
  }

  .redo-setup-button:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.3);
  }
</style>

