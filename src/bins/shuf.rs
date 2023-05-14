pub static BIN_NAME: &'static str = "shuf";
pub static FW_DESC: &'static str = "The written file content is corrupted by adding a newline.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    shuf -e DATA -o "$LFILE"
"#;
pub static FR_DESC: &'static str =
    "The read file content is corrupted by randomizing the order of NUL terminated strings.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    shuf -z "$LFILE"
"#;
pub static SUID_DESC: &'static str = "The written file content is corrupted by adding a newline.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which shuf) .

    LFILE=file_to_write
    ./shuf -e DATA -o "$LFILE"
"#;
pub static SUDO_DESC: &'static str = "The written file content is corrupted by adding a newline.";
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_write
    sudo shuf -e DATA -o "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
};
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
