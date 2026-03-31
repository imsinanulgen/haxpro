use tauri::{WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
                (function() {
                    // 1. Anti Throttling & Visibility Force
                    window.addEventListener('blur', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
                    window.addEventListener('focusout', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
                    window.addEventListener('visibilitychange', function(e) { e.preventDefault(); e.stopImmediatePropagation(); }, true);
                    Object.defineProperty(document, 'visibilityState', { get: function() { return 'visible'; }, configurable: true });
                    Object.defineProperty(document, 'hidden', { get: function() { return false; }, configurable: true });

                    // 2. Auto-Kick Anti-Ghosting Spam Macro (60Hz)
                    var activeMacros = {};
                    var MACRO_KEYS = ['Space', 'KeyX'];

                    window.addEventListener('keydown', function(e) {
                        // Option+L (Alt+L) opens room navigator
                        if (e.altKey && e.code === 'KeyL') {
                            e.preventDefault();
                            e.stopImmediatePropagation();
                            showNav();
                            return;
                        }
                        // Macro processing
                        if (MACRO_KEYS.indexOf(e.code) !== -1 && e.isTrusted) {
                            if (!activeMacros[e.code]) {
                                activeMacros[e.code] = setInterval(function() {
                                    document.dispatchEvent(new KeyboardEvent('keydown', { key: e.key, code: e.code, keyCode: e.keyCode, bubbles: true }));
                                    document.dispatchEvent(new KeyboardEvent('keyup',   { key: e.key, code: e.code, keyCode: e.keyCode, bubbles: true }));
                                }, 16);
                            }
                        }
                    }, true);

                    window.addEventListener('keyup', function(e) {
                        if (MACRO_KEYS.indexOf(e.code) !== -1 && e.isTrusted) {
                            if (activeMacros[e.code]) {
                                clearInterval(activeMacros[e.code]);
                                delete activeMacros[e.code];
                            }
                        }
                    }, true);

                    // 3. Room Navigator UI
                    function showNav() {
                        var navBox = document.getElementById('haxpro-navbox');
                        if (!navBox) {
                            navBox = document.createElement('div');
                            navBox.id = 'haxpro-navbox';
                            navBox.style.cssText = 'position:fixed;top:50%;left:50%;transform:translate(-50%,-50%);background:rgba(15,15,15,0.95);padding:30px;border-radius:15px;z-index:9999999;border:1px solid #333;text-align:center;font-family:-apple-system,BlinkMacSystemFont,sans-serif;box-shadow:0 10px 40px rgba(0,0,0,0.8);backdrop-filter:blur(5px);';

                            var h = document.createElement('h3');
                            h.textContent = 'Oda Linkini Yapistiriniz';
                            h.style.cssText = 'color:#fff;margin-top:0;margin-bottom:20px;font-weight:600;';
                            navBox.appendChild(h);

                            var inp = document.createElement('input');
                            inp.type = 'text';
                            inp.id = 'haxpro-link-input';
                            inp.placeholder = 'https://www.haxball.com/play?c=...';
                            inp.style.cssText = 'display:block;width:380px;padding:12px;border-radius:8px;border:1px solid #444;margin-bottom:20px;font-size:15px;background:#222;color:#fff;outline:none;box-sizing:border-box;';
                            inp.autocomplete = 'off';
                            inp.spellcheck = false;
                            navBox.appendChild(inp);

                            var btnWrap = document.createElement('div');

                            var goBtn = document.createElement('button');
                            goBtn.textContent = 'GIT';
                            goBtn.style.cssText = 'padding:12px 30px;background:#2E7D32;color:#fff;border:none;border-radius:8px;cursor:pointer;font-weight:bold;font-size:14px;';
                            goBtn.onclick = function() {
                                var val = inp.value.trim();
                                if (val && val.indexOf('haxball.com') !== -1) {
                                    window.location.href = val;
                                }
                            };
                            btnWrap.appendChild(goBtn);

                            var closeBtn = document.createElement('button');
                            closeBtn.textContent = 'KAPAT';
                            closeBtn.style.cssText = 'padding:12px 30px;background:#D32F2F;color:#fff;border:none;border-radius:8px;cursor:pointer;font-weight:bold;margin-left:10px;font-size:14px;';
                            closeBtn.onclick = function() {
                                navBox.style.display = 'none';
                                inp.value = '';
                                window.focus();
                            };
                            btnWrap.appendChild(closeBtn);
                            navBox.appendChild(btnWrap);

                            inp.addEventListener('keydown', function(ev) { ev.stopPropagation(); if (ev.key === 'Enter') goBtn.click(); }, true);
                            inp.addEventListener('keyup', function(ev) { ev.stopPropagation(); }, true);

                            document.body.appendChild(navBox);
                        }
                        navBox.style.display = 'block';
                        var linkInput = document.getElementById('haxpro-link-input');
                        if (linkInput) setTimeout(function() { linkInput.focus(); }, 50);
                    }

                    // 4. Floating button on screen
                    function addFloatingButton() {
                        if (document.getElementById('haxpro-open-nav-btn')) return;
                        var openBtn = document.createElement('div');
                        openBtn.id = 'haxpro-open-nav-btn';
                        openBtn.textContent = 'Oda Degistir';
                        openBtn.style.cssText = 'position:fixed;top:15px;left:15px;background:rgba(0,0,0,0.6);color:#fff;padding:8px 12px;border-radius:6px;cursor:pointer;font-family:sans-serif;font-size:13px;z-index:9999998;border:1px solid #444;user-select:none;';
                        openBtn.onmouseover = function() { openBtn.style.background = 'rgba(0,0,0,0.9)'; };
                        openBtn.onmouseout  = function() { openBtn.style.background = 'rgba(0,0,0,0.6)'; };
                        openBtn.onclick = showNav;
                        document.body.appendChild(openBtn);
                    }

                    if (document.readyState === 'loading') {
                        document.addEventListener('DOMContentLoaded', addFloatingButton);
                    } else {
                        addFloatingButton();
                    }
                })();
            "#)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
