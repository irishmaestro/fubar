pub static BIN_NAME: &'static str = "zathura";
pub static BIN_DESC: &'static str =
    "The interaction happens in a GUI window, while the shell is dropped in the terminal.";
pub static SH_CODE: &'static str = r#"
    
    zathura
    :! /bin/sh -c 'exec /bin/sh 0<&1'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo zathura
    :! /bin/sh -c 'exec /bin/sh 0<&1'
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
