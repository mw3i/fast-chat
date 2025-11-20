<script>
  import { X } from 'lucide-svelte';
  import ShortcutPicker from '../ShortcutPicker.svelte';
  
  export let show = false;
  export let closing = false;
  export let loading = false;
  export let settings = {};
  export let shortcutError = '';
  export let onClose;
  export let onSettingChange;
  export let onSave;
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
              on:blur={onSave}
              placeholder="http://localhost:11434"
              class="setting-input"
            />
          </label>
        </div>
        <div class="setting-item">
          <label class="setting-label">
            <span class="setting-label-text">Model Name</span>
            <input
              type="text"
              value={settings['provider-params'].ollama.model}
              on:input={(e) => {
                settings['provider-params'].ollama.model = e.target.value;
                settings = settings;
              }}
              on:blur={onSave}
              placeholder="gemma2:2b"
              class="setting-input"
            />
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
</style>

