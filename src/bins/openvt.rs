pub static BIN_NAME: &'static str = "openvt";
pub static SUDO_DESC: &'static str = "The command execution is blind (displayed on the virtual console), but it is possible to save the output on a temporary file.";
pub static SUDO_CODE: &'static str = r#"
    
    COMMAND=id
    TF=$(mktemp -u)
    sudo openvt -- sh -c "$COMMAND >$TF 2>&1"
    cat $TF
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
