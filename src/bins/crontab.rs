
pub static BIN_NAME: &'static str = "crontab";
pub static CMD_DESC: &'static str =
    "The commands are executed according to the crontab file edited via the `crontab` utility.";
pub static CMD_CODE: &'static str = r#"
    
    crontab -e
"#;
pub static SUDO_DESC: &'static str =
    "The commands are executed according to the crontab file edited via the `crontab` utility.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo crontab -e
"#;
use crate::code::{Code, Tag};
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
