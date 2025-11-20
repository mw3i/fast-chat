<script>
  import { onMount, onDestroy, tick, afterUpdate } from 'svelte';
  import { ArrowLeft } from 'lucide-svelte';
  import Message from './Message.svelte';
  
  export let messages = [];
  export let loadingConversation = false;
  export let query = '';
  export let inputRef;
  export let onBack;
  export let onKeydown;
  export let onInputRef = (ref) => {};
  
  let chatMessagesRef;
  let userHasScrolledUp = false;
  let scrollListenerAttached = false;
  
  // Check if user is near the bottom of the chat
  function isNearBottom(threshold = 50) {
    if (!chatMessagesRef) return false;
    const { scrollTop, scrollHeight, clientHeight } = chatMessagesRef;
    return scrollHeight - scrollTop - clientHeight < threshold;
  }

  // Handle manual scroll - detect if user scrolls up
  function handleChatScroll() {
    if (!chatMessagesRef) return;
    
    if (!isNearBottom(150)) {
      userHasScrolledUp = true;
    } else if (isNearBottom(30)) {
      userHasScrolledUp = false;
    }
  }

  // Scroll to bottom only if user is already near the bottom
  async function scrollToBottomIfNeeded() {
    await tick();
    if (chatMessagesRef && !userHasScrolledUp && isNearBottom()) {
      chatMessagesRef.scrollTop = chatMessagesRef.scrollHeight;
    }
  }

  // Scroll chat messages to bottom (forced)
  async function scrollToBottom() {
    await tick();
    if (chatMessagesRef) {
      chatMessagesRef.scrollTop = chatMessagesRef.scrollHeight;
    }
  }

  // Add copy buttons to code blocks
  function addCopyButtonsToCodeBlocks() {
    if (!chatMessagesRef) return;
    
    const codeBlocks = chatMessagesRef.querySelectorAll('pre code');
    codeBlocks.forEach((codeBlock) => {
      const pre = codeBlock.parentElement;
      if (!pre || pre.querySelector('.copy-code-button')) return;
      
      const button = document.createElement('button');
      button.className = 'copy-code-button';
      button.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>';
      button.title = 'Copy code';
      
      button.style.position = 'absolute';
      button.style.top = '0.5rem';
      button.style.right = '0.5rem';
      button.style.zIndex = '10';
      
      button.addEventListener('click', async () => {
        const text = codeBlock.textContent || '';
        try {
          await navigator.clipboard.writeText(text);
          button.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>';
          button.title = 'Copied!';
          setTimeout(() => {
            button.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>';
            button.title = 'Copy code';
          }, 2000);
        } catch (err) {
          console.error('Failed to copy:', err);
        }
      });
      
      pre.style.position = 'relative';
      pre.style.overflowY = 'visible';
      pre.appendChild(button);
    });
  }

  // Add copy buttons after messages update
  afterUpdate(() => {
    if (messages.length > 0) {
      tick().then(() => {
        addCopyButtonsToCodeBlocks();
      });
    }
    
    // Manage scroll listener
    if (chatMessagesRef && !scrollListenerAttached) {
      chatMessagesRef.addEventListener('scroll', handleChatScroll, { passive: true });
      scrollListenerAttached = true;
    } else if (!chatMessagesRef && scrollListenerAttached) {
      if (chatMessagesRef) {
        chatMessagesRef.removeEventListener('scroll', handleChatScroll);
      }
      scrollListenerAttached = false;
    }
  });

  // Auto-scroll when messages change (only if near bottom)
  $: if (messages.length > 0 && !loadingConversation) {
    scrollToBottomIfNeeded();
  }

  // Expose scroll functions to parent
  export function resetScrollState() {
    userHasScrolledUp = false;
  }

  export function forceScrollToBottom() {
    scrollToBottom();
  }
</script>

<div class="chat-messages" bind:this={chatMessagesRef}>
  {#if loadingConversation}
    <div class="message ai-message">
      <div class="message-content">Loading conversation...</div>
    </div>
  {:else if messages.length > 0}
    {#each messages as message, index (message.timestamp + index)}
      <Message {message} />
    {/each}
  {:else if query.trim()}
    <div class="message user-message">
      <div class="message-content">{query}</div>
    </div>
    <div class="message ai-message">
      <div class="message-content">Thinking...</div>
    </div>
  {/if}
</div>

<div class="input-container">
  <button
    on:click={onBack}
    class="back-button"
    title="Back to conversations"
  >
    <ArrowLeft size={20} />
  </button>
  <input
    bind:this={inputRef}
    bind:value={query}
    on:keydown={onKeydown}
    type="text"
    placeholder="Type your message..."
    class="launcher-input"
    autofocus
  />
</div>

<style>
  .chat-messages {
    @apply flex flex-col gap-4;
    @apply overflow-y-auto;
    @apply pr-2;
    flex: 1;
    min-height: 0;
    padding-top: 1rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

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

  .back-button {
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

  .back-button:hover {
    color: rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.1);
  }

  .message {
    @apply rounded-lg;
    @apply p-4;
    @apply max-w-[85%];
  }

  .user-message {
    @apply self-end;
    background: rgba(59, 130, 246, 0.3);
    border: 1px solid rgba(59, 130, 246, 0.5);
    color: rgba(255, 255, 255, 0.95);
  }

  .ai-message {
    @apply self-start;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.9);
  }

  .message-content {
    @apply text-base;
    line-height: 1.6;
    white-space: pre-wrap;
  }

  /* Scrollbar styling */
  .chat-messages::-webkit-scrollbar {
    width: 6px;
  }

  .chat-messages::-webkit-scrollbar-track {
    background: transparent;
  }

  .chat-messages::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
  }

  .chat-messages::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }
</style>

