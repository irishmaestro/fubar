pub static BIN_NAME: &'static str = "tac";
pub static BIN_DESC: &'static str = "Make sure that `RANDOM` does not appear in the file to read, otherwise the content of the file is corrupted by reversing the order of `RANDOM`-separated chunks.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    tac -s 'RANDOM' "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which tac) .

    LFILE=file_to_read
    ./tac -s 'RANDOM' "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo tac -s 'RANDOM' "$LFILE"
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
