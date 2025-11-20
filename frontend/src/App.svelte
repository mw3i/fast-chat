<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import ChatView from './components/ChatView.svelte';
  import LauncherView from './components/LauncherView.svelte';
  import SettingsPanel from './components/SettingsPanel.svelte';
  import DeleteModal from './components/DeleteModal.svelte';

  let query = '';
  let inputRef;
  let isChatMode = false;
  let conversations = [];
  let loading = false;
  let currentConversationId = null;
  let currentMessages = [];
  let loadingConversation = false;
  let sendingMessage = false;
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
    'keyboard-shortcut': 'Ctrl+Space'
  };
  let loadingSettings = false;
  let showDeleteAllModal = false;
  let shortcutError = '';
  let chatViewRef;

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
    if (event.key === 'Enter' && !event.shiftKey && query.trim() && !sendingMessage) {
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

  // Send a message to the current conversation (with streaming)
  async function sendMessage(userMessage) {
    if (!currentConversationId || sendingMessage) {
      return;
    }
    
    sendingMessage = true;
    
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
          timestamp: userTimestamp
        };
        currentMessages = [...currentMessages, userMsg];
        if (chatViewRef) {
          chatViewRef.resetScrollState();
          chatViewRef.forceScrollToBottom();
        }
      }
      
      const assistantMsg = {
        role: 'assistant',
        content: 'loading-dots',
        timestamp: new Date().toISOString()
      };
      currentMessages = [...currentMessages, assistantMsg];
      const assistantIndex = currentMessages.length - 1;
      if (chatViewRef) {
        chatViewRef.resetScrollState();
        chatViewRef.forceScrollToBottom();
      }
      
      const eventName = `stream-chunk-${currentConversationId}`;
      let streamContent = '';
      let firstChunkReceived = false;
      
      const unlisten = await listen(eventName, (event) => {
        const chunk = event.payload;
        if (chunk === 'DONE') {
          unlisten();
          sendingMessage = false;
        } else if (typeof chunk === 'string') {
          if (!firstChunkReceived) {
            firstChunkReceived = true;
            streamContent = chunk;
          } else {
            streamContent += chunk;
          }
          currentMessages[assistantIndex] = {
            ...currentMessages[assistantIndex],
            content: streamContent
          };
          currentMessages = currentMessages;
        }
      });
      
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
      sendingMessage = false;
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
      
      settings = {
        'provider': provider,
        'provider-params': providerParams,
        'keyboard-shortcut': keyboardShortcut
      };
    } catch (error) {
      console.error('Error loading settings:', error);
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
      
      currentConversationId = data.id;
      currentMessages = data.messages || [];
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
</script>

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
      />
                  {:else}
      <LauncherView
        bind:query
        bind:inputRef
        {conversations}
        {loading}
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
    />
                  {/if}
            </div>
            
<DeleteModal
  show={showDeleteAllModal}
  onConfirm={confirmDeleteAll}
  onCancel={cancelDeleteAll}
/>

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
    flex-shrink: 0;
    min-height: 0;
    overflow: hidden;
  }

  .launcher-window.chat-mode .top-block {
    flex: 1;
    min-height: 0;
  }
</style>
