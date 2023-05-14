pub static BIN_NAME: &'static str = "grep";
pub static BIN_DESC: &'static str = "There are many `grep` flavors that in many cases are just copies, symlinks or wrappers around the original binary that may share the same behavior, for example: `egrep`, `fgrep`, `zgrep`, etc.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    grep '' $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which grep) .

    LFILE=file_to_read
    ./grep '' $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo grep '' $LFILE
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
