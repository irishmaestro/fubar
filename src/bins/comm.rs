
pub static BIN_NAME: &'static str = "comm";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    comm $LFILE /dev/null 2>/dev/null
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which comm) .

    LFILE=file_to_read
    comm $LFILE /dev/null 2>/dev/null
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo comm $LFILE /dev/null 2>/dev/null
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
