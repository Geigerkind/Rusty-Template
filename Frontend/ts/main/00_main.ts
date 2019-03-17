// Will be inlined
declare var cssPath: string;

// Deferring load of non render critical css
{
let gos = document.createElement('link');
gos.rel = 'stylesheet';
gos.href = cssPath;
gos.type = 'text/css';
let godefer = document.getElementsByTagName('head')[0];
godefer.appendChild(gos);
}

// Serviceworker registration
if ('serviceWorker' in navigator) {
    window.addEventListener('load', function() {
      navigator.serviceWorker.register('/serviceworker.js').then(function(registration) {
        // Registration was successful
        console.log('ServiceWorker registration successful with scope: ', registration.scope);
      }, function(err) {
        // registration failed :(
        console.log('ServiceWorker registration failed: ', err);
      });
    });
  }