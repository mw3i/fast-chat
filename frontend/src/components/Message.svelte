<script>
  import { renderMarkdown } from '../lib/utils.js';
  import { Copy, Check } from 'lucide-svelte';
  
  export let message;
  
  let copied = false;
  
  async function copyMessage() {
    try {
      await navigator.clipboard.writeText(message.content);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  }
</script>

<div class="message-wrapper" class:user-message={message.role === 'user'} class:ai-message={message.role === 'assistant'}>
  <div class="message" class:user-message={message.role === 'user'} class:ai-message={message.role === 'assistant'}>
    {#if message.role === 'assistant'}
      {#if message.content === 'loading-dots' || (message.complete === false && (!message.content || message.content.trim().length === 0))}
        <div class="message-content loading-dots">
          <span></span>
          <span></span>
          <span></span>
        </div>
      {:else}
        <div class="message-content markdown">{@html renderMarkdown(message.content)}</div>
        {#if message.complete === false}
          <div class="message-content loading-dots" style="margin-top: 0.5rem;">
            <span></span>
            <span></span>
            <span></span>
          </div>
        {/if}
      {/if}
    {:else}
      <div class="message-content">{message.content}</div>
    {/if}
    
    {#if message.content !== 'loading-dots' && message.role === 'assistant'}
      <button 
        class="copy-message-button"
        on:click={copyMessage}
        title={copied ? 'Copied!' : 'Copy message'}
      >
        {#if copied}
          <Check size={14} />
        {:else}
          <Copy size={14} />
        {/if}
      </button>
    {/if}
  </div>
  
  {#if message.content !== 'loading-dots' && message.role === 'user'}
    <button 
      class="copy-message-button copy-message-button-external"
      on:click={copyMessage}
      title={copied ? 'Copied!' : 'Copy message'}
    >
      {#if copied}
        <Check size={14} />
      {:else}
        <Copy size={14} />
      {/if}
    </button>
  {/if}
</div>

<style>
  .message {
    @apply rounded-lg;
    @apply p-4;
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .message.user-message {
    background: rgba(128, 128, 128, 0.3);
    border: none;
    color: rgba(255, 255, 255, 0.95);
    @apply p-3;
  }

  .message.ai-message {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.9);
  }

  .message-content {
    @apply text-base;
    line-height: 1.6;
    white-space: pre-wrap;
  }
  
  .message-content.loading-dots {
    @apply flex items-center gap-1;
    white-space: normal;
  }
  
  .message-content.loading-dots span {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.6);
    animation: loading-dot 1.4s infinite ease-in-out;
  }
  
  .message-content.loading-dots span:nth-child(1) {
    animation-delay: -0.32s;
  }
  
  .message-content.loading-dots span:nth-child(2) {
    animation-delay: -0.16s;
  }
  
  @keyframes loading-dot {
    0%, 80%, 100% {
      transform: scale(0.8);
      opacity: 0.5;
    }
    40% {
      transform: scale(1);
      opacity: 1;
    }
  }
  
  .message-content.markdown {
    white-space: normal;
  }
  
  .message-content.markdown :global(p) {
    margin: 0.5rem 0;
  }
  
  .message-content.markdown :global(p:first-child) {
    margin-top: 0;
  }
  
  .message-content.markdown :global(p:last-child) {
    margin-bottom: 0;
  }
  
  .message-content.markdown :global(code) {
    @apply bg-gray-800;
    @apply px-1.5 py-0.5;
    @apply rounded;
    @apply text-sm;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  }
  
  .message-content.markdown :global(pre) {
    @apply bg-gray-800;
    @apply p-4;
    @apply rounded-lg;
    @apply overflow-x-auto;
    margin: 1rem 0;
    position: relative !important;
    overflow-y: visible !important;
  }
  
  .copy-code-button {
    position: absolute !important;
    top: 0.5rem !important;
    right: 0.5rem !important;
    z-index: 10 !important;
    @apply p-2;
    @apply rounded;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    transition: all 0.2s;
    display: flex !important;
    align-items: center;
    justify-content: center;
    margin: 0 !important;
  }
  
  .copy-code-button:hover {
    background: rgba(255, 255, 255, 0.2);
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.3);
  }
  
  .copy-code-button:active {
    transform: scale(0.95);
  }
  
  .message-content.markdown :global(pre code) {
    @apply bg-transparent;
    @apply p-0;
  }
  
  .message-content.markdown :global(ul) {
    margin: 0.5rem 0;
    padding-left: 1.5rem;
  }
  
  .message-content.markdown :global(ol) {
    margin: 0.5rem 0;
    padding-left: 1.5rem;
  }
  
  .message-content.markdown :global(li) {
    margin: 0.25rem 0;
  }
  
  .message-content.markdown :global(h1) {
    @apply font-bold;
    @apply text-2xl;
    margin: 1rem 0 0.5rem 0;
  }
  
  .message-content.markdown :global(h2) {
    @apply font-bold;
    @apply text-xl;
    margin: 1rem 0 0.5rem 0;
  }
  
  .message-content.markdown :global(h3) {
    @apply font-bold;
    @apply text-lg;
    margin: 1rem 0 0.5rem 0;
  }
  
  .message-content.markdown :global(h4) {
    @apply font-bold;
    margin: 1rem 0 0.5rem 0;
  }
  
  .message-content.markdown :global(h5) {
    @apply font-bold;
    margin: 1rem 0 0.5rem 0;
  }
  
  .message-content.markdown :global(h6) {
    @apply font-bold;
    margin: 1rem 0 0.5rem 0;
  }
  
  .message-content.markdown :global(blockquote) {
    @apply border-l-4;
    border-color: rgba(255, 255, 255, 0.3);
    @apply pl-4;
    @apply italic;
    margin: 1rem 0;
  }
  
  .message-content.markdown :global(a) {
    @apply text-blue-400;
    @apply underline;
  }
  
  .message-content.markdown :global(strong) {
    @apply font-bold;
  }
  
  .message-content.markdown :global(em) {
    @apply italic;
    word-wrap: break-word;
  }
  
  .message-content.markdown :global(.table-wrapper) {
    @apply my-4;
    display: block;
    width: 100%;
    max-width: 100%;
    overflow-x: auto;
    overflow-y: visible;
    -webkit-overflow-scrolling: touch;
  }
  
  .message-content.markdown :global(.table-wrapper)::-webkit-scrollbar {
    height: 8px;
  }
  
  .message-content.markdown :global(.table-wrapper)::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 4px;
  }
  
  .message-content.markdown :global(.table-wrapper)::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 4px;
  }
  
  .message-content.markdown :global(.table-wrapper table) {
    @apply border-collapse;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 0.5rem;
    display: table;
    width: max-content;
    min-width: 100%;
    table-layout: auto;
  }
  
  .message-content.markdown :global(.table-wrapper table thead) {
    background: rgba(255, 255, 255, 0.05);
  }
  
  .message-content.markdown :global(.table-wrapper table th) {
    @apply px-1;
    @apply py-0.5;
    @apply text-left;
    @apply font-semibold;
    border-bottom: 2px solid rgba(255, 255, 255, 0.2);
    color: rgba(255, 255, 255, 0.95);
    white-space: nowrap;
  }
  
  .message-content.markdown :global(.table-wrapper table td) {
    @apply px-1;
    @apply py-0.5;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.9);
    white-space: nowrap;
  }
  
  .message-content.markdown :global(.table-wrapper table tbody tr:last-child td) {
    border-bottom: none;
  }
  
  .message-content.markdown :global(.table-wrapper table th:not(:last-child)),
  .message-content.markdown :global(.table-wrapper table td:not(:last-child)) {
    border-right: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .message-wrapper {
    @apply flex flex-col;
    @apply max-w-[85%];
  }
  
  .message-wrapper.user-message {
    @apply self-end items-end;
  }
  
  .message-wrapper.ai-message {
    @apply self-start items-start;
  }
  
  .copy-message-button {
    @apply flex items-center justify-center;
    @apply mt-2;
    @apply p-1.5;
    @apply rounded;
    @apply transition-colors;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    opacity: 0.7;
    min-width: 28px;
    min-height: 28px;
    box-sizing: border-box;
  }
  
  .copy-message-button-external {
    @apply self-end;
  }
  
  .message-wrapper:hover .copy-message-button {
    opacity: 1;
  }
  
  .copy-message-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.9);
  }
  
  .copy-message-button:active {
    transform: scale(0.95);
  }
</style>

