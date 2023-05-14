pub static BIN_NAME: &'static str = "psql";
pub static BIN_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SH_CODE: &'static str = r#"
    
    psql
    \?
    !/bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    psql
    \?
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
