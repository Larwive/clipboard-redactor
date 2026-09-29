use arboard::Clipboard;

pub fn init_clipboard() -> Clipboard {
    Clipboard::new().expect("Could not access the clipboard.")
}

pub fn read(clipboard: &mut Clipboard) -> String {
    match clipboard.get_text() {
        Ok(text) => text,
        Err(e)   => {eprintln!("failed: {e}"); String::new()},
    }
}

pub fn write(clipboard: &mut Clipboard, text: &str) -> () {
    clipboard.set_text(text).expect("Could not write into clipboard.")
}