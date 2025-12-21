(function() {
  'use strict';

  let searchIndex = null;
  let selectedIndex = -1;

  // Load search index
  async function loadSearchIndex() {
    if (searchIndex) return searchIndex;

    try {
      const response = await fetch('search-index.json');
      searchIndex = await response.json();
      return searchIndex;
    } catch (error) {
      console.error('Failed to load search index:', error);
      return null;
    }
  }

  // Open search modal
  function openSearch() {
    const modal = document.getElementById('search-modal');
    const input = document.getElementById('search-input');

    modal.style.display = 'flex';
    input.focus();
    input.value = '';
    selectedIndex = -1;
    showHint();

    // Load index if not already loaded
    loadSearchIndex();
  }

  // Close search modal
  function closeSearch() {
    const modal = document.getElementById('search-modal');
    modal.style.display = 'none';
  }

  // Show hint message
  function showHint() {
    const results = document.getElementById('search-results');
    results.innerHTML = '<div class="search-hint">Start typing to search...</div>';
  }

  // Perform search
  function performSearch(query) {
    if (!query || query.length < 2) {
      showHint();
      return;
    }

    if (!searchIndex) {
      const results = document.getElementById('search-results');
      results.innerHTML = '<div class="search-hint">Loading search index...</div>';
      return;
    }

    const lowerQuery = query.toLowerCase();
    const results = [];

    for (const page of searchIndex.pages) {
      const titleMatch = page.title.toLowerCase().includes(lowerQuery);
      const contentMatch = page.content.toLowerCase().includes(lowerQuery);

      if (titleMatch || contentMatch) {
        // Find context snippet
        const preview = extractPreview(page.content, lowerQuery);
        results.push({
          title: page.title,
          url: page.url,
          preview: preview,
        });
      }
    }

    displayResults(results, query);
  }

  // Extract preview snippet around search term
  function extractPreview(content, query) {
    const index = content.toLowerCase().indexOf(query.toLowerCase());
    if (index === -1) return content.substring(0, 100) + '...';

    const start = Math.max(0, index - 40);
    const end = Math.min(content.length, index + query.length + 60);

    let preview = content.substring(start, end);
    if (start > 0) preview = '...' + preview;
    if (end < content.length) preview = preview + '...';

    return preview;
  }

  // Highlight search terms in text
  function highlightText(text, query) {
    const regex = new RegExp('(' + escapeRegex(query) + ')', 'gi');
    return text.replace(regex, '<span class="search-highlight">$1</span>');
  }

  function escapeRegex(str) {
    return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }

  // Display search results
  function displayResults(results, query) {
    const container = document.getElementById('search-results');
    selectedIndex = -1;

    if (results.length === 0) {
      container.innerHTML = '<div class="no-results">No results found</div>';
      return;
    }

    const html = results.map((result, index) => `
      <div class="search-result" data-index="${index}" data-url="${result.url}">
        <div class="search-result-title">${highlightText(result.title, query)}</div>
        <div class="search-result-preview">${highlightText(result.preview, query)}</div>
      </div>
    `).join('');

    container.innerHTML = html;

    // Add click handlers
    const resultElements = container.querySelectorAll('.search-result');
    resultElements.forEach((el) => {
      el.addEventListener('click', () => {
        window.location.href = el.dataset.url;
      });
    });
  }

  // Handle keyboard navigation
  function handleKeyDown(event) {
    const results = document.querySelectorAll('.search-result');

    if (event.key === 'ArrowDown') {
      event.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
      updateSelection(results);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, -1);
      updateSelection(results);
    } else if (event.key === 'Enter' && selectedIndex >= 0 && results[selectedIndex]) {
      event.preventDefault();
      window.location.href = results[selectedIndex].dataset.url;
    } else if (event.key === 'Escape') {
      event.preventDefault();
      closeSearch();
    }
  }

  // Update visual selection
  function updateSelection(results) {
    results.forEach((el, index) => {
      if (index === selectedIndex) {
        el.classList.add('selected');
        el.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
      } else {
        el.classList.remove('selected');
      }
    });
  }

  // Initialize search functionality
  document.addEventListener('DOMContentLoaded', function() {
    // Global keyboard shortcut
    document.addEventListener('keydown', function(event) {
      // Ctrl+K or Cmd+K
      if ((event.ctrlKey || event.metaKey) && event.key === 'k') {
        event.preventDefault();
        openSearch();
      }
    });

    // Search button in sidebar
    const searchButton = document.getElementById('search-button');
    if (searchButton) {
      searchButton.addEventListener('click', function() {
        openSearch();
      });
    }

    // Search input handler
    const searchInput = document.getElementById('search-input');
    if (searchInput) {
      searchInput.addEventListener('input', function(event) {
        performSearch(event.target.value);
      });

      searchInput.addEventListener('keydown', handleKeyDown);
    }

    // Close button
    const closeButton = document.getElementById('search-close');
    if (closeButton) {
      closeButton.addEventListener('click', closeSearch);
    }

    // Close on backdrop click
    const backdrop = document.querySelector('.search-backdrop');
    if (backdrop) {
      backdrop.addEventListener('click', closeSearch);
    }
  });
})();
