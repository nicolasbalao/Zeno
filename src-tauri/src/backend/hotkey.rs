use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

pub fn setup_hotkey<F>(callback: F) -> Result<(), String>
where
    F: Fn() + Send + 'static,
{
    let mut hkm = HotkeyManager::new();

    match hkm.register(VKey::Return, &[ModKey::Ctrl], callback) {
        Ok(_) => {
            println!("Hotkey registered: Ctrl + Enter");
            hkm.event_loop();
            Ok(())
        }
        Err(e) => Err(format!("Failed to register hotkey: {:?}", e)),
    }
}
