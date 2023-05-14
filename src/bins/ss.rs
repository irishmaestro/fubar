pub static BIN_NAME: &'static str = "ss";
pub static BIN_DESC: &'static str = "The file content is actually parsed so only a part of the first line is returned as a part of an error message.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ss -a -F $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ss) .

    LFILE=file_to_read
    ./ss -a -F $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo ss -a -F $LFILE
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
