pub static BIN_NAME: &'static str = "gcore";
pub static BIN_DISC: &'static str = "It can be used to generate core dumps of running processes. Such files often contains sensitive information such as open files content, cryptographic keys, passwords, etc. This command produces a binary file named `core.$PID`, that is then often filtered with `strings` to narrow down relevant information.";
pub static FR_CODE: &'static str = r#"
    
    gcore $PID
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which gcore) .

    ./gcore $PID
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo gcore $PID
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
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
