pub static BIN_NAME: &'static str = "pkexec";
pub static SUDO_CODE: &'static str = r#"
    
    sudo pkexec /bin/sh
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
