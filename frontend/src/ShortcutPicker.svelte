<script>
  import { invoke } from '@tauri-apps/api/core';
  
  export let value = '';
  export let placeholder = 'Press keys...';
  export let onSave = (shortcut) => {};
  
  let isListening = false;
  let capturedKeys = [];
  let displayValue = value;
  
  $: if (value !== displayValue && !isListening) {
    displayValue = value;
  }
  
  function formatShortcut(keys) {
    if (!keys || keys.length === 0) return '';
    
    const parts = [];
    
    // Add modifiers in standard order
    if (keys.includes('Meta')) parts.push('Cmd');
    if (keys.includes('Control')) parts.push('Ctrl');
    if (keys.includes('Alt')) parts.push('Alt');
    if (keys.includes('Shift')) parts.push('Shift');
    
    // Add the main key (exclude modifiers)
    const mainKey = keys.find(k => 
      !['Meta', 'Control', 'Alt', 'Shift'].includes(k)
    );
    
    if (mainKey) {
      // Format special keys nicely
      const keyMap = {
        ' ': 'Space',
        'ArrowUp': '↑',
        'ArrowDown': '↓',
        'ArrowLeft': '←',
        'ArrowRight': '→',
        'Enter': 'Enter',
        'Escape': 'Esc',
        'Tab': 'Tab',
        'Backspace': 'Backspace',
        'Delete': 'Delete',
        'Home': 'Home',
        'End': 'End',
        'PageUp': 'PageUp',
        'PageDown': 'PageDown',
      };
      
      parts.push(keyMap[mainKey] || mainKey);
    }
    
    return parts.join('+');
  }
  
  function handleKeyDown(event) {
    if (!isListening) return;
    
    event.preventDefault();
    event.stopPropagation();
    
    // Handle Escape to cancel
    if (event.key === 'Escape') {
      stopListening();
      return;
    }
    
    const keys = [];
    
    // Capture modifiers
    if (event.metaKey) keys.push('Meta');
    if (event.ctrlKey) keys.push('Control');
    if (event.altKey) keys.push('Alt');
    if (event.shiftKey) keys.push('Shift');
    
    // Capture the main key (skip if it's just a modifier)
    const key = event.key;
    const modifierKeys = ['Meta', 'Control', 'Alt', 'Shift', 'MetaLeft', 'MetaRight', 'ControlLeft', 'ControlRight', 'AltLeft', 'AltRight', 'ShiftLeft', 'ShiftRight'];
    
    if (key && !modifierKeys.includes(key)) {
      keys.push(key);
      capturedKeys = keys;
      displayValue = formatShortcut(keys);
      
      // Auto-save after a short delay
      setTimeout(async () => {
        if (isListening && keys.length > 0) {
          const shortcutString = formatShortcut(keys);
          onSave(shortcutString);
          await stopListening();
        }
      }, 100);
    }
  }
  
  async function startListening() {
    isListening = true;
    capturedKeys = [];
    displayValue = placeholder;
    
    // Notify backend that we're in listening mode
    try {
      await invoke('set_shortcut_listening_mode', { isListening: true });
    } catch (error) {
      console.error('Failed to set listening mode:', error);
    }
    
    // Focus the button to capture keyboard events
    // We'll use a global listener instead
    window.addEventListener('keydown', handleKeyDown, true);
  }
  
  async function stopListening() {
    isListening = false;
    
    // Notify backend that we're no longer in listening mode
    try {
      await invoke('set_shortcut_listening_mode', { isListening: false });
    } catch (error) {
      console.error('Failed to clear listening mode:', error);
    }
    
    window.removeEventListener('keydown', handleKeyDown, true);
    if (capturedKeys.length === 0) {
      displayValue = value; // Reset if nothing was captured
    }
  }
  
  async function handleClick() {
    if (isListening) {
      await stopListening();
    } else {
      await startListening();
    }
  }
  
  // Clean up listener on destroy
  import { onDestroy } from 'svelte';
  onDestroy(async () => {
    window.removeEventListener('keydown', handleKeyDown, true);
    // Make sure to clear listening mode when component is destroyed
    if (isListening) {
      try {
        await invoke('set_shortcut_listening_mode', { isListening: false });
      } catch (error) {
        console.error('Failed to clear listening mode on destroy:', error);
      }
    }
  });
</script>

<button
  type="button"
  class="shortcut-picker"
  class:listening={isListening}
  on:click={handleClick}
  on:blur={stopListening}
>
  {#if isListening}
    <span class="shortcut-placeholder">{displayValue}</span>
  {:else}
    <span class="shortcut-value">{displayValue || placeholder}</span>
  {/if}
</button>

<style>
  .shortcut-picker {
    @apply px-4 py-2;
    @apply rounded-lg;
    @apply bg-transparent;
    @apply border;
    @apply outline-none;
    @apply transition-colors;
    @apply text-left;
    @apply cursor-pointer;
    @apply font-mono;
    @apply text-sm;
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
    min-width: 150px;
  }
  
  .shortcut-picker:focus,
  .shortcut-picker.listening {
    border-color: rgba(59, 130, 246, 0.5);
    background: rgba(255, 255, 255, 0.08);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }
  
  .shortcut-picker:hover:not(.listening) {
    border-color: rgba(255, 255, 255, 0.3);
    background: rgba(255, 255, 255, 0.08);
  }
  
  .shortcut-value {
    color: rgba(255, 255, 255, 0.9);
  }
  
  .shortcut-placeholder {
    color: rgba(59, 130, 246, 0.8);
    animation: pulse 1.5s ease-in-out infinite;
  }
  
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
