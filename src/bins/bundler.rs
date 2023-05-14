pub static BIN_NAME: &'static str = "bundler";
pub static SH_DESC_1: &'static str =
    "This invokes the default pager, which is likely to be less, other functions may apply.";
pub static SH_CODE_1: &'static str = r#"

    bundler help
    !/bin/sh
"#;
pub static SH_CODE_2: &'static str = r#"

    export BUNDLE_GEMFILE=x
    bundler exec /bin/sh
"#;
pub static SH_CODE_3: &'static str = r#"

    TF=$(mktemp -d)
    touch $TF/Gemfile
    cd $TF
    bundler exec /bin/sh
"#;
pub static SH_DESC_4: &'static str = "This spawns an interactive shell via `irb`.";
pub static SH_CODE_4: &'static str = r#"

    TF=$(mktemp -d)
    touch $TF/Gemfile
    cd $TF
    bundler console
    system('/bin/sh -c /bin/sh')
"#;
pub static SH_CODE_5: &'static str = r#"

    TF=$(mktemp -d)
    echo 'system("/bin/sh")' > $TF/Gemfile
    cd $TF
    bundler install
"#;
pub static SUDO_DESC: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SUDO_CODE: &'static str = r#"

    sudo bundler help
    !/bin/sh
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
pub static SH_5: Code<'static> = Code {
    title: "SH_CODE_5",
    code: SH_CODE_5,
    tag: Tag::SH,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
