// inject.js — HaxPro v5.0 — Runs in page context (game iframe)
(function() {
    'use strict';

    if (window.__haxpro) return;
    window.__haxpro = true;

    var PHYSICS_TICK = 1000 / 60;
    var MACRO_KEYS = ['KeyX', 'KeyC'];
    var FRAME_ADVANTAGE = 8;
    var frameStats = { modified: 0, total: 0, lastOrig: 0, lastMod: 0 };

    // =============================================
    // 1. FRAME PRIORITY — WebRTC Paket Mudahalesi
    // =============================================
    var origSend = RTCDataChannel.prototype.send;
    RTCDataChannel.prototype.send = function(data) {
        try {
            if (data instanceof ArrayBuffer && data.byteLength > 9) {
                var view = new DataView(data);
                if (view.getUint8(0) === 1) {
                    var f = view.getInt32(1, false);
                    frameStats.total++;
                    frameStats.lastOrig = f;
                    if (f > FRAME_ADVANTAGE) {
                        view.setInt32(1, f - FRAME_ADVANTAGE, false);
                        frameStats.modified++;
                        frameStats.lastMod = f - FRAME_ADVANTAGE;
                    }
                }
            }
        } catch(e) {}
        return origSend.call(this, data);
    };

    // =============================================
    // 2. ANTI-THROTTLING
    // =============================================
    window.addEventListener('blur', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
    window.addEventListener('focusout', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
    window.addEventListener('visibilitychange', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
    Object.defineProperty(document, 'visibilityState', { get: function() { return 'visible'; }, configurable: true });
    Object.defineProperty(document, 'hidden', { get: function() { return false; }, configurable: true });

    // =============================================
    // 3. INTERPOLATION BYPASS
    // =============================================
    var origRAF = window.requestAnimationFrame;
    window.requestAnimationFrame = function(cb) {
        return origRAF.call(window, function(ts) {
            cb(Math.round(ts / PHYSICS_TICK) * PHYSICS_TICK);
        });
    };

    // =============================================
    // 4. DESYNCHRONIZED CANVAS
    // =============================================
    var origCtx = HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.getContext = function(type, attrs) {
        if (type === '2d' || type === 'webgl' || type === 'webgl2') {
            attrs = attrs || {};
            attrs.desynchronized = true;
            attrs.alpha = false;
        }
        return origCtx.call(this, type, attrs);
    };

    // =============================================
    // 5. TICK-ALIGNED MACRO (X ve C)
    // =============================================
    var tickOrigin = performance.now();
    origRAF.call(window, function(t) {
        tickOrigin = Math.floor(t / PHYSICS_TICK) * PHYSICS_TICK;
    });

    function dispatchAligned(key, code, keyCode) {
        var now = performance.now();
        var rem = PHYSICS_TICK - ((now - tickOrigin) % PHYSICS_TICK);
        var delay = rem > 2 ? rem - 1.5 : 0;
        setTimeout(function() {
            document.dispatchEvent(new KeyboardEvent('keydown', { key: key, code: code, keyCode: keyCode, bubbles: true, cancelable: true }));
            document.dispatchEvent(new KeyboardEvent('keyup', { key: key, code: code, keyCode: keyCode, bubbles: true, cancelable: true }));
        }, delay);
    }

    var activeMacros = {};
    window.addEventListener('keydown', function(e) {
        if (MACRO_KEYS.indexOf(e.code) !== -1 && e.isTrusted && !activeMacros[e.code]) {
            var k = e.key, c = e.code, kc = e.keyCode;
            dispatchAligned(k, c, kc);
            activeMacros[c] = setInterval(function() { dispatchAligned(k, c, kc); }, PHYSICS_TICK);
        }
    }, true);

    window.addEventListener('keyup', function(e) {
        if (MACRO_KEYS.indexOf(e.code) !== -1 && e.isTrusted && activeMacros[e.code]) {
            clearInterval(activeMacros[e.code]);
            delete activeMacros[e.code];
        }
    }, true);

    // =============================================
    // 6. CANLI IZLEME PANELI
    // =============================================
    function createMonitor() {
        if (!document.body || document.getElementById('haxpro-mon')) return;
        var mon = document.createElement('div');
        mon.id = 'haxpro-mon';
        mon.style.cssText = 'position:fixed;bottom:10px;right:10px;background:rgba(0,0,0,0.85);color:#fff;padding:10px 14px;border-radius:8px;font-family:monospace;font-size:12px;z-index:9999999;border:1px solid #333;line-height:1.6;pointer-events:none;min-width:210px;';
        document.body.appendChild(mon);
        setInterval(function() {
            if (frameStats.total === 0) {
                mon.innerHTML = '<span style="color:#FF9800;">Maca girin - bekleniyor...</span>';
            } else {
                mon.innerHTML =
                    '<span style="color:#4CAF50;font-weight:bold;">HaxPro v5.0 AKTIF</span><br>' +
                    'Paket: <span style="color:#64B5F6;">' + frameStats.total + '</span><br>' +
                    'Manipule: <span style="color:#FF5722;font-weight:bold;">' + frameStats.modified + '</span><br>' +
                    'Son: ' + frameStats.lastOrig + ' > <span style="color:#4CAF50;">' + frameStats.lastMod + '</span>' +
                    ' <span style="color:#FF5722;">(-' + FRAME_ADVANTAGE + ')</span>';
            }
        }, 250);
    }

    if (document.body) {
        createMonitor();
    } else {
        document.addEventListener('DOMContentLoaded', createMonitor);
    }

    console.log('%c HaxPro v5.0 INJECTED [' + window.location.pathname.substring(0, 30) + '] ', 'background:#2E7D32;color:#fff;font-size:13px;padding:3px 8px;border-radius:4px;');

})();
