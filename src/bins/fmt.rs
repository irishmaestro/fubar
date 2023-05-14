pub static BIN_NAME: &'static str = "fmt";
pub static BIN_DESC: &'static str = "The read file content is not binary-safe";
pub static FR_DESC_1: &'static str = "This only works for the GNU version of `fmt`.";
pub static FR_CODE_1: &'static str = r#"
    
    LFILE=file_to_read
    fmt -pNON_EXISTING_PREFIX "$LFILE"
"#;
pub static FR_DESC_2: &'static str =
    "This corrupts the output by wrapping very long lines at the given width.";
pub static FR_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    fmt -999 "$LFILE"
"#;
pub static SUID_DESC: &'static str =
    "This corrupts the output by wrapping very long lines at the given width.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which fmt) .

    LFILE=file_to_read
    ./fmt -999 "$LFILE"
"#;
pub static SUDO_DESC: &'static str =
    "This corrupts the output by wrapping very long lines at the given width.";
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo fmt -999 "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FR_1: Code<'static> = Code {
    title: "FR_CODE_1",
    code: FR_CODE_1,
    tag: Tag::FR,
};
pub static FR_2: Code<'static> = Code {
    title: "FR_CODE_2",
    code: FR_CODE_2,
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
