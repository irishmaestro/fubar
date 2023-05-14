pub static BIN_NAME: &'static str = "torify";
pub static SH_CODE: &'static str = r#"
    
    torify /bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo torify /bin/sh
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
