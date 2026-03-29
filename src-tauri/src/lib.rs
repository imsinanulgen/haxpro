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

                // 2. [REMOVED HACKTIMER TO PREVENT NETWORK DESYNC]

                // 3. Auto-Kick Anti-Ghosting Spam Macro (60Hz) & URL Navigator
                let activeMacros = {};
                // We monitor Space and X
                const MACRO_KEYS = ['Space', 'KeyX']; 
                
                window.addEventListener('keydown', function(e) {
                    // Custom URL Navigator (Cmd+L / Ctrl+L) via Custom HTML Dialog
                    if ((e.metaKey || e.ctrlKey) && e.code === 'KeyL') {
                        e.preventDefault();
                        e.stopImmediatePropagation();
                        
                        let navBox = document.getElementById('haxpro-navbox');
                        if (!navBox) {
                            navBox = document.createElement('div');
                            navBox.id = 'haxpro-navbox';
                            navBox.style.cssText = 'position:fixed;top:50%;left:50%;transform:translate(-50%,-50%);background:rgba(15,15,15,0.95);padding:30px;border-radius:15px;z-index:9999999;border:1px solid #333;text-align:center;font-family:-apple-system,BlinkMacSystemFont,sans-serif;box-shadow: 0 10px 40px rgba(0,0,0,0.8);backdrop-filter:blur(5px);';
                            navBox.innerHTML = `
                                <h3 style="color:#fff;margin-top:0;margin-bottom:20px;font-weight:600;">Oda Linkini Yapıştırın</h3>
                                <input type="text" id="haxpro-link-input" placeholder="https://www.haxball.com/play?c=..." style="width:380px;padding:12px;border-radius:8px;border:1px solid #444;margin-bottom:20px;font-size:15px;background:#222;color:#fff;outline:none;box-sizing:border-box;" autocomplete="off" spellcheck="false" />
                                <br/>
                                <button id="haxpro-join-btn" style="padding:12px 30px;background:#2E7D32;color:#fff;border:none;border-radius:8px;cursor:pointer;font-weight:bold;font-size:14px;transition:0.2s;">GİT</button>
                                <button id="haxpro-close-btn" style="padding:12px 30px;background:#D32F2F;color:#fff;border:none;border-radius:8px;cursor:pointer;font-weight:bold;margin-left:10px;font-size:14px;">KAPAT</button>
                            `;
                            document.body.appendChild(navBox);

                            document.getElementById('haxpro-join-btn').onclick = function() {
                                let val = document.getElementById('haxpro-link-input').value.trim();
                                if (val && val.includes('haxball.com')) window.location.href = val;
                            };
                            document.getElementById('haxpro-close-btn').onclick = function() {
                                navBox.style.display = 'none';
                                document.getElementById('haxpro-link-input').value = '';
                                window.focus(); // Give focus back to game
                            };
                            // Prevent Haxball catching keys in our input
                            document.getElementById('haxpro-link-input').addEventListener('keydown', function(ev) {
                                ev.stopPropagation();
                                if (ev.key === 'Enter') document.getElementById('haxpro-join-btn').click();
                            }, true);
                            document.getElementById('haxpro-link-input').addEventListener('keyup', function(ev) {
                                ev.stopPropagation();
                            }, true);
                        }
                        
                        navBox.style.display = 'block';
                        setTimeout(() => document.getElementById('haxpro-link-input').focus(), 50);
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
