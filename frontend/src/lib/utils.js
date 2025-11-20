import MarkdownIt from 'markdown-it';
import DOMPurify from 'dompurify';

const md = new MarkdownIt();

// Render markdown to HTML and sanitize it to prevent XSS
export function renderMarkdown(content) {
  if (!content) return '';
  try {
    let html = md.render(content);
    // Wrap tables in a scrollable container div
    html = html.replace(/<table>/g, '<div class="table-wrapper"><table>');
    html = html.replace(/<\/table>/g, '</table></div>');
    // Sanitize HTML to remove any potentially dangerous scripts/tags
    return DOMPurify.sanitize(html, {
      // Allow common markdown HTML tags but strip scripts and event handlers
      ALLOWED_TAGS: ['p', 'br', 'strong', 'em', 'u', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'ul', 'ol', 'li', 'blockquote', 'code', 'pre', 'a', 'img', 'table', 'thead', 'tbody', 'tr', 'th', 'td', 'div'],
      ALLOWED_ATTR: ['href', 'src', 'alt', 'title', 'class'],
    });
  } catch (e) {
    console.error('Error rendering markdown:', e);
    return content;
  }
}

// Format timestamp for display
export function formatTimestamp(timestamp) {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now - date;
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return 'Just now';
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;
  return date.toLocaleDateString();
}

