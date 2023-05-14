pub static BIN_NAME: &'static str = "timedatectl";
pub static BIN_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.

This might not work if run by unprivileged users depending on the system configuration.";
pub static SH_CODE: &'static str = r#"
    
    timedatectl list-timezones
    !/bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo timedatectl list-timezones
    !/bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
