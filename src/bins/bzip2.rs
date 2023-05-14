
pub static BIN_NAME: &'static str = "bzip2";
pub static BIN_DESC: &'static str = "There are also a number of other utilities that rely on `bzip2` under the hood, e.g., `bzless`, `bzcat`, `bunzip2`, etc. Besides having similar features, they also allow privileged reads if `bzip2` itself is SUID.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    bzip2 -c $LFILE | bzip2 -d
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which bzip2) .

    LFILE=file_to_read
    ./bzip2 -c $LFILE | bzip2 -d
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo bzip2 -c $LFILE | bzip2 -d
"#;
use crate::code::{Code, Tag};
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
