pub static BIN_NAME: &'static str = "mv";
pub static BIN_DESC: &'static str = "This can be used to move and then read or write files from a restricted file systems or with elevated privileges.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which mv) .

    LFILE=file_to_write
    TF=$(mktemp)
    echo "DATA" > $TF
    ./mv $TF $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo "DATA" > $TF
    sudo mv $TF $LFILE
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
