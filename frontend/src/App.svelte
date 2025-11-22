<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import ChatView from './components/ChatView.svelte';
  import LauncherView from './components/LauncherView.svelte';
  import SettingsPanel from './components/SettingsPanel.svelte';
  import DeleteModal from './components/DeleteModal.svelte';
  import WelcomeScreen from './components/WelcomeScreen.svelte';

  let query = '';
  let inputRef;
  let isChatMode = false;
  let conversations = [];
  let loading = false;
  let currentConversationId = null;
  let currentMessages = [];
  let loadingConversation = false;
  let activeSessions = new Set(); // Track which conversation IDs are currently streaming
  let eventListeners = new Map(); // Track event listeners per conversation for cleanup
  let messageCache = new Map(); // Cache messages for all conversations (conversationId -> messages[])
  let showSettings = false;
  let settingsPanelClosing = false;
  let settings = {
    'provider': 'ollama',
    'provider-params': {
      'ollama': { 'url': 'http://localhost:11434', 'model': 'gemma2:2b' },
      'openai': { 'api_key': '' },
      'google': { 'api_key': '' },
      'lmstudio': { 'url': 'http://localhost:1234', 'model': 'local-model' },
      'custom': { 'url': '' }
    },
    'keyboard-shortcut': 'Ctrl+Space',
    'setup-completed': false
  };
  let loadingSettings = false;
  let showDeleteAllModal = false;
  let shortcutError = '';
  let chatViewRef;
  let showWelcomeScreen = false;

  // Load conversation history from backend
  async function loadConversationHistory() {
    loading = true;
    try {
      const data = await invoke('get_conversation_history');
      conversations = data || [];
    } catch (error) {
      console.error('Error loading conversation history:', error);
      conversations = [];
    } finally {
      loading = false;
    }
  }

  // Global Escape key handler
  function handleGlobalKeydown(event) {
    if (event.key === 'Escape') {
      if (showSettings) {
        settingsPanelClosing = true;
        setTimeout(() => {
          showSettings = false;
          settingsPanelClosing = false;
        }, 300);
      } else if (isChatMode) {
        isChatMode = false;
        query = '';
        currentConversationId = null;
        currentMessages = [];
        loadConversationHistory();
      }
    }
  }

  onMount(async () => {
    if (inputRef) {
      inputRef.focus();
    }
    await loadSettings();
    loadConversationHistory();

    window.addEventListener('keydown', handleGlobalKeydown);
    
    const unlistenWindowShown = await listen('window-shown', () => {
      setTimeout(() => {
        if (inputRef) {
          inputRef.focus();
        }
      }, 50);
    });
    
    return () => {
      window.removeEventListener('keydown', handleGlobalKeydown);
      unlistenWindowShown();
    };
  });

  async function handleKeydown(event) {
    // Allow sending if no conversation is active, or if current conversation is not streaming
    const canSend = !currentConversationId || !activeSessions.has(currentConversationId);
    
    if (event.key === 'Enter' && !event.shiftKey && query.trim() && canSend) {
      event.preventDefault();
      
      const userMessage = query.trim();
      query = '';
      
      if (!isChatMode) {
        try {
          const conversationId = await invoke('create_conversation', { userMessage });
          await loadConversation(conversationId);
          await sendMessage(userMessage);
        } catch (error) {
          console.error('Error creating conversation:', error);
          isChatMode = true;
          currentConversationId = null;
          currentMessages = [];
        }
      } else {
        await sendMessage(userMessage);
      }
    }
  }

  // Stop streaming for current conversation
  async function stopStreaming() {
    if (!currentConversationId || !activeSessions.has(currentConversationId)) {
      return;
    }
    
    try {
      await invoke('stop_message_stream', { conversationId: currentConversationId });
    } catch (error) {
      console.error('Error stopping stream:', error);
    }
  }

  // Send a message to the current conversation (with streaming)
  async function sendMessage(userMessage) {
    if (!currentConversationId || activeSessions.has(currentConversationId)) {
      return;
    }
    
    // Mark this conversation as active
    activeSessions.add(currentConversationId);
    activeSessions = new Set(activeSessions); // Trigger reactivity by creating new Set
    
    try {
      const lastMessage = currentMessages[currentMessages.length - 1];
      const messageAlreadyAdded = lastMessage && 
                                  lastMessage.role === 'user' && 
                                  lastMessage.content === userMessage;
      
      if (!messageAlreadyAdded) {
        const userTimestamp = new Date().toISOString();
        const userMsg = {
          role: 'user',
          content: userMessage,
          timestamp: userTimestamp,
          complete: true
        };
        currentMessages = [...currentMessages, userMsg];
        
        // Update cache
        const cachedMessages = messageCache.get(currentConversationId) || [];
        messageCache.set(currentConversationId, [...cachedMessages, userMsg]);
        messageCache = new Map(messageCache); // Trigger reactivity
        
        if (chatViewRef) {
          chatViewRef.resetScrollState();
          chatViewRef.forceScrollToBottom();
        }
      }
      
      const assistantMsg = {
        role: 'assistant',
        content: 'loading-dots',
        timestamp: new Date().toISOString(),
        complete: false
      };
      currentMessages = [...currentMessages, assistantMsg];
      
      // Update cache with placeholder (use current messages as base)
      messageCache.set(currentConversationId, [...currentMessages]);
      messageCache = new Map(messageCache); // Trigger reactivity
      if (chatViewRef) {
        chatViewRef.resetScrollState();
        chatViewRef.forceScrollToBottom();
      }
      
      const eventName = `stream-chunk-${currentConversationId}`;
      let streamContent = '';
      let firstChunkReceived = false;
      
      const unlisten = await listen(eventName, async (event) => {
        const chunk = event.payload;
        const eventConvId = eventName.replace('stream-chunk-', '');
        
        if (chunk === 'DONE') {
          unlisten();
          // Remove from active sessions and clean up listener
          activeSessions.delete(eventConvId);
          activeSessions = new Set(activeSessions); // Trigger reactivity by creating new Set
          eventListeners.delete(eventConvId);
          // Clear cache for this conversation (will reload from disk on next view)
          messageCache.delete(eventConvId);
          messageCache = new Map(messageCache); // Trigger reactivity
          
          // If we're currently viewing this conversation, reload it from disk to get final complete state
          if (currentConversationId === eventConvId) {
            // Reload the conversation to get the complete message from disk
            const data = await invoke('load_conversation', { conversationId: eventConvId });
            currentMessages = data.messages || [];
            currentMessages = currentMessages; // Trigger reactivity
          }
          
          // Reload conversation history to update UI
          loadConversationHistory();
        } else if (chunk === 'CANCELLED') {
          unlisten();
          // Remove from active sessions and clean up listener
          activeSessions.delete(eventConvId);
          activeSessions = new Set(activeSessions); // Trigger reactivity by creating new Set
          eventListeners.delete(eventConvId);
          
          // If we're currently viewing this conversation, reload it from disk to get partial state
          if (currentConversationId === eventConvId) {
            // Reload the conversation to get the partial message from disk
            const data = await invoke('load_conversation', { conversationId: eventConvId });
            currentMessages = data.messages || [];
            currentMessages = currentMessages; // Trigger reactivity
          }
          
          // Reload conversation history to update UI
          loadConversationHistory();
        } else if (typeof chunk === 'string') {
          streamContent += chunk;
          
          // Only hide loading dots when we have actual content (non-empty after trimming)
          if (!firstChunkReceived && streamContent.trim().length > 0) {
            firstChunkReceived = true;
          }
          
          // Update cache for this conversation (regardless of which conversation is viewed)
          if (firstChunkReceived) {
            const cachedMessages = messageCache.get(eventConvId) || [];
            // Find the last assistant message and update it, or create new one
            let updatedMessages = [...cachedMessages];
            const lastIndex = updatedMessages.length - 1;
            if (lastIndex >= 0 && updatedMessages[lastIndex].role === 'assistant') {
              updatedMessages[lastIndex] = {
                ...updatedMessages[lastIndex],
                content: streamContent,
                complete: false // Mark as incomplete while streaming
              };
          } else {
              // No assistant message yet, add one
              updatedMessages.push({
                role: 'assistant',
                content: streamContent,
                timestamp: new Date().toISOString(),
                complete: false
              });
            }
            messageCache.set(eventConvId, updatedMessages);
            messageCache = new Map(messageCache); // Trigger reactivity
            
            // Also update currentMessages if this is the conversation being viewed
            if (currentConversationId === eventConvId) {
              // Find the last incomplete assistant message (the one we're streaming)
              const assistantMsgIndex = currentMessages.findLastIndex(m => 
                m.role === 'assistant' && m.complete === false
              );
              if (assistantMsgIndex >= 0) {
                currentMessages[assistantMsgIndex] = {
                  ...currentMessages[assistantMsgIndex],
                  content: streamContent
                };
                currentMessages = currentMessages;
              }
            }
          }
        }
      });
      
      // Store the listener for cleanup
      eventListeners.set(currentConversationId, unlisten);
      
      await invoke('send_message_stream', {
        conversationId: currentConversationId,
        userMessage: userMessage
      });
    } catch (error) {
      console.error('Error sending message:', error);
      currentMessages = currentMessages.slice(0, -1);
      
      const errorMsg = {
        role: 'assistant',
        content: `Error: ${error}`,
        timestamp: new Date().toISOString()
      };
      currentMessages = [...currentMessages, errorMsg];
      
      // Remove from active sessions on error
      activeSessions.delete(currentConversationId);
      activeSessions = new Set(activeSessions); // Trigger reactivity by creating new Set
      eventListeners.delete(currentConversationId);
    }
  }

  // Load settings from app local storage
  async function loadSettings(showLoading = true) {
    if (showLoading) {
      loadingSettings = true;
    }
    try {
      const data = await invoke('load_settings');
      
      const provider = String(data['provider'] ?? 'ollama');
      const providerParams = data['provider-params'] ?? {
        'ollama': { 'url': 'http://localhost:11434', 'model': 'gemma2:2b' },
        'openai': { 'api_key': '' },
        'google': { 'api_key': '' },
        'lmstudio': { 'url': 'http://localhost:1234', 'model': 'local-model' },
        'custom': { 'url': '' }
      };
      
      const keyboardShortcut = String(data['keyboard-shortcut'] ?? 'Ctrl+Space');
      const setupCompleted = Boolean(data['setup-completed'] ?? false);
      
      settings = {
        'provider': provider,
        'provider-params': providerParams,
        'keyboard-shortcut': keyboardShortcut,
        'setup-completed': setupCompleted
      };
      
      // Show welcome screen only if setup hasn't been completed
      showWelcomeScreen = !setupCompleted;
    } catch (error) {
      console.error('Error loading settings:', error);
      // On error, show welcome screen
      showWelcomeScreen = true;
    } finally {
      if (showLoading) {
        loadingSettings = false;
      }
    }
  }

  // Save settings to app local storage
  async function saveSettings() {
    try {
      await invoke('save_settings', { settingsMap: settings });
      await loadSettings(false);
      shortcutError = '';
    } catch (error) {
      console.error('Error saving settings:', error);
      if (error && typeof error === 'string' && error.includes('keyboard shortcut')) {
        shortcutError = error;
        await loadSettings(false);
        setTimeout(() => {
          shortcutError = '';
        }, 4000);
      }
    }
  }

  function handleSettingsClick() {
    if (!showSettings) {
      showSettings = true;
      settingsPanelClosing = false;
      loadSettings();
    } else {
      settingsPanelClosing = true;
      setTimeout(() => {
        showSettings = false;
        settingsPanelClosing = false;
      }, 300);
    }
  }

  function handleSettingChange(key, value) {
    settings = { ...settings, [key]: value };
    saveSettings();
  }

  async function handleBackClick() {
    isChatMode = false;
    query = '';
    currentConversationId = null;
    currentMessages = [];
    await loadConversationHistory();
  }

  // Load a conversation by ID from backend
  async function loadConversation(conversationId) {
    loadingConversation = true;
    try {
      const data = await invoke('load_conversation', { conversationId });
      
      // Note: We don't clean up listeners here because we want to keep receiving
      // updates even if we switch away and come back. The listeners are scoped
      // per conversation and will clean themselves up when streams complete.
      
      currentConversationId = data.id;
      
      // If conversation is actively streaming, merge cache with disk
      // Otherwise, use disk as source of truth and clear cache
      const diskMessages = data.messages || [];
      const isStreaming = activeSessions.has(conversationId);
      const cachedMessages = messageCache.get(conversationId);
      
      if (isStreaming && cachedMessages && cachedMessages.length > 0) {
        // Merge: use disk messages up to the last complete message,
        // then append cached updates
        const lastCompleteIndex = diskMessages.findLastIndex(m => m.complete !== false);
        const baseMessages = lastCompleteIndex >= 0 
          ? diskMessages.slice(0, lastCompleteIndex + 1)
          : diskMessages;
        
        // Append cached messages that are newer or incomplete
        const mergedMessages = [...baseMessages];
        for (const cachedMsg of cachedMessages) {
          const existingIndex = mergedMessages.findIndex(m => 
            m.role === cachedMsg.role && 
            m.timestamp === cachedMsg.timestamp
          );
          if (existingIndex >= 0) {
            // Update existing message with cached content
            mergedMessages[existingIndex] = cachedMsg;
          } else {
            // New message from cache
            mergedMessages.push(cachedMsg);
          }
        }
        currentMessages = mergedMessages;
      } else {
        // Not streaming, use disk as source of truth and clear cache
        currentMessages = diskMessages;
        if (cachedMessages) {
          messageCache.delete(conversationId);
          messageCache = new Map(messageCache); // Trigger reactivity
        }
      }
      
      isChatMode = true;
      query = '';
      
      if (chatViewRef) {
        await chatViewRef.forceScrollToBottom();
      }
    } catch (error) {
      console.error('Error loading conversation:', error);
      currentMessages = [];
    } finally {
      loadingConversation = false;
    }
  }

  function handleConversationClick(conversationId) {
    loadConversation(conversationId);
  }

  // Delete a single conversation
  async function handleDeleteConversation(conversationId, event) {
    event.stopPropagation();
    try {
      await invoke('delete_conversation', { conversationId: conversationId });
      await loadConversationHistory();
      if (currentConversationId === conversationId) {
        isChatMode = false;
        currentConversationId = null;
        currentMessages = [];
      }
    } catch (error) {
      console.error('Error deleting conversation:', error);
      alert(`Failed to delete conversation: ${error}`);
    }
  }

  // Show delete all confirmation modal
  function handleDeleteAllClick() {
    showDeleteAllModal = true;
  }

  // Confirm delete all
  async function confirmDeleteAll() {
    try {
      await invoke('delete_all_conversations');
      showDeleteAllModal = false;
      await loadConversationHistory();
      if (isChatMode) {
        isChatMode = false;
        currentConversationId = null;
        currentMessages = [];
      }
    } catch (error) {
      console.error('Error deleting all conversations:', error);
      alert(`Failed to delete all conversations: ${error}`);
      showDeleteAllModal = false;
    }
  }

  // Cancel delete all
  function cancelDeleteAll() {
    showDeleteAllModal = false;
  }

  // Dismiss welcome screen
  async function dismissWelcomeScreen() {
    showWelcomeScreen = false;
    // Mark setup as completed
    settings['setup-completed'] = true;
    await saveSettings();
  }
  
  // Redo setup
  function redoSetup() {
    showWelcomeScreen = true;
  }
