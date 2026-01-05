/**
 * Dropdown functionality
 * Handles toggling dropdown menus.
 */

document.addEventListener('click', function(e) {
    // Close all dropdowns when clicking outside
    if (!e.target.closest('.dropdown')) {
        document.querySelectorAll('.dropdown').forEach(el => {
            el.classList.remove('active');
        });
        return;
    }

    // Toggle dropdown on trigger click
    const trigger = e.target.closest('.dropdown-trigger');
    if (trigger) {
        const dropdown = trigger.closest('.dropdown');
        // Close other dropdowns first
        document.querySelectorAll('.dropdown').forEach(el => {
            if (el !== dropdown) el.classList.remove('active');
        });
        dropdown.classList.toggle('active');
    }
});
