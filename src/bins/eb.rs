
pub static BIN_NAME: &'static str = "eb";
pub static BIN_DESC: &'static str = "This invokes the default logging service, which is likely to be `journalctl`, other functions may apply. For this to work the target must be connected to AWS instance via EB-CLI.";
pub static SH_CODE: &'static str = r#"
    
    eb logs
    !/bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo eb logs
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
