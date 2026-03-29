# Haxball Architecture Lessons
- `rustup` should be installed locally inside the workspace to bypass strict `macOS` and `.npm` cache permission limits.
- Background throttling in Tauri depends heavily on OS-level thread management and disabling standard window constraints.
