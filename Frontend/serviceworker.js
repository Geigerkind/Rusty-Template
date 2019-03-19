importScripts('https://storage.googleapis.com/workbox-cdn/releases/4.1.1/workbox-sw.js');

workbox.routing.registerRoute(
    new RegExp('/API/'),
    new workbox.strategies.NetworkOnly()
);

workbox.routing.registerRoute(
    /\.(?:png|gif|jpg|jpeg|svg|webp)$/,
    new workbox.strategies.CacheFirst({
        cacheName: 'images',
        plugins: [
        new workbox.expiration.Plugin({
            maxEntries: 1000,
            maxAgeSeconds: 30 * 24 * 60 * 60, // 30 Days
        }),
        ],
    })
);

workbox.routing.registerRoute(
    /\.(?:js|css|html)$/,
    new workbox.strategies.StaleWhileRevalidate({
        cacheName: 'static-resources',
    })
);
  
  