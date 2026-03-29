use tauri::{WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Attempt to bypass Chromium frame rate and vsync limitations (Windows specific but harmless on Mac)
    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--disable-frame-rate-limit --disable-gpu-vsync");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://www.haxball.com/play".parse().unwrap())
            )
            .fullscreen(true)
            .initialization_script(r#"
                // 1. Anti Throttling & Visibility Force
                window.addEventListener('blur', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
                window.addEventListener('focusout', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
                window.addEventListener('visibilitychange', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
                Object.defineProperty(document, 'visibilityState', { get: function() { return 'visible'; }, configurable: true });
                Object.defineProperty(document, 'hidden', { get: function() { return false; }, configurable: true });

                // 2. HackTimer (0ms precision for WebRTC and Physics sync via Web Worker)
                try {
                    const workerCode = `
                        let timers = {};
                        self.onmessage = function(e) {
                            if (e.data.command === 'setInterval') {
                                timers[e.data.id] = setInterval(() => postMessage({id: e.data.id}), e.data.timeout);
                            } else if (e.data.command === 'clearInterval') {
                                clearInterval(timers[e.data.id]);
                            } else if (e.data.command === 'setTimeout') {
                                timers[e.data.id] = setTimeout(() => postMessage({id: e.data.id}), e.data.timeout);
                            } else if (e.data.command === 'clearTimeout') {
                                clearTimeout(timers[e.data.id]);
                            }
                        };
                    `;
                    const blob = new Blob([workerCode], {type: 'application/javascript'});
                    const worker = new Worker(URL.createObjectURL(blob));
                    
                    let timerId = 1;
                    const callbacks = {};
                    
                    worker.onmessage = function(e) {
                        const id = e.data.id;
                        if (callbacks[id]) {
                            callbacks[id].fn.apply(null, callbacks[id].args);
                            if (!callbacks[id].isInterval) delete callbacks[id];
                        }
                    };

                    const _originalSetInterval = window.setInterval;
                    const _originalClearInterval = window.clearInterval;
                    const _originalSetTimeout = window.setTimeout;
                    const _originalClearTimeout = window.clearTimeout;

                    window.setInterval = function(fn, time, ...args) {
                        if (typeof fn !== 'function') return _originalSetInterval(fn, time, ...args);
                        const id = timerId++;
                        callbacks[id] = { fn, isInterval: true, args };
                        worker.postMessage({command: 'setInterval', id, timeout: time || 0});
                        return id;
                    };
                    window.clearInterval = function(id) {
                        if (callbacks[id]) {
                            delete callbacks[id];
                            worker.postMessage({command: 'clearInterval', id});
                        } else {
                            _originalClearInterval(id);
                        }
                    };
                    window.setTimeout = function(fn, time, ...args) {
                        if (typeof fn !== 'function') return _originalSetTimeout(fn, time, ...args);
                        const id = timerId++;
                        callbacks[id] = { fn, isInterval: false, args };
                        worker.postMessage({command: 'setTimeout', id, timeout: time || 0});
                        return id;
                    };
                    window.clearTimeout = function(id) {
                        if (callbacks[id]) {
                            delete callbacks[id];
                            worker.postMessage({command: 'clearTimeout', id});
                        } else {
                            _originalClearTimeout(id);
                        }
                    };
                } catch (e) { console.error("HackTimer failed", e); }

                // 3. Auto-Kick Anti-Ghosting Spam Macro (60Hz) & URL Navigator
                let activeMacros = {};
                // We monitor Space and X
                const MACRO_KEYS = ['Space', 'KeyX']; 
                
                window.addEventListener('keydown', function(e) {
                    // Custom URL Navigator (Cmd+L / Ctrl+L)
                    if ((e.metaKey || e.ctrlKey) && e.code === 'KeyL') {
                        e.preventDefault();
                        e.stopImmediatePropagation();
                        let newUrl = prompt("HaXball Özel Oda Linkini Yapıştırın:", "https://www.haxball.com/play?c=");
                        if (newUrl && newUrl.includes("haxball.com")) {
                            window.location.href = newUrl;
                        }
                        return;
                    }

                    // Macro processing
                    if (MACRO_KEYS.includes(e.code) && e.isTrusted) {
                        if (!activeMacros[e.code]) {
                            activeMacros[e.code] = setInterval(() => {
                                document.dispatchEvent(new KeyboardEvent('keydown', { key: e.key, code: e.code, keyCode: e.keyCode, bubbles: true }));
                                document.dispatchEvent(new KeyboardEvent('keyup',   { key: e.key, code: e.code, keyCode: e.keyCode, bubbles: true }));
                            }, 16);
                        }
                    }
                }, true);

                window.addEventListener('keyup', function(e) {
                    if (MACRO_KEYS.includes(e.code) && e.isTrusted) {
                        if (activeMacros[e.code]) {
                            clearInterval(activeMacros[e.code]);
                            delete activeMacros[e.code];
                        }
                    }
                }, true);
            "#)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
