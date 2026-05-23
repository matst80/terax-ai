// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    webkit2gtk_nvidia_quirk::apply_workaround_with_options(
        webkit2gtk_nvidia_quirk::ApplyWorkaroundOptions::default(),
    );

    terax_lib::run()
}