</script>

{#if showWelcomeScreen}
  <WelcomeScreen onContinue={dismissWelcomeScreen} />
{:else}
<div class="launcher-window" class:chat-mode={isChatMode}>
    <div class="top-block">
      {#if isChatMode}
      <ChatView
        bind:this={chatViewRef}
        messages={currentMessages}
        {loadingConversation}
        bind:query
        bind:inputRef
        onBack={handleBackClick}
        onKeydown={handleKeydown}
        isStreaming={currentConversationId && activeSessions.has(currentConversationId)}
        onStop={stopStreaming}
      />
                  {:else}
      <LauncherView
        bind:query
        bind:inputRef
        {conversations}
        {loading}
        activeSessions={activeSessions}
        onKeydown={handleKeydown}
        onSettingsClick={handleSettingsClick}
        onConversationClick={handleConversationClick}
        onDeleteConversation={handleDeleteConversation}
        onDeleteAll={handleDeleteAllClick}
      />
        {/if}
    </div>

    {#if showSettings}
    <SettingsPanel
      show={showSettings}
      closing={settingsPanelClosing}
      loading={loadingSettings}
      bind:settings
                      {shortcutError}
      onClose={handleSettingsClick}
      onSettingChange={handleSettingChange}
      onSave={saveSettings}
      onRedoSetup={redoSetup}
    />
                  {/if}
            </div>
            
<DeleteModal
  show={showDeleteAllModal}
  onConfirm={confirmDeleteAll}
  onCancel={cancelDeleteAll}
/>
{/if}

<style>
  .launcher-window {
    @apply w-full;
    height: 100vh;
    @apply px-6 pb-6;
    background: rgba(20, 20, 20, 0.9);
    backdrop-filter: blur(24px) saturate(180%);
    -webkit-backdrop-filter: blur(24px) saturate(180%);
    display: flex;
    flex-direction: column;
    transition: height 0.3s ease, max-height 0.3s ease;
    position: relative;
    overflow: hidden;
    margin: 0;
  }

  :global(body) {
    position: relative;
  }

  .top-block {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }
</style>
