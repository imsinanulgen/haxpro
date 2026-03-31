// content.js — Runs in EVERY frame (including game iframe)
// Injects inject.js into the page's JavaScript context
var s = document.createElement('script');
s.src = chrome.runtime.getURL('inject.js');
s.onload = function() { s.remove(); };
(document.head || document.documentElement).appendChild(s);
