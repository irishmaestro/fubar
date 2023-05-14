pub static BIN_NAME: &'static str = "chmod";
pub static BIN_DESC: &'static str = "This can be run with elevated privileges to change permissions (6 denotes the SUID bits) and then read, write, or execute a file.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which chmod) .

    LFILE=file_to_change
    ./chmod 6777 $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_change
    sudo chmod 6777 $LFILE
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
