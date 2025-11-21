<script>
  import { Settings } from 'lucide-svelte';
  import ConversationList from './ConversationList.svelte';
  
  export let query = '';
  export let inputRef;
  export let conversations = [];
  export let loading = false;
  export let activeSessions = new Set();
  export let onKeydown;
  export let onSettingsClick;
  export let onConversationClick;
  export let onDeleteConversation;
  export let onDeleteAll;
</script>

<div class="input-container">
  <input
    bind:this={inputRef}
    bind:value={query}
    on:keydown={onKeydown}
    type="text"
    placeholder="Type to search..."
    class="launcher-input"
    autofocus
  />
  <button
    on:click={onSettingsClick}
    class="settings-button"
    title="Settings"
  >
    <Settings size={20} />
  </button>
</div>

<ConversationList
  {conversations}
  {loading}
  {activeSessions}
  {onConversationClick}
  {onDeleteConversation}
  {onDeleteAll}
/>

<style>
  .input-container {
    @apply flex items-center gap-3;
    position: relative;
    flex-shrink: 0;
    padding-top: 0.5rem;
  }

  .launcher-input {
    @apply flex-1;
    @apply bg-transparent;
    @apply text-xl;
    @apply border-none;
    @apply outline-none;
    @apply focus:ring-0;
    color: rgba(255, 255, 255, 0.9);
    font-size: 1.25rem;
    padding: 0.5rem 0;
  }

  .launcher-input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .settings-button {
    @apply flex items-center justify-center;
    @apply p-2;
    @apply rounded-lg;
    @apply transition-colors;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    flex-shrink: 0;
  }

  .settings-button:hover {
    color: rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.1);
  }
</style>

