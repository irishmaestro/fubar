
pub static BIN_NAME: &'static str = "check_log";
pub static BIN_DESC: &'static str =
    "This is the `check_log` Nagios plugin, available e.g. in `/usr/lib/nagios/plugins/`.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    INPUT=input_file
    check_log -F $INPUT -O $LFILE
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    OUTPUT=output_file
    check_log -F $LFILE -O $OUTPUT
    cat $OUTPUT
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_write
    INPUT=input_file
    sudo check_log -F $INPUT -O $LFILE
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
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
