pub static BIN_NAME: &'static str = "redcarpet";
pub static BIN_DESC: &'static str = "The file is actually parsed as a markdown file.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    redcarpet "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo redcarpet "$LFILE"
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
