/**
 * Clipboard functionality
 * Handles copying text to clipboard when elements with specific attributes are clicked.
 */

window.copyToClipboard = async function(text, btnElement) {
    try {
        await navigator.clipboard.writeText(text);
        
        // Show feedback
        const originalContent = btnElement.innerHTML;
        btnElement.innerHTML = '<i data-lucide="check" style="width:16px;height:16px;"></i> Copied!';
        btnElement.classList.add('success');
        
        // Re-render icons for the checkmark
        if (window.lucide) window.lucide.createIcons();
        
        setTimeout(() => {
            btnElement.innerHTML = originalContent;
            btnElement.classList.remove('success');
            if (window.lucide) window.lucide.createIcons();
        }, 2000);
        
    } catch (err) {
        console.error('Failed to copy:', err);
        // Fallback feedback
        const originalContent = btnElement.innerHTML;
        btnElement.innerHTML = '<i data-lucide="x" style="width:16px;height:16px;"></i> Error';
        if (window.lucide) window.lucide.createIcons();
        
        setTimeout(() => {
            btnElement.innerHTML = originalContent;
            if (window.lucide) window.lucide.createIcons();
        }, 2000);
    }
}
