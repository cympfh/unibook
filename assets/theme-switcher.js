(function() {
  'use strict';

  const THEME_KEY = 'unibook-theme';
  let defaultTheme = 'light'; // Will be set from config

  // Get current theme (user preference or default)
  function getCurrentTheme() {
    return localStorage.getItem(THEME_KEY) || defaultTheme;
  }

  // Set theme
  function setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem(THEME_KEY, theme);
    updateActiveButton(theme);
  }

  // Update active button styling
  function updateActiveButton(theme) {
    document.querySelectorAll('.theme-option').forEach(btn => {
      if (btn.dataset.theme === theme) {
        btn.classList.add('active');
      } else {
        btn.classList.remove('active');
      }
    });
  }

  // Initialize theme on page load
  function initTheme() {
    // Read default theme from meta tag (set by unibook config)
    const metaTheme = document.querySelector('meta[name="unibook-theme"]');
    if (metaTheme) {
      defaultTheme = metaTheme.content;
    }

    const theme = getCurrentTheme();
    setTheme(theme);
  }

  // Toggle theme menu
  function toggleThemeMenu() {
    const menu = document.getElementById('theme-menu');
    if (menu.style.display === 'none' || menu.style.display === '') {
      menu.style.display = 'block';
    } else {
      menu.style.display = 'none';
    }
  }

  // Close theme menu when clicking outside
  function handleOutsideClick(event) {
    const switcher = document.getElementById('theme-switcher');
    const menu = document.getElementById('theme-menu');

    if (switcher && !switcher.contains(event.target)) {
      menu.style.display = 'none';
    }
  }

  // Initialize on DOM ready
  document.addEventListener('DOMContentLoaded', function() {
    initTheme();

    // Theme button click handler
    const themeButton = document.getElementById('theme-button');
    if (themeButton) {
      themeButton.addEventListener('click', function(e) {
        e.stopPropagation();
        toggleThemeMenu();
      });
    }

    // Theme option click handlers
    document.querySelectorAll('.theme-option').forEach(button => {
      button.addEventListener('click', function() {
        const theme = this.dataset.theme;
        setTheme(theme);
        document.getElementById('theme-menu').style.display = 'none';
      });
    });

    // Close menu when clicking outside
    document.addEventListener('click', handleOutsideClick);
  });
})();
