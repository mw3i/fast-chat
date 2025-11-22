<script>
  import { Trash2 } from 'lucide-svelte';
  import ConversationItem from './ConversationItem.svelte';
  
  export let conversations = [];
  export let loading = false;
  export let activeSessions = new Set();
  export let onConversationClick;
  export let onDeleteConversation;
  export let onDeleteAll;
</script>

<div class="bottom-block">
  <div class="history-header">
    <span class="history-title">Recent Conversations</span>
    {#if conversations.length > 0}
      <button
        on:click={onDeleteAll}
        class="delete-all-button"
        title="Delete all conversations"
      >
        <Trash2 size={16} />
      </button>
    {/if}
  </div>
  <div class="history-list">
    {#if loading}
      <div class="history-item loading">Loading...</div>
    {:else if conversations.length === 0}
      <div class="history-empty">No conversations yet</div>
    {:else}
      {#each conversations as conv (conv.id)}
        <ConversationItem
          conversation={conv}
          isStreaming={activeSessions.has(conv.id)}
          onDelete={(e) => onDeleteConversation(conv.id, e)}
          onClick={() => onConversationClick(conv.id)}
        />
      {/each}
    {/if}
  </div>
</div>

<style>
  .bottom-block {
    @apply mt-4;
    @apply border-t;
    border-color: var(--border-primary);
    padding-top: 1rem;
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .history-header {
    @apply mb-3;
    @apply flex items-center justify-between;
  }

  .history-title {
    @apply text-sm;
    @apply font-medium;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .delete-all-button {
    @apply flex items-center justify-center;
    @apply p-2;
    @apply rounded-lg;
    @apply transition-colors;
    background: transparent;
    border: none;
    color: var(--error-color);
    cursor: pointer;
    flex-shrink: 0;
  }

  .delete-all-button:hover {
    background: rgba(239, 68, 68, 0.1);
    color: var(--error-hover);
  }

  .history-list {
    @apply flex flex-col gap-2;
    @apply overflow-y-auto;
    @apply pr-2;
    flex: 1;
  }

  .history-item.loading {
    @apply text-center;
    @apply py-4;
    color: var(--text-tertiary);
    cursor: default;
  }

  .history-empty {
    @apply text-center;
    @apply py-4;
    color: var(--text-tertiary);
    font-size: 0.875rem;
  }

  /* Scrollbar styling */
  .history-list::-webkit-scrollbar {
    width: 6px;
  }

  .history-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .history-list::-webkit-scrollbar-thumb {
    background: var(--scrollbar-thumb);
    border-radius: 3px;
  }

  .history-list::-webkit-scrollbar-thumb:hover {
    background: var(--scrollbar-thumb-hover);
  }
</style>

