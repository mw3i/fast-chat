<script>
  export let show = false;
  export let onConfirm;
  export let onCancel;
</script>

{#if show}
  <div class="modal-overlay" on:click={onCancel} role="dialog" aria-modal="true" aria-labelledby="modal-title">
    <div class="modal-content" on:click|stopPropagation>
      <h3 id="modal-title" class="modal-title">Delete All Conversations?</h3>
      <p class="modal-message">Are you sure you want to delete all chats? There's no going back.</p>
      <div class="modal-actions">
        <button
          on:click={onCancel}
          class="modal-button modal-button-cancel"
        >
          Cancel
        </button>
        <button
          on:click={onConfirm}
          class="modal-button modal-button-confirm"
        >
          Delete All
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--bg-modal-overlay);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s ease;
  }

  .modal-content {
    background: var(--bg-modal);
    backdrop-filter: blur(24px) saturate(180%);
    -webkit-backdrop-filter: blur(24px) saturate(180%);
    border: 1px solid var(--border-primary);
    border-radius: 1rem;
    padding: 2rem;
    max-width: 400px;
    width: 90%;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
    animation: modalSlideIn 0.3s ease;
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

  @keyframes modalSlideIn {
    from {
      opacity: 0;
      transform: translateY(-20px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .modal-title {
    @apply text-xl;
    @apply font-semibold;
    color: var(--text-primary);
    margin: 0 0 1rem 0;
  }

  .modal-message {
    @apply text-base;
    color: var(--text-secondary);
    margin: 0 0 1.5rem 0;
    line-height: 1.5;
  }

  .modal-actions {
    @apply flex items-center justify-end gap-3;
  }

  .modal-button {
    @apply px-4 py-2;
    @apply rounded-lg;
    @apply font-medium;
    @apply transition-colors;
    border: none;
    cursor: pointer;
  }

  .modal-button-cancel {
    background: var(--bg-button);
    color: var(--text-primary);
  }

  .modal-button-cancel:hover {
    background: var(--bg-button-hover);
  }

  .modal-button-confirm {
    background: var(--error-color);
    color: white;
  }

  .modal-button-confirm:hover {
    background: var(--error-hover);
  }
</style>

