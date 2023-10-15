use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn run() {
    let contents = "🚀 initial commit";

    let mut ctx = ClipboardContext::new().unwrap();

    ctx.set_contents(contents.to_owned()).unwrap();
    
    println!("Printed \"{}\" to the clipboard, just press paste!", contents);
}
