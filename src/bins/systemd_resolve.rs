pub static BIN_NAME: &'static str = "systemd-resolve";
pub static SUDO_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo systemd-resolve --status
    !sh
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
