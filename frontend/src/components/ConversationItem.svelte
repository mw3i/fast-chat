<script>
  import { X } from 'lucide-svelte';
  import { formatTimestamp } from '../lib/utils.js';
  
  export let conversation;
  export let isStreaming = false;
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
    <div class="history-item-title">
      {conversation.title}
      {#if isStreaming}
        <span class="streaming-indicator">
          <span class="streaming-dot"></span>
          <span class="streaming-dot"></span>
          <span class="streaming-dot"></span>
        </span>
      {/if}
    </div>
    <div class="history-item-meta">
      <span>{formatTimestamp(conversation.timestamp)}</span>
      <span>•</span>
      <span>{conversation.messageCount} messages</span>
      {#if isStreaming}
        <span class="streaming-label">Streaming...</span>
      {/if}
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
    @apply flex items-center gap-2;
    color: rgba(255, 255, 255, 0.9);
  }
  
  .streaming-indicator {
    @apply flex items-center gap-1;
  }
  
  .streaming-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: rgba(59, 130, 246, 0.8);
    animation: streaming-pulse 1.4s infinite ease-in-out;
  }
  
  .streaming-dot:nth-child(1) {
    animation-delay: -0.32s;
  }
  
  .streaming-dot:nth-child(2) {
    animation-delay: -0.16s;
  }
  
  @keyframes streaming-pulse {
    0%, 80%, 100% {
      transform: scale(0.8);
      opacity: 0.5;
    }
    40% {
      transform: scale(1);
      opacity: 1;
    }
  }
  
  .streaming-label {
    color: rgba(59, 130, 246, 0.8);
    font-weight: 500;
  }

  .history-item-meta {
    @apply text-xs;
    @apply flex items-center gap-2;
    color: rgba(255, 255, 255, 0.4);
  }
</style>

