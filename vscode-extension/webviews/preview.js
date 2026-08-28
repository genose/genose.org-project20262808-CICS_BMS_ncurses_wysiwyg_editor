// BMS Preview Webview Script
// Handles interactivity for the BMS map preview

document.addEventListener('DOMContentLoaded', () => {
    const grid = document.querySelector('.grid');
    const cells = document.querySelectorAll('.cell');
    
    // Add click handler for cells
    cells.forEach(cell => {
        cell.addEventListener('click', (e) => {
            // Toggle selection
            cell.classList.toggle('selected');
            
            // Notify parent (VSCode)
            const rect = cell.getBoundingClientRect();
            const fieldInfo = cell.title;
            
            if (fieldInfo) {
                console.log('Cell clicked:', fieldInfo);
            }
        });
        
        // Add hover effect
        cell.addEventListener('mouseenter', () => {
            cell.style.transform = 'scale(1.1)';
            cell.style.zIndex = '10';
        });
        
        cell.addEventListener('mouseleave', () => {
            cell.style.transform = '';
            cell.style.zIndex = '';
        });
    });
    
    // Handle keyboard navigation
    document.addEventListener('keydown', (e) => {
        // Could be used for future keyboard navigation
        console.log('Key pressed:', e.key);
    });
    
    // Notify that we're loaded
    vscode.postMessage({
        command: 'ready'
    });
});

// Handle messages from VSCode
window.addEventListener('message', event => {
    const message = event.data;
    switch (message.command) {
        case 'refresh':
            // Reload with new content
            document.querySelector('.bms-preview').innerHTML = message.html;
            break;
        case 'select':
            // Select a specific cell
            const cell = document.querySelector(`[title*="${message.fieldName}"]`);
            if (cell) {
                cell.classList.add('selected');
                cell.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
            break;
    }
});
