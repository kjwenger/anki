// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

import { build, files, version } from '$service-worker';

const CACHE = `cache-${version}`;

const ASSETS = [
  ...build, // the app itself
  ...files  // everything in `static`
];

self.addEventListener('install', (event) => {
  async function addFilesToCache() {
    const cache = await caches.open(CACHE);
    await cache.addAll(ASSETS);
  }

  event.waitUntil(addFilesToCache());
  // Force the waiting service worker to become the active service worker.
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  async function deleteOldCaches() {
    for (const key of await caches.keys()) {
      if (key !== CACHE) await caches.delete(key);
    }
  }

  event.waitUntil(deleteOldCaches());
  // Become the active service worker for all clients of this scope.
  self.clients.claim();
});

self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET' || event.request.headers.has('range')) return;

  const url = new URL(event.request.url);

  // Skip non-http requests and API calls
  if (!url.protocol.startsWith('http') || url.pathname.startsWith('/api/')) return;

  async function respond() {
    const cache = await caches.open(CACHE);

    // 1. For pre-cached assets, try cache first
    if (ASSETS.includes(url.pathname)) {
      const cachedResponse = await cache.match(event.request);
      if (cachedResponse) return cachedResponse;
    }

    // 2. Try the network
    try {
      const response = await fetch(event.request);

      // Cache successful same-origin responses
      if (response.status === 200 && response.type === 'basic') {
        cache.put(event.request, response.clone());
      }

      return response;
    } catch (err) {
      // 3. If the network fails, fall back to the cache
      const cachedResponse = await cache.match(event.request);
      if (cachedResponse) return cachedResponse;

      // If we have nothing, throw the original error
      throw err;
    }
  }

  event.respondWith(respond());
});
