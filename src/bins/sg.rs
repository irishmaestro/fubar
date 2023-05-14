pub static BIN_NAME: &'static str = "sg";
pub static SH_DESC: &'static str = "Commands can be run if the current user’s group is specified, therefore no additional permissions are needed.";
pub static SH_CODE: &'static str = r#"
    
    sg $(id -ng)
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo sg root
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
