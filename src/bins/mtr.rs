pub static BIN_NAME: &'static str = "mtr";
pub static BIN_DESC: &'static str = "The read file content is corrupted by error prints.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    mtr --raw -F "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo mtr --raw -F "$LFILE"
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
