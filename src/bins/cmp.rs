
pub static BIN_NAME: &'static str = "cmp";
pub static BIN_DESC: &'static str = "Dump the bytes of the input file that are different from the NUL byte in a tabular format, hence this may not be suitable to read arbitrary binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    cmp $LFILE /dev/zero -b -l
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which cmp) .

    LFILE=file_to_read
    ./cmp $LFILE /dev/zero -b -l
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo cmp $LFILE /dev/zero -b -l
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
