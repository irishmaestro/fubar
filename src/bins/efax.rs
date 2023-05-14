
pub static BIN_NAME: &'static str = "efax";
pub static BIN_DESC: &'static str = "The content is actually parsed by the command, thus it may not be suitable for reading arbitrary files.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which efax) .

    LFILE=file_to_read
    ./efax -d "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo efax -d "$LFILE"
"#;
use crate::code::{Code, Tag};
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
