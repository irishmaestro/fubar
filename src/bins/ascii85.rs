pub static BIN_NAME: &'static str = "ascii85";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ascii85 "$LFILE" | ascii85 --decode
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo ascii85 "$LFILE" | ascii85 --decode
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
