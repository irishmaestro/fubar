
pub static BIN_NAME: &'static str = "check_ssl_cert";
pub static BIN_DESC: &'static str =
    "This is the `check_by_ssh` Nagios plugin, available e.g. in `/usr/lib/nagios/plugins/`.";
pub static CMD_DESC: &'static str = "The host example.net must return a certificate via TLS";
pub static CMD_CODE: &'static str = r#"
    
    COMMAND=id
    OUTPUT=output_file
    TF=$(mktemp)
    echo "$COMMAND | tee $OUTPUT" > $TF
    chmod +x $TF
    check_ssl_cert --curl-bin $TF -H example.net
    cat $OUTPUT
"#;
pub static SUDO_DESC: &'static str = "The host example.net must return a certificate via TLS";
pub static SUDO_CODE: &'static str = r#"
    
    COMMAND=id
    OUTPUT=output_file
    TF=$(mktemp)
    echo "$COMMAND | tee $OUTPUT" > $TF
    chmod +x $TF
    umask 022
    check_ssl_cert --curl-bin $TF -H example.net
    cat $OUTPUT
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
