// Prism highlight retry script
// Fixes timing issues by re-running highlight after all resources are loaded
// This script is placed before Prism loads, so we use 'load' event instead of 'DOMContentLoaded'
window.addEventListener('load', () => {
    // Wait 100ms after page load to ensure Prism has initialized and all elements are ready
    setTimeout(() => {
        if (typeof Prism !== 'undefined') {
            Prism.highlightAll();
        }
    }, 100);
});
