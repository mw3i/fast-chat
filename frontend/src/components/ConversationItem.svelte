<script>
  import { X } from 'lucide-svelte';
  import { formatTimestamp } from '../lib/utils.js';
  
  export let conversation;
  export let onDelete;
  export let onClick;
</script>

<div 
  class="history-item" 
  role="button" 
  tabindex="0"
  on:click={onClick}
  on:keydown={(e) => e.key === 'Enter' && onClick()}
>
  <div class="history-item-content">
    <div class="history-item-title">{conversation.title}</div>
    <div class="history-item-meta">
      <span>{formatTimestamp(conversation.timestamp)}</span>
      <span>•</span>
      <span>{conversation.messageCount} messages</span>
    </div>
  </div>
  <button
    on:click={(e) => onDelete(e)}
    class="delete-conversation-button"
    title="Delete conversation"
  >
    <X size={16} />
  </button>
</div>

<style>
  .history-item {
    @apply p-3;
    @apply rounded-lg;
    @apply cursor-pointer;
    @apply transition-colors;
    @apply flex items-center justify-between gap-3;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .history-item:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.15);
  }

  .history-item-content {
    @apply flex-1;
    @apply min-w-0;
  }

  .delete-conversation-button {
    @apply flex items-center justify-center;
    @apply p-1.5;
    @apply rounded;
    @apply transition-colors;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.4);
    cursor: pointer;
    flex-shrink: 0;
    opacity: 0;
  }

  .history-item:hover .delete-conversation-button {
    opacity: 1;
  }

  .delete-conversation-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
  }

  .history-item-title {
    @apply text-base;
    @apply font-medium;
    @apply mb-1;
    color: rgba(255, 255, 255, 0.9);
  }

  .history-item-meta {
    @apply text-xs;
    @apply flex items-center gap-2;
    color: rgba(255, 255, 255, 0.4);
  }
</style>

