pub static BIN_NAME: &'static str = "diff";
pub static FR_CODE_1: &'static str = r#"
    
    LFILE=file_to_read
    diff --line-format=%L /dev/null $LFILE
"#;
pub static FR_DESC_2: &'static str = "
This lists the content of a directory. `$TF` can be any directory, but for convenience it is better to use an empty directory to avoid noise output.";
pub static FR_CODE_2: &'static str = r#"
    
    LFOLDER=folder_to_list
    TF=$(mktemp -d)
    diff --recursive $TF $LFOLDER
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which diff) .

    LFILE=file_to_read
    ./diff --line-format=%L /dev/null $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo diff --line-format=%L /dev/null $LFILE
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
