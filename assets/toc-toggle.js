(function() {
  'use strict';

  const toggleButton = document.getElementById('toc-toggle');
  const STORAGE_KEY = 'unibook-sidebar-hidden';

  // Initialize sidebar state from localStorage
  function initializeSidebar() {
    const isHidden = localStorage.getItem(STORAGE_KEY) === 'true';
    if (isHidden) {
      document.body.classList.add('sidebar-hidden');
    }
  }

  // Toggle sidebar visibility
  function toggleSidebar() {
    const isHidden = document.body.classList.toggle('sidebar-hidden');
    localStorage.setItem(STORAGE_KEY, isHidden.toString());
  }

  // Set up event listener
  if (toggleButton) {
    toggleButton.addEventListener('click', toggleSidebar);
  }

  // Initialize on page load
  initializeSidebar();
})();
