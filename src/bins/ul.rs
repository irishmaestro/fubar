pub static BIN_NAME: &'static str = "ul";
pub static BIN_DESC: &'static str = r#"The read file content is corrupted by replacing occurrences of `$'\b_'` to terminal sequences and by converting tabs to spaces."#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ul "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ul) .

    LFILE=file_to_read
    ./ul "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo ul "$LFILE"
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
