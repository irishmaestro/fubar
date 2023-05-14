pub static BIN_NAME: &'static str = "strings";
pub static BIN_DESC: &'static str =
    "This only returns ASCII strings, thus it is not suitable for binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    strings "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which strings) .

    LFILE=file_to_read
    ./strings "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo strings "$LFILE"
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
