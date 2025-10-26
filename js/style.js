function resizeIframe() {
    const header = document.querySelector('header');
    const footer = document.querySelector('footer');
    const iframeContainer = document.querySelector('.iframe-container');
    const iframe = document.getElementById('main-iframe');
    
    // Calculate available height
    const headerHeight = header.offsetHeight;
    const footerHeight = footer.offsetHeight;
    const windowHeight = window.innerHeight;
    const availableHeight = windowHeight - headerHeight - footerHeight;
    
    // Set iframe container height
    iframeContainer.style.height = availableHeight + 'px';
    
    // Ensure iframe fills the container
    iframe.style.height = '100%';
}

// Initial resize
resizeIframe();

// Resize on window changes
window.addEventListener('resize', resizeIframe);
document.getElementById('main-iframe').onload = function() {
  console.log('Iframe loaded successfully');
};