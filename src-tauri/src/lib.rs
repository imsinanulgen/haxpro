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
