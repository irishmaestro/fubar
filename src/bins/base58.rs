
pub static BIN_NAME: &'static str = "base58";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    base58 "$LFILE" | base58 --decode
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo base58 "$LFILE" | base58 --decode
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
