pub static BIN_NAME: &'static str = "split";
pub static SH_DESC: &'static str = "The shell prompt is not printed.";
pub static SH_CODE: &'static str = r#"
    
    split --filter=/bin/sh /dev/stdin
"#;
pub static CMD_DESC_1: &'static str = "Command execution using an existing or newly created file.";
pub static CMD_CODE_1: &'static str = r#"
    
    COMMAND=id
    TF=$(mktemp)
    split --filter=$COMMAND $TF
"#;
pub static CMD_DESC_2: &'static str = "Command execution using stdin (and close it directly).";
pub static CMD_CODE_2: &'static str = r#"
    
    COMMAND=id
    echo | split --filter=$COMMAND /dev/stdin
"#;
pub static FW_DESC_1: &'static str = "Data will be written in the current directory in a file named `xaa` by default. The input file will be split in multiple smaller files unless the `-b` option is used, pick a value in MB big enough.";
pub static FW_CODE_1: &'static str = r#"
    
    TF=$(mktemp)
    echo DATA >$TF
    split -b999m $TF
"#;
pub static FW_DESC_2: &'static str = "GNU version only. Data will be written in the current directory in a file named `xaa.xxx` by default. The input file will be split in multiple smaller files unless the `-b` option is used, pick a value in MB big enough.";
pub static FW_CODE_2: &'static str = r#"
    
    EXT=.xxx
    TF=$(mktemp)
    echo DATA >$TF
    split -b999m --additional-suffix $EXTENSION $TF
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    TF=$(mktemp)
    split $LFILE $TF
    cat $TF*
"#;
pub static SUDO_DESC: &'static str = "The shell prompt is not printed.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo split --filter=/bin/sh /dev/stdin
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static CMD_1: Code<'static> = Code {
    title: "CMD_CODE_1",
    code: CMD_CODE_1,
    tag: Tag::CMD,
};
pub static CMD_2: Code<'static> = Code {
    title: "CMD_CODE_2",
    code: CMD_CODE_2,
    tag: Tag::CMD,
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
