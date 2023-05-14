pub static BIN_NAME: &'static str = "alpine";
pub static BIN_DESC: &'static str = "The file is displayed in the alpine curses terminal interface. Other options might be available, for example by pressing S is possible to save the file content elsewhere.";

static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    alpine -F "$LFILE"
"#;
static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which alpine) .

    LFILE=file_to_read
    ./alpine -F "$LFILE"
"#;
static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo alpine -F "$LFILE"
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
