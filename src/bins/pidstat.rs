pub static BIN_NAME: &'static str = "pidstat";
pub static CMD_CODE: &'static str = r#"
    
    COMMAND=id
    pidstat -e $COMMAND
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which pidstat) .

    COMMAND=id
    ./pidstat -e $COMMAND
"#;
pub static SUDO_CODE: &'static str = r#"
    
    COMMAND=id
    sudo pidstat -e $COMMAND
"#;
use crate::code::{Code, Tag};
pub static CMD: Code<'static> = Code { 
	title: "CMD_CODE",
	code: CMD_CODE,
	tag: Tag::CMD,
};
pub static SUID: Code<'static> = Code { 
	title: "SUID_CODE",
	code: SUID_CODE,
	tag: Tag::SUID,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
