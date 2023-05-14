pub static BIN_NAME: &'static str = "wall";
pub static BIN_DESC: &'static str =
    "The textual file is dumped on the current TTY (neither to `stdout` nor to `stderr`).";
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo wall --nobanner "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
