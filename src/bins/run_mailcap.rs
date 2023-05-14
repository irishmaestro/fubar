pub static BIN_NAME: &'static str = "run-mailcap";
pub static SH_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SH_CODE: &'static str = r#"
    
    run-mailcap --action=view /etc/hosts
    !/bin/sh
"#;
pub static FW_DESC: &'static str = "The file must exist and not be empty.

This invokes the default editor, which is likely to be `vi`, other functions may apply.";
pub static FW_CODE: &'static str = r#"
    
    run-mailcap --action=edit file_to_read
"#;
pub static FR_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static FR_CODE: &'static str = r#"
    
    run-mailcap --action=view file_to_read
"#;
pub static SUDO_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo run-mailcap --action=view /etc/hosts
    !/bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
