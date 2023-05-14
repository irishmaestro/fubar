
pub static BIN_NAME: &'static str = "exiftool";
pub static BIN_DESC: &'static str =
    "If the permissions allow it, files are moved (instead of copied) to the destination.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    INPUT=input_file
    exiftool -filename=$LFILE $INPUT
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    OUTPUT=output_file
    exiftool -filename=$OUTPUT $LFILE
    cat $OUTPUT
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_write
    INPUT=input_file
    sudo exiftool -filename=$LFILE $INPUT
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
