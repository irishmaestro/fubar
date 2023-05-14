pub static BIN_NAME: &'static str = "troff";
pub static BIN_DESC: &'static str = "The file is typeset but text is still readable in the output, alternatively the output can be read with `man -l`.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    troff $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which troff) .

    LFILE=file_to_read
    ./troff $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo troff $LFILE
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
