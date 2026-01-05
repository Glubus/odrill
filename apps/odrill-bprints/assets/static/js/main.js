/**
 * Main application entry point
 * Initializes icons and global behaviors.
 */

document.addEventListener('DOMContentLoaded', () => {
    // Initialize Lucide icons
    if (window.lucide) {
        window.lucide.createIcons();
    }
});
