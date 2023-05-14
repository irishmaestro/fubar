
pub static BIN_NAME: &'static str = "crash";
pub static SH_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SH_CODE: &'static str = r#"
    
    crash -h
    !sh
"#;
pub static CMD_CODE: &'static str = r#"
    
    COMMAND='/usr/bin/id'
    CRASHPAGER="$COMMAND" crash -h
"#;
pub static SUDO_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo crash -h
    !sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static CMD: Code<'static> = Code { 
	title: "CMD_CODE",
	code: CMD_CODE,
	tag: Tag::CMD,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
