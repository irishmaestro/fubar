pub static BIN_NAME: &'static str = "apt";
pub static SH_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
static SH_CODE: &'static str = r#"
    
    apt changelog apt
    !/bin/sh
"#;
pub static SUDO_DESC_1: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
static SUDO_CODE_1: &'static str = r#"
    
    sudo apt changelog apt
    !/bin/sh
"#;
pub static SUDO_DESC_2: &'static str =
    "For this to work the target package (e.g., `sl`) must not be installed.";
static SUDO_CODE_2: &'static str = r#"
    
    TF=$(mktemp)
    echo 'Dpkg::Pre-Invoke {"/bin/sh;false"}' > $TF
    sudo apt install -c $TF sl
"#;
pub static SUDO_DESC_3: &'static str =
    "When the shell exits the `update` command is actually executed.";
static SUDO_CODE_3: &'static str = r#"
    
    sudo apt update -o APT::Update::Pre-Invoke::=/bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SHELL_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static SUDO_1: Code<'static> = Code {
    title: "SUDO_CODE_1",
    code: SUDO_CODE_1,
    tag: Tag::SUDO,
};
pub static SUDO_2: Code<'static> = Code {
    title: "SUDO_CODE_2",
    code: SUDO_CODE_2,
    tag: Tag::SUDO,
};
pub static SUDO_3: Code<'static> = Code {
    title: "SUDO_CODE_3",
    code: SUDO_CODE_3,
    tag: Tag::SUDO,
};
