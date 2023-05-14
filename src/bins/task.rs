pub static BIN_NAME: &'static str = "task";
pub static SH_CODE: &'static str = r#"
    
    task execute /bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo task execute /bin/sh
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
