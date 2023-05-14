pub static BIN_NAME: &'static str = "xpad";
pub static BIN_DESC: &'static str =
    "This is a GUI application. The file content is displayed in a sticky note.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    xpad -f "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo xpad -f "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
