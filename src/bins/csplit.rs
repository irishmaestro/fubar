
pub static BIN_NAME: &'static str = "csplit";
pub static FW_DESC: &'static str = "Writes the data to `xx0file_to_write`. If needed, a different prefix can be specified with `-f` (instead of `xx`).";
pub static FW_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo "DATA" > $TF
    LFILE=file_to_write
    csplit -z -b "%d$LFILE" $TF 1
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    csplit $LFILE 1
    cat xx01
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which csplit) .

    LFILE=file_to_read
    csplit $LFILE 1
    cat xx01
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    csplit $LFILE 1
    cat xx01
"#;
use crate::code::{Code, Tag};
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
