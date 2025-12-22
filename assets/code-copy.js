// Add copy buttons to all code blocks
document.addEventListener('DOMContentLoaded', () => {
    // Find all pre > code blocks
    const codeBlocks = document.querySelectorAll('pre > code');

    codeBlocks.forEach((codeBlock) => {
        const pre = codeBlock.parentElement;

        // Create wrapper div
        const wrapper = document.createElement('div');
        wrapper.className = 'code-block-wrapper';

        // Create copy button
        const copyButton = document.createElement('button');
        copyButton.className = 'code-copy-button';
        copyButton.innerHTML = `
            <svg class="copy-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
                <path d="M4 2h8a2 2 0 0 1 2 2v8M4 6H2a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-2"
                      stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            </svg>
            <span class="copy-text">Copy</span>
        `;
        copyButton.title = 'Copy code to clipboard';

        // Add click handler
        copyButton.addEventListener('click', async () => {
            const code = codeBlock.textContent;

            try {
                await navigator.clipboard.writeText(code);

                // Show success feedback
                copyButton.classList.add('success');
                copyButton.innerHTML = `
                    <svg class="check-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path d="M3 8l3 3 7-7" stroke="currentColor" stroke-width="2"
                              stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                    <span class="copy-text">Copied!</span>
                `;

                // Reset after 2 seconds
                setTimeout(() => {
                    copyButton.classList.remove('success');
                    copyButton.innerHTML = `
                        <svg class="copy-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
                            <path d="M4 2h8a2 2 0 0 1 2 2v8M4 6H2a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-2"
                                  stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                        </svg>
                        <span class="copy-text">Copy</span>
                    `;
                }, 2000);
            } catch (err) {
                // Show error feedback
                copyButton.classList.add('error');
                copyButton.innerHTML = `
                    <svg class="error-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path d="M8 1l7 14H1L8 1z" stroke="currentColor" stroke-width="1.5"
                              stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M8 6v4M8 12v.5" stroke="currentColor" stroke-width="1.5"
                              stroke-linecap="round"/>
                    </svg>
                    <span class="copy-text">Failed</span>
                `;

                // Reset after 2 seconds
                setTimeout(() => {
                    copyButton.classList.remove('error');
                    copyButton.innerHTML = `
                        <svg class="copy-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
                            <path d="M4 2h8a2 2 0 0 1 2 2v8M4 6H2a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-2"
                                  stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                        </svg>
                        <span class="copy-text">Copy</span>
                    `;
                }, 2000);
            }
        });

        // Wrap the pre element
        pre.parentNode.insertBefore(wrapper, pre);
        wrapper.appendChild(pre);
        wrapper.appendChild(copyButton);
    });
});
