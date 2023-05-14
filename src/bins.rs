use crate::code::Code;

pub fn bin_list() -> Vec<&'static str> {
    vec![
        "7z",
        "ab",
        "agetty",
        "alpine",
        "ansible-playbook",
        "aoss",
        "apt-get",
        "apt",
        "ar",
        "aria2c",
        "arj",
        "arp",
        "as",
        "ascii-xfr",
        "ascii85",
        "ash",
        "aspell",
        "at",
        "atobm",
        "awk",
        "aws",
        "base32",
        "base58",
        "base64",
        "basenc",
        "basez",
        "bash",
        "batcat",
        "bc",
        "bconsole",
        "bpftrace",
        "bridge",
        "bundle",
        "bundler",
        "busctl",
        "busybox",
        "byebug",
        "bzip2",
        "c89",
        "c99",
        "cabal",
        "cancel",
        "capsh",
        "cat",
        "cdist",
        "certbot",
        "check_by_ssh",
        "check_cups",
        "check_log",
        "check_memory",
        "check_raid",
        "check_ssl_cert",
        "check_statusfile",
        "chmod",
        "choom",
        "chown",
        "chroot",
        "cmp",
        "cobc",
        "column",
        "comm",
        "composer",
        "cowsay",
        "cowthink",
        "cp",
        "cpan",
        "cpio",
        "cpulimit",
        "crash",
        "crontab",
        "csh",
        "csplit",
        "csvtool",
        "cupsfilter",
        "curl",
        "cut",
        "dash",
        "date",
        "dd",
        "debugfs",
        "dialog",
        "diff",
        "dig",
        "distcc",
        "dmesg",
        "dmidecode",
        "dmsetup",
        "dnf",
        "docker",
        "dosbox",
        "dotnet",
        "dpkg",
        "dstat",
        "dvips",
        "easy_install",
        "eb",
        "ed",
        "efax",
        "emacs",
        "env",
        "eqn",
        "espeak",
        "ex",
        "exiftool",
        "expand",
        "expect",
        "facter",
        "file",
        "find",
        "finger",
        "fish",
        "flock",
        "fmt",
        "fold",
        "fping",
        "ftp",
        "gawk",
        "gcc",
        "gcloud",
        "gcore",
        "gdb",
        "gem",
        "genie",
        "genisoimage",
        "ghc",
        "ghci",
        "gimp",
        "ginsh",
        "git",
        "grc",
        "grep",
        "gtester",
        "gzip",
        "hd",
        "head",
        "hexdump",
        "highlight",
        "hping3",
        "iconv",
        "iftop",
        "install",
        "ionice",
        "ip",
        "irb",
        "ispell",
        "jjs",
        "joe",
        "join",
        "journalctl",
        "jq",
        "jrunscript",
        "jtag",
        "knife",
        "ksh",
        "ksshell",
        "ksu",
        "kubectl",
        "latex",
        "latexmk",
        "ld.so",
        "ldconfig",
        "less",
        "lftp",
        "ln",
        "loginctl",
        "logsave",
        "look",
        "lp",
        "ltrace",
        "lua",
        "lualatex",
        "luatex",
        "lwp-download",
        "lwp-request",
        "mail",
        "make",
        "man",
        "mawk",
        "more",
        "mosquitto",
        "mount",
        "msfconsole",
        "msgattrib",
        "msgcat",
        "msgconv",
        "msgfilter",
        "msgmerge",
        "msguniq",
        "mtr",
        "multitime",
        "mv",
        "mysql",
        "nano",
        "nasm",
        "nawk",
        "nc",
        "neofetch",
        "nft",
        "nice",
        "nl",
        "nm",
        "nmap",
        "node",
        "nohup",
        "npm",
        "nroff",
        "nsenter",
        "octave",
        "od",
        "openssl",
        "openvpn",
        "openvt",
        "opkg",
        "paste",
        "pax",
        "pdb",
        "pdflatex",
        "pdftex",
        "perf",
        "perl",
        "perlbug",
        "pexec",
        "pg",
        "php",
        "pic",
        "pico",
        "pidstat",
        "pip",
        "pkexec",
        "pkg",
        "posh",
        "pr",
        "pry",
        "psftp",
        "psql",
        "ptx",
        "puppet",
        "python",
        "rake",
        "readelf",
        "red",
        "redcarpet",
        "redis",
        "restic",
        "rev",
        "rlogin",
        "rlwrap",
        "rpm",
        "rpmdb",
        "rpmquery",
        "rpmverify",
        "rsync",
        "rtorrent",
        "ruby",
        "run-mailcap",
        "run-parts",
        "rview",
        "rvim",
        "sash",
        "scanmem",
        "scp",
        "screen",
        "script",
        "scrot",
        "sed",
        "service",
        "setarch",
        "setfacl",
        "setlock",
        "sftp",
        "sg",
        "shuf",
        "slsh",
        "smbclient",
        "snap",
        "socat",
        "socket",
        "soelim",
        "softlimit",
        "sort",
        "split",
        "sqlite3",
        "sqlmap",
        "ss",
        "ssh_keygen",
        "ssh_keyscan",
        "ssh",
        "sshpass",
        "start_stop_daemon",
        "stdbuf",
        "strace",
        "strings",
        "su",
        "sysctl",
        "systemctl",
        "systemd_resolve",
        "tac",
        "tail",
        "tar",
        "task",
        "taskset",
        "tasksh",
        "tbl",
        "tclsh",
        "tcpdump",
        "tdbtool",
        "tee",
        "telnet",
        "tex",
        "tftp",
        "tic",
        "time",
        "timedatectl",
        "timeout",
        "tmate",
        "tmux",
        "top",
        "torify",
        "torsocks",
        "troff",
        "tshark",
        "ul",
        "unexpand",
        "uniq",
        "unshare",
        "unzip",
        "update_alternatives",
        "uudecode",
        "uuencode",
        "valgrind",
        "vi",
        "view",
        "vigr",
        "vim",
        "vimdiff",
        "vipw",
        "virsh",
        "volatility",
        "w3m",
        "wall",
        "watch",
        "wc",
        "wget",
        "whiptail",
        "whois",
        "wireshark",
        "wish",
        "xargs",
        "xdotool",
        "xelatex",
        "xetex",
        "xmodmap",
        "xmore",
        "xpad",
        "xxd",
        "xz",
        "yarn",
        "yash",
        "yelp",
        "yum",
        "zathura",
        "zip",
        "zsh",
        "zsoelim",
        "zypper",
    ]
}

pub fn get_code<'a>(bin_name: &str) -> Vec<Code<'static>> {
    match bin_name {
        "7z" => vec![sevenz::FR, sevenz::SUDO],
        "ab" => vec![ab::FU, ab::FD, ab::SUID, ab::SUDO],
        "agetty" => vec![agetty::SUID],
        "alpine" => vec![alpine::FR, alpine::SUID, alpine::SUDO],
        "ansible-playbook" => vec![ansible_playbook::SH, ansible_playbook::SUDO],
        "aoss" => vec![aoss::SH, aoss::SUDO],
        "apt-get" => vec![
            apt_get::SH,
            apt_get::SUDO_1,
            apt_get::SUDO_2,
            apt_get::SUDO_3,
        ],
        "apt" => vec![apt::SH, apt::SUDO_1, apt::SUDO_2, apt::SUDO_3],
        "ar" => vec![ar::FR, ar::SUID, ar::SUDO],
        "aria2c" => vec![
            aria2c::CMD_1,
            aria2c::CMD_2,
            aria2c::FD,
            aria2c::SUDO,
            aria2c::LSUID,
        ],
        "arj" => vec![arj::FW, arj::FR, arj::SUID, arj::SUDO],
        "arp" => vec![arp::FR, arp::SUID, arp::SUDO],
        "as" => vec![as_::FR, as_::SUID, as_::SUDO],
        "ascii-xfr" => vec![ascii_xfr::FR, ascii_xfr::SUID, ascii_xfr::SUDO],
        "ascii85" => vec![ascii85::FR, ascii85::SUDO],
        "ash" => vec![ash::SH, ash::FW, ash::SUID, ash::SUDO],
        "aspell" => vec![aspell::FR, aspell::SUID, aspell::SUDO],
        "at" => vec![at::SH, at::CMD, at::SUDO],
        "atobm" => vec![atobm::FR, atobm::SUID, atobm::SUDO],
        "awk" => vec![
            awk::SH,
            awk::NIRS,
            awk::NIBS,
            awk::FW,
            awk::FR,
            awk::SUID,
            awk::SUDO,
            awk::LSUID,
        ],
        "aws" => vec![aws::SH, aws::SUDO],
        "base32" => vec![base32::FR, base32::SUID, base32::SUDO],
        "base58" => vec![base58::FR, base58::SUDO],
        "base64" => vec![base64::FR, base64::SUID, base64::SUDO],
        "basenc" => vec![basenc::FR, basenc::SUID, basenc::SUDO],
        "basez" => vec![basez::FR, basez::SUID, basez::SUDO],
        "bash" => vec![
            bash::SH,
            bash::RS,
            bash::FU_1,
            bash::FU_2,
            bash::FD_1,
            bash::FD_2,
            bash::FW_1,
            bash::FW_2,
            bash::FR_1,
            bash::FR_2,
            bash::LL,
            bash::SUID,
            bash::SUDO,
        ],
        "batcat" => vec![batcat::SH, batcat::SUDO, batcat::LSUID],
        "bc" => vec![bc::FR, bc::SUID, bc::SUDO],
        "bconsole" => vec![bconsole::SH, bconsole::FR, bconsole::SUDO],
        "bpftrace" => vec![bpftrace::SUDO_1, bpftrace::SUDO_2, bpftrace::SUDO_3],
        "bridge" => vec![bridge::FR, bridge::SUID, bridge::SUDO],
        "bundle" => vec![
            bundle::SH_1,
            bundle::SH_2,
            bundle::SH_3,
            bundle::SH_4,
            bundle::SH_5,
            bundle::SUDO,
        ],
        "bundler" => vec![
            bundler::SH_1,
            bundler::SH_2,
            bundler::SH_3,
            bundler::SH_4,
            bundler::SH_5,
            bundler::SUDO,
        ],
        "busctl" => vec![busctl::SH, busctl::SUDO],
        "busybox" => vec![
            busybox::SH,
            busybox::FU,
            busybox::FW,
            busybox::FR,
            busybox::SUID,
            busybox::SUDO,
        ],
        "byebug" => vec![byebug::SH, byebug::SUDO, byebug::LSUID],
        "bzip2" => vec![bzip2::FR, bzip2::SUID, bzip2::SUDO],
        "c89" => vec![c89::SH, c89::FW, c89::FR, c89::SUDO],
        "c99" => vec![c99::SH, c99::FW, c99::FR, c99::SUDO],
        "cabal" => vec![cabal::SH, cabal::SUID, cabal::SUDO],
        "cancel" => vec![cancel::FU],
        "capsh" => vec![capsh::SH, capsh::SUID, capsh::SUDO],
        "cat" => vec![cat::FR, cat::SUID, cat::SUDO],
        "cdist" => vec![cdist::SH, cdist::SUDO],
        "certbot" => vec![certbot::SH, certbot::SUDO],
        "check_by_ssh" => vec![check_by_ssh::SH, check_by_ssh::SUDO],
        "check_cups" => vec![check_cups::FR, check_cups::SUDO],
        "check_log" => vec![check_log::FW, check_log::FR, check_log::SUDO],
        "check_memory" => vec![check_memory::FR, check_memory::SUDO],
        "check_raid" => vec![check_raid::FR, check_raid::SUDO],
        "check_ssl_cert" => vec![check_ssl_cert::CMD, check_ssl_cert::SUDO],
        "check_statusfile" => vec![check_statusfile::FR, check_statusfile::SUDO],
        "chmod" => vec![chmod::SUID, chmod::SUDO],
        "choom" => vec![choom::SH, choom::SUID, choom::SUDO],
        "chown" => vec![chown::SUID, chown::SUDO],
        "chroot" => vec![chroot::SUID, chroot::SUDO],
        "cmp" => vec![cmp::FR, cmp::SUID, cmp::SUDO],
        "cobc" => vec![cobc::SH, cobc::SUDO],
        "column" => vec![column::FR, column::SUID, column::SUDO],
        "comm" => vec![comm::FR, comm::SUID, comm::SUDO],
        "composer" => vec![composer::SH, composer::SUDO, composer::LSUID],
        "cowsay" => vec![cowsay::SH, cowsay::SUDO],
        "cowthink" => vec![cowthink::SH, cowthink::SUDO],
        "cp" => vec![
            cp::FW,
            cp::FR,
            cp::SUID_1,
            cp::SUID_2,
            cp::SUID_3,
            cp::SUDO_1,
            cp::SUDO_2,
            cp::SUDO_3,
        ],
        "cpan" => vec![cpan::SH, cpan::RS, cpan::FU, cpan::FD, cpan::SUDO],
        "cpio" => vec![
            cpio::SH,
            cpio::FW,
            cpio::FR_1,
            cpio::FR_2,
            cpio::SUID_1,
            cpio::SUID_2,
            cpio::SUDO_1,
            cpio::SUDO_2,
            cpio::SUDO_3,
        ],
        "cpulimit" => vec![cpulimit::SH, cpulimit::SUID, cpulimit::SUDO],
        "crash" => vec![crash::SH, crash::CMD, crash::SUDO],
        "crontab" => vec![crontab::CMD, crontab::SUDO],
        "csh" => vec![csh::SH, csh::FW, csh::SUID, csh::SUDO],
        "csplit" => vec![csplit::FW, csplit::FR, csplit::SUID, csplit::SUDO],
        "csvtool" => vec![
            csvtool::SH,
            csvtool::FW,
            csvtool::FR,
            csvtool::SUID,
            csvtool::SUDO,
        ],
        "cupsfilter" => vec![cupsfilter::FR, cupsfilter::SUID, cupsfilter::SUDO],
        "curl" => vec![
            curl::FU,
            curl::FD,
            curl::FW,
            curl::FR,
            curl::SUID,
            curl::SUDO,
        ],
        "cut" => vec![cut::FR, cut::SUID, cut::SUDO],
        "dash" => vec![dash::SH, dash::FW, dash::SUID, dash::SUDO],
        "date" => vec![date::FR, date::SUID, date::SUDO],
        "dd" => vec![dd::FW, dd::FR, dd::SUID, dd::SUDO],
        "debugfs" => vec![debugfs::SH, debugfs::SUID, debugfs::SUDO],
        "dialog" => vec![dialog::FR, dialog::SUID, dialog::SUDO],
        "diff" => vec![diff::FR_1, diff::FR_2, diff::SUID, diff::SUDO],
        "dig" => vec![dig::FR, dig::SUID, dig::SUDO],
        "distcc" => vec![distcc::SH, distcc::SUID, distcc::SUDO],
        "dmesg" => vec![dmesg::SH, dmesg::FR, dmesg::SUDO],
        "dmidecode" => vec![dmidecode::SUDO_1, dmidecode::SUDO_2],
        "dmsetup" => vec![dmsetup::SUID, dmsetup::SUDO],
        "dnf" => vec![dnf::SUDO_1, dnf::SUDO_2],
        "docker" => vec![
            docker::SH,
            docker::FW,
            docker::FR,
            docker::SUID,
            docker::SUDO,
        ],
        "dosbox" => vec![
            dosbox::FW,
            dosbox::FR_1,
            dosbox::FR_2,
            dosbox::SUID,
            dosbox::SUDO,
        ],
        "dotnet" => vec![dotnet::SH, dotnet::FR, dotnet::SUDO],
        "dpkg" => vec![dpkg::SH, dpkg::SUDO_1, dpkg::SUDO_2, dpkg::SUDO_3],
        "dstat" => vec![dstat::SH, dstat::SUDO],
        "dvips" => vec![dvips::SH, dvips::SUDO, dvips::LSUID],
        "easy_install" => vec![
            easy_install::SH,
            easy_install::RS,
            easy_install::FU_1,
            easy_install::FU_2,
            easy_install::FD,
            easy_install::FW,
            easy_install::FR,
            easy_install::LL,
            easy_install::SUDO,
        ],
        "eb" => vec![eb::SH, eb::SUDO],
        "ed" => vec![ed::SH, ed::FW, ed::FR, ed::SUID, ed::SUDO, ed::LSUID],
        "efax" => vec![efax::SUID, efax::SUDO],
        "emacs" => vec![emacs::SH, emacs::FW, emacs::FR, emacs::SUID, emacs::SUDO],
        "env" => vec![env::SH, env::SUID, env::SUDO],
        "eqn" => vec![eqn::FR, eqn::SUID, eqn::SUDO],
        "espeak" => vec![espeak::FR, espeak::SUID, espeak::SUDO],
        "ex" => vec![ex::SH, ex::FW, ex::FR, ex::SUDO],
        "exiftool" => vec![exiftool::FW, exiftool::FR, exiftool::SUDO],
        "expand" => vec![expand::FR, expand::SUID, expand::SUDO],
        "expect" => vec![expect::SH, expect::FR, expect::SUID, expect::SUDO],
        "facter" => vec![facter::SH, facter::SUDO],
        "file" => vec![file::FR_1, file::FR_2, file::SUID, file::SUDO],
        "find" => vec![find::SH, find::SUID, find::SUDO],
        "finger" => vec![finger::FU, finger::FD],
        "fish" => vec![fish::SH, fish::SUID, fish::SUDO],
        "flock" => vec![flock::SH, flock::SUID, flock::SUDO],
        "fmt" => vec![fmt::FR_1, fmt::FR_2, fmt::SUID, fmt::SUDO],
        "fold" => vec![fold::FR, fold::SUID, fold::SUDO],
        "fping" => vec![fping::FR, fping::SUDO],
        "ftp" => vec![ftp::SH, ftp::FU, ftp::FD, ftp::SUDO],
        "gawk" => vec![
            gawk::SH,
            gawk::NIRS,
            gawk::NIBS,
            gawk::FW,
            gawk::FR,
            gawk::SUID,
            gawk::SUDO,
            gawk::LSUID,
        ],
        "gcc" => vec![gcc::SH, gcc::FW, gcc::FR_1, gcc::FR_2, gcc::SUDO],
        "gcloud" => vec![gcloud::SH, gcloud::SUDO],
        "gcore" => vec![gcore::FR, gcore::SUID, gcore::SUDO],
        "gdb" => vec![
            gdb::SH,
            gdb::RS,
            gdb::FU_1,
            gdb::FU_2,
            gdb::FD,
            gdb::FW,
            gdb::FR,
            gdb::LL,
            gdb::SUID,
            gdb::SUDO,
            gdb::CB,
        ],
        "gem" => vec![gem::SH_1, gem::SH_2, gem::SH_3, gem::SH_4, gem::SUDO],
        "genie" => vec![genie::SH, genie::SUID, genie::SUDO],
        "genisoimage" => vec![genisoimage::FR, genisoimage::SUID, genisoimage::SUDO],
        "ghc" => vec![ghc::SH, ghc::SUDO],
        "ghci" => vec![ghci::SH, ghci::SUDO],
        "gimp" => vec![
            gimp::SH,
            gimp::RS,
            gimp::FU_1,
            gimp::FU_2,
            gimp::FD,
            gimp::FW,
            gimp::FR,
            gimp::LL,
            gimp::SUID,
            gimp::SUDO,
        ],
        "ginsh" => vec![ginsh::SH, ginsh::SUDO, ginsh::LSUID],
        "git" => vec![
            git::SH_1,
            git::SH_2,
            git::SH_3,
            git::SH_4,
            git::SH_5,
            git::FR,
            git::SUDO_1,
            git::SUDO_2,
            git::SUDO_3,
            git::SUDO_4,
            git::SUDO_5,
            git::LSUID,
        ],
        "grc" => vec![grc::SH, grc::SUDO],
        "grep" => vec![grep::FR, grep::SUID, grep::SUDO],
        "gtester" => vec![gtester::SH, gtester::FW, gtester::SUID, gtester::SUDO],
        "gzip" => vec![gzip::FR_1, gzip::FR_2, gzip::SUID, gzip::SUDO],
        "hd" => vec![hd::FR, hd::SUID, hd::SUDO],
        "head" => vec![head::FR, head::SUID, head::SUDO],
        "hexdump" => vec![hexdump::FR, hexdump::SUID, hexdump::SUDO],
        "highlight" => vec![highlight::FR, highlight::SUID, highlight::SUDO],
        "hping3" => vec![
            hping3::SH,
            hping3::SUID,
            hping3::SUDO_1,
            hping3::SUDO_2,
            hping3::SUDO_3,
        ],
        "iconv" => vec![iconv::FW, iconv::FR, iconv::SUID, iconv::SUDO],
        "iftop" => vec![iftop::SH, iftop::SUDO, iftop::LSUID],
        "install" => vec![install::SUID, install::SUDO],
        "ionice" => vec![ionice::SH, ionice::SUID, ionice::SUDO],
        "ip" => vec![
            ip::FR,
            ip::SUID_1,
            ip::SUID_2,
            ip::SUDO_1,
            ip::SUDO_2,
            ip::SUDO_3,
        ],
        "irb" => vec![
            irb::SH,
            irb::RS,
            irb::FU,
            irb::FD,
            irb::FW,
            irb::FR,
            irb::LL,
            irb::SUDO,
        ],
        "ispell" => vec![ispell::SH, ispell::SUID, ispell::SUDO],
        "jjs" => vec![
            jjs::SH,
            jjs::RS,
            jjs::FD,
            jjs::FW,
            jjs::FR,
            jjs::SUID,
            jjs::SUDO,
        ],
        "joe" => vec![joe::SH, joe::SUDO, joe::LSUID],
        "join" => vec![join::FR, join::SUID, join::SUDO],
        "journalctl" => vec![journalctl::SH, journalctl::SUDO],
        "jq" => vec![jq::FR, jq::SUID, jq::SUDO],
        "jrunscript" => vec![
            jrunscript::SH,
            jrunscript::RS,
            jrunscript::FD,
            jrunscript::FW,
            jrunscript::FR,
            jrunscript::SUID,
            jrunscript::SUDO,
        ],
        "jtag" => vec![jtag::SH, jtag::SUDO],
        "knife" => vec![knife::SH, knife::SUDO],
        "ksh" => vec![
            ksh::SH,
            ksh::RS,
            ksh::FU_1,
            ksh::FU_2,
            ksh::FD_1,
            ksh::FD_2,
            ksh::FW,
            ksh::FR_1,
            ksh::FR_2,
            ksh::SUID,
            ksh::SUDO,
        ],
        "ksshell" => vec![ksshell::FR, ksshell::SUID, ksshell::SUDO],
        "ksu" => vec![ksu::SUDO],
        "kubectl" => vec![kubectl::FU, kubectl::SUID, kubectl::SUDO],
        "latex" => vec![
            latex::SH,
            latex::FR,
            latex::SUDO_1,
            latex::SUDO_2,
            latex::LSUID,
        ],
        "latexmk" => vec![
            latexmk::SH_1,
            latexmk::SH_2,
            latexmk::FR_1,
            latexmk::FR_2,
            latexmk::SUDO,
        ],
        "ld.so" => vec![ld_so::SH, ld_so::SUID, ld_so::SUDO],
        "ldconfig" => vec![ldconfig::SUDO, ldconfig::LSUID],
        "less" => vec![
            less::SH_1,
            less::SH_2,
            less::SH_3,
            less::FW_1,
            less::FW_2,
            less::FR_1,
            less::FR_2,
            less::SUID,
            less::SUDO,
        ],
        "lftp" => vec![lftp::SH, lftp::SUDO, lftp::LSUID],
        "ln" => vec![ln::SUDO],
        "loginctl" => vec![loginctl::SH, loginctl::SUDO],
        "logsave" => vec![logsave::SH, logsave::SUID, logsave::SUDO],
        "look" => vec![look::FR, look::SUID, look::SUDO],
        "lp" => vec![lp::FU],
        "ltrace" => vec![ltrace::SH, ltrace::FW, ltrace::FR, ltrace::SUDO],
        "lua" => vec![
            lua::SH,
            lua::NIRS,
            lua::NIBS,
            lua::FU,
            lua::FD,
            lua::FW,
            lua::FR,
            lua::SUID,
            lua::SUDO,
            lua::LSUID,
        ],
        "lualatex" => vec![lualatex::SH, lualatex::SUDO, lualatex::LSUID],
        "luatex" => vec![luatex::SH, luatex::SUDO, luatex::LSUID],
        "lwp-download" => vec![
            lwp_download::FD,
            lwp_download::FW,
            lwp_download::FR,
            lwp_download::SUDO,
        ],
        "lwp-request" => vec![lwp_request::FR, lwp_request::SUDO],
        "mail" => vec![mail::SH_1, mail::SH_2, mail::SUDO],
        "make" => vec![make::SH, make::FW, make::SUID, make::SUDO],
        "man" => vec![man::SH_1, man::SH_2, man::FR, man::SUDO],
        "mawk" => vec![
            mawk::SH,
            mawk::FW,
            mawk::FR,
            mawk::SUID,
            mawk::SUDO,
            mawk::LSUID,
        ],
        "more" => vec![more::SH, more::FR, more::SUID, more::SUDO],
        "mosquitto" => vec![mosquitto::FR, mosquitto::SUID, mosquitto::SUDO],
        "mount" => vec![mount::SUDO],
        "msfconsole" => vec![msfconsole::SH, msfconsole::SUDO],
        "msgattrib" => vec![msgattrib::FR, msgattrib::SUID, msgattrib::SUDO],
        "msgcat" => vec![msgcat::FR, msgcat::SUID, msgcat::SUDO],
        "msgconv" => vec![msgconv::FR, msgconv::SUID, msgconv::SUDO],
        "msgfilter" => vec![
            msgfilter::SH,
            msgfilter::FR,
            msgfilter::SUID,
            msgfilter::SUDO,
        ],
        "msgmerge" => vec![msgmerge::FR, msgmerge::SUID, msgmerge::SUDO],
        "msguniq" => vec![msguniq::FR, msguniq::SUID, msguniq::SUDO],
        "mtr" => vec![mtr::FR, mtr::SUDO],
        "multitime" => vec![multitime::SH, multitime::SUID, multitime::SUDO],
        "mv" => vec![mv::SUID, mv::SUDO],
        "mysql" => vec![mysql::SH, mysql::LL, mysql::SUDO, mysql::LSUID],
        "nano" => vec![
            nano::SH_1,
            nano::SH_2,
            nano::FW,
            nano::FR,
            nano::SUDO,
            nano::LSUID,
        ],
        "nasm" => vec![nasm::FR, nasm::SUID, nasm::SUDO],
        "nawk" => vec![
            nawk::SH,
            nawk::NIRS,
            nawk::NIBS,
            nawk::FW,
            nawk::FR,
            nawk::SUID,
            nawk::SUDO,
            nawk::LSUID,
        ],
        "nc" => vec![nc::RS, nc::BS, nc::FU, nc::FD, nc::SUDO, nc::LSUID],
        "neofetch" => vec![neofetch::SH, neofetch::FR, neofetch::SUDO],
        "nft" => vec![nft::FR, nft::SUID, nft::SUDO],
        "nice" => vec![nice::SH, nice::SUID, nice::SUDO],
        "nl" => vec![nl::FR, nl::SUID, nl::SUDO],
        "nm" => vec![nm::FR, nm::SUID, nm::SUDO],
        "nmap" => vec![
            nmap::SH_1,
            nmap::SH_2,
            nmap::NIRS,
            nmap::NIBS,
            nmap::FU_1,
            nmap::FU_2,
            nmap::FD_1,
            nmap::FD_2,
            nmap::FW_1,
            nmap::FW_2,
            nmap::FR,
            nmap::SUID,
            nmap::SUDO_1,
            nmap::SUDO_2,
            nmap::LSUID,
        ],
        "node" => vec![
            node::SH,
            node::RS,
            node::BS,
            node::FU,
            node::FD,
            node::FW,
            node::FR,
            node::SUID,
            node::SUDO,
            node::CB,
        ],
        "nohup" => vec![nohup::SH, nohup::CMD, nohup::SUID, nohup::SUDO],
        "npm" => vec![npm::SH_1, npm::SH_2, npm::SUDO],
        "nroff" => vec![nroff::SH, nroff::FR, nroff::SUDO],
        "nsenter" => vec![nsenter::SH, nsenter::SUDO],
        "octave" => vec![
            octave::SH,
            octave::FW,
            octave::FR,
            octave::SUDO,
            octave::LSUID,
        ],
        "od" => vec![od::FR, od::SUID, od::SUDO],
        "openssl" => vec![
            openssl::RS_1,
            openssl::RS_2,
            openssl::FU_1,
            openssl::FU_2,
            openssl::FD_1,
            openssl::FD_2,
            openssl::FW_1,
            openssl::FW_2,
            openssl::FR,
            openssl::LL,
            openssl::SUID_1,
            openssl::SUID_2,
            openssl::SUID_3,
            openssl::SUDO_1,
            openssl::SUDO_2,
        ],
        "openvpn" => vec![
            openvpn::SH,
            openvpn::FR,
            openvpn::SUID_1,
            openvpn::SUID_2,
            openvpn::SUDO_1,
            openvpn::SUDO_2,
        ],
        "openvt" => vec![openvt::SUDO],
        "opkg" => vec![opkg::SUDO_1, opkg::SUDO_2],
        "paste" => vec![paste::FR, paste::SUID, paste::SUDO],
        "pax" => vec![pax::FR],
        "pdb" => vec![pdb::SH, pdb::SUDO],
        "pdflatex" => vec![
            pdflatex::SH,
            pdflatex::FR,
            pdflatex::SUDO_1,
            pdflatex::SUDO_2,
            pdflatex::LSUID,
        ],
        "pdftex" => vec![pdftex::SH, pdftex::SUDO, pdftex::LSUID],
        "perf" => vec![perf::SH, perf::SUID, perf::SUDO],
        "perl" => vec![
            perl::SH,
            perl::RS,
            perl::FR,
            perl::SUID,
            perl::SUDO,
            perl::CB,
        ],
        "perlbug" => vec![perlbug::SH, perlbug::SUDO],
        "pexec" => vec![pexec::SH, pexec::SUID, pexec::SUDO],
        "pg" => vec![pg::SH, pg::FR, pg::SUID, pg::SUDO],
        "php" => vec![
            php::SH_1,
            php::SH_2,
            php::SH_3,
            php::SH_4,
            php::SH_5,
            php::CMD,
            php::RS,
            php::FU,
            php::FD,
            php::FW,
            php::FR,
            php::SUID,
            php::SUDO,
            php::CB,
        ],
        "pic" => vec![pic::SH, pic::FR, pic::SUDO, pic::LSUID],
        "pico" => vec![
            pico::SH_1,
            pico::SH_2,
            pico::FW,
            pico::FR,
            pico::SUDO,
            pico::LSUID,
        ],
        "pidstat" => vec![pidstat::CMD, pidstat::SUID, pidstat::SUDO],
        "pip" => vec![
            pip::SH,
            pip::RS,
            pip::FU_1,
            pip::FU_2,
            pip::FD,
            pip::FW,
            pip::FR,
            pip::LL,
            pip::SUDO,
        ],
        "pkexec" => vec![pkexec::SUDO],
        "pkg" => vec![pkg::SUDO_1, pkg::SUDO_2],
        "posh" => vec![posh::SH, posh::SUDO, posh::LSUID],
        "pr" => vec![pr::FR, pr::SUID, pr::SUDO],
        "pry" => vec![pry::SH, pry::SUDO, pry::LSUID],
        "psftp" => vec![psftp::SH, psftp::SUDO, psftp::LSUID],
        "psql" => vec![psql::SH, psql::SUDO],
        "ptx" => vec![ptx::FR, ptx::SUID, ptx::SUDO],
        "puppet" => vec![puppet::SH, puppet::FW, puppet::FR, puppet::SUDO],
        "python" => vec![
            python::SH,
            python::RS,
            python::FU_1,
            python::FU_2,
            python::FD,
            python::FW,
            python::FR,
            python::LL,
            python::SUID,
            python::SUDO,
            python::CB,
        ],
        "rake" => vec![rake::SH, rake::FR, rake::SUDO, rake::LSUID],
        "readelf" => vec![readelf::FR, readelf::SUID, readelf::SUDO],
        "red" => vec![red::FW, red::FR, red::SUDO],
        "redcarpet" => vec![redcarpet::FR, redcarpet::SUDO],
        "redis" => vec![redis::FW],
        "restic" => vec![restic::FU, restic::SUID, restic::SUDO],
        "rev" => vec![rev::FR, rev::SUID, rev::SUDO],
        "rlogin" => vec![rlogin::FU],
        "rlwrap" => vec![rlwrap::SH, rlwrap::FW, rlwrap::SUID, rlwrap::SUDO],
        "rpm" => vec![rpm::SH_1, rpm::SH_2, rpm::SUDO_1, rpm::SUDO_2, rpm::LSUID],
        "rpmdb" => vec![rpmdb::SH, rpmdb::SUDO, rpmdb::LSUID],
        "rpmquery" => vec![rpmquery::SH, rpmquery::SUDO, rpmquery::LSUID],
        "rpmverify" => vec![rpmverify::SH, rpmverify::SUDO, rpmverify::LSUID],
        "rsync" => vec![rsync::SH, rsync::SUID, rsync::SUDO],
        "rtorrent" => vec![rtorrent::SH, rtorrent::SUID],
        "ruby" => vec![
            ruby::SH,
            ruby::RS,
            ruby::FU,
            ruby::FD,
            ruby::FW,
            ruby::FR,
            ruby::LL,
            ruby::SUDO,
            ruby::CB,
        ],
        "run-mailcap" => vec![
            run_mailcap::SH,
            run_mailcap::FW,
            run_mailcap::FR,
            run_mailcap::SUDO,
        ],
        "run-parts" => vec![run_parts::SH, run_parts::SUID, run_parts::SUDO],
        "rview" => vec![
            rview::SH_1,
            rview::SH_2,
            rview::RS,
            rview::NIRS,
            rview::NIBS,
            rview::FU_1,
            rview::FU_2,
            rview::FU_3,
            rview::FW,
            rview::FR,
            rview::LL,
            rview::SUID,
            rview::SUDO_1,
            rview::SUDO_2,
            rview::CB,
            rview::LSUID,
        ],
        "rvim" => vec![
            rvim::SH_1,
            rvim::SH_2,
            rvim::RS,
            rvim::NIRS,
            rvim::NIBS,
            rvim::FU_1,
            rvim::FU_2,
            rvim::FU_3,
            rvim::FD_1,
            rvim::FD_2,
            rvim::FW,
            rvim::FR,
            rvim::LL,
            rvim::SUID,
            rvim::SUDO_1,
            rvim::SUDO_2,
            rvim::CB,
            rvim::LSUID,
        ],
        "sash" => vec![sash::SH, sash::SUID, sash::SUDO],
        "scanmem" => vec![scanmem::SH, scanmem::SUID, scanmem::SUDO],
        "scp" => vec![scp::SH, scp::FU, scp::FD, scp::SUDO, scp::LSUID],
        "screen" => vec![screen::SH, screen::FW_1, screen::FW_2, screen::SUDO],
        "script" => vec![script::SH, script::FW, script::SUDO],
        "scrot" => vec![scrot::SH, scrot::SUDO, scrot::LSUID],
        "sed" => vec![
            sed::SH_1,
            sed::SH_2,
            sed::CMD,
            sed::FW,
            sed::FR,
            sed::SUID,
            sed::SUDO,
        ],
        "service" => vec![service::SH, service::SUDO],
        "setarch" => vec![setarch::SH, setarch::SUID, setarch::SUDO],
        "setfacl" => vec![setfacl::SUID, setfacl::SUDO],
        "setlock" => vec![setlock::SH, setlock::SUID, setlock::SUDO],
        "sftp" => vec![sftp::SH, sftp::FU, sftp::FD, sftp::SUDO],
        "sg" => vec![sg::SH, sg::SUDO],
        "shuf" => vec![shuf::FW, shuf::FR, shuf::SUID, shuf::SUDO],
        "slsh" => vec![slsh::SH, slsh::SUDO, slsh::LSUID],
        "smbclient" => vec![smbclient::SH, smbclient::FU, smbclient::FD, smbclient::SUDO],
        "snap" => vec![snap::SUDO],
        "socat" => vec![
            socat::SH,
            socat::RS,
            socat::BS,
            socat::FU,
            socat::FD,
            socat::FW,
            socat::FR,
            socat::SUDO,
            socat::LSUID,
        ],
        "socket" => vec![socket::RS, socket::BS],
        "soelim" => vec![soelim::FR, soelim::SUID, soelim::SUDO],
        "softlimit" => vec![softlimit::SH, softlimit::SUID, softlimit::SUDO],
        "sort" => vec![sort::FR, sort::SUID, sort::SUDO],
        "split" => vec![
            split::SH,
            split::CMD_1,
            split::CMD_2,
            split::FW_1,
            split::FW_2,
            split::FR,
            split::SUDO,
        ],
        "sqlite3" => vec![
            sqlite3::SH,
            sqlite3::FW,
            sqlite3::FR,
            sqlite3::SUID,
            sqlite3::SUDO,
            sqlite3::LSUID,
        ],
        "sqlmap" => vec![sqlmap::SH, sqlmap::SUDO],
        "ss" => vec![ss::FR, ss::SUID, ss::SUDO],
        "ssh_keygen" => vec![ssh_keygen::LL, ssh_keygen::SUID, ssh_keygen::SUDO],
        "ssh_keyscan" => vec![ssh_keyscan::FR, ssh_keyscan::SUID, ssh_keyscan::SUDO],
        "ssh" => vec![
            ssh::SH_1,
            ssh::SH_2,
            ssh::SH_3,
            ssh::FU,
            ssh::FD,
            ssh::FR,
            ssh::SUDO,
        ],
        "sshpass" => vec![sshpass::SH, sshpass::SUID, sshpass::SUDO],
        "start_stop_daemon" => vec![
            start_stop_daemon::SH,
            start_stop_daemon::SUID,
            start_stop_daemon::SUDO,
        ],
        "stdbuf" => vec![stdbuf::SH, stdbuf::SUID, stdbuf::SUDO],
        "strace" => vec![strace::SH, strace::FW, strace::SUID, strace::SUDO],
        "strings" => vec![strings::FR, strings::SUID, strings::SUDO],
        "su" => vec![su::SUDO],
        "sysctl" => vec![sysctl::CMD, sysctl::FR, sysctl::SUID, sysctl::SUDO],
        "systemctl" => vec![
            systemctl::SUID,
            systemctl::SUDO_1,
            systemctl::SUDO_2,
            systemctl::SUDO_3,
        ],
        "systemd_resolve" => vec![systemd_resolve::SUDO],
        "tac" => vec![tac::FR, tac::SUID, tac::SUDO],
        "tail" => vec![tail::FR, tail::SUID, tail::SUDO],
        "tar" => vec![
            tar::SH_1,
            tar::SH_2,
            tar::SH_3,
            tar::FU,
            tar::FD,
            tar::FW,
            tar::FR,
            tar::SUDO,
            tar::LSUID,
        ],
        "task" => vec![task::SH, task::SUDO],
        "taskset" => vec![taskset::SH, taskset::SUID, taskset::SUDO],
        "tasksh" => vec![tasksh::SH, tasksh::SUDO, tasksh::LSUID],
        "tbl" => vec![tbl::FR, tbl::SUID, tbl::SUDO],
        "tclsh" => vec![tclsh::SH, tclsh::NIRS, tclsh::SUID, tclsh::SUDO],
        "tcpdump" => vec![tcpdump::CMD, tcpdump::SUDO],
        "tdbtool" => vec![tdbtool::SH, tdbtool::SUDO, tdbtool::LSUID],
        "tee" => vec![tee::FW, tee::SUID, tee::SUDO],
        "telnet" => vec![telnet::SH, telnet::RS, telnet::SUDO, telnet::LSUID],
        "tex" => vec![tex::SH, tex::SUDO, tex::LSUID],
        "tftp" => vec![tftp::FU, tftp::FD, tftp::SUID, tftp::SUDO],
        "tic" => vec![tic::FR, tic::SUID, tic::SUDO],
        "time" => vec![time::SH, time::SUID, time::SUDO],
        "timedatectl" => vec![timedatectl::SH, timedatectl::SUDO],
        "timeout" => vec![timeout::SH, timeout::SUID, timeout::SUDO],
        "tmate" => vec![tmate::SH, tmate::SUDO, tmate::LSUID],
        "tmux" => vec![tmux::SH, tmux::FR, tmux::SUDO],
        "top" => vec![top::SH, top::SUDO],
        "torify" => vec![torify::SH, torify::SUDO],
        "torsocks" => vec![torsocks::SH, torsocks::SUDO],
        "troff" => vec![troff::FR, troff::SUID, troff::SUDO],
        "tshark" => vec![tshark::SH],
        "ul" => vec![ul::FR, ul::SUID, ul::SUDO],
        "unexpand" => vec![unexpand::FR, unexpand::SUID, unexpand::SUDO],
        "uniq" => vec![uniq::FR, uniq::SUID, uniq::SUDO],
        "unshare" => vec![unshare::SH, unshare::SUID, unshare::SUDO],
        "unzip" => vec![unzip::SUID, unzip::SUDO],
        "update_alternatives" => vec![update_alternatives::SUID, update_alternatives::SUDO],
        "uudecode" => vec![uudecode::FR, uudecode::SUID, uudecode::SUDO],
        "uuencode" => vec![uuencode::FR, uuencode::SUID, uuencode::SUDO],
        "valgrind" => vec![valgrind::SH, valgrind::SUDO],
        "vi" => vec![vi::SH_1, vi::SH_2, vi::FW, vi::FR, vi::SUDO],
        "view" => vec![
            view::SH_1,
            view::SH_2,
            view::SH_3,
            view::SH_4,
            view::RS,
            view::NIRS,
            view::NIBS,
            view::FU_1,
            view::FU_2,
            view::FU_3,
            view::FD_1,
            view::FD_2,
            view::FW,
            view::FR,
            view::LL,
            view::SUID,
            view::SUDO_1,
            view::SUDO_2,
            view::SUDO_3,
            view::CB,
            view::LSUID,
        ],
        "vigr" => vec![vigr::SUID, vigr::SUDO],
        "vim" => vec![
            vim::SH_1,
            vim::SH_2,
            vim::SH_3,
            vim::SH_4,
            vim::RS,
            vim::NIRS,
            vim::NIBS,
            vim::FU_1,
            vim::FU_2,
            vim::FU_3,
            vim::FD_1,
            vim::FD_2,
            vim::FW,
            vim::FR,
            vim::LL,
            vim::SUID,
            vim::SUDO_1,
            vim::SUDO_2,
            vim::SUDO_3,
            vim::CB,
            vim::LSUID,
        ],
        "vimdiff" => vec![
            vimdiff::SH_1,
            vimdiff::SH_2,
            vimdiff::SH_3,
            vimdiff::SH_4,
            vimdiff::RS,
            vimdiff::NIRS,
            vimdiff::NIBS,
            vimdiff::FU_1,
            vimdiff::FU_2,
            vimdiff::FU_3,
            vimdiff::FD_1,
            vimdiff::FD_2,
            vimdiff::FW,
            vimdiff::FR,
            vimdiff::LL,
            vimdiff::SUID,
            vimdiff::SUDO_1,
            vimdiff::SUDO_2,
            vimdiff::SUDO_3,
            vimdiff::CB,
            vimdiff::LSUID,
        ],
        "vipw" => vec![vipw::SUID, vipw::SUDO],
        "virsh" => vec![virsh::FW, virsh::FR, virsh::SUDO],
        "volatility" => vec![volatility::SH],
        "w3m" => vec![w3m::FR, w3m::SUID, w3m::SUDO],
        "wall" => vec![wall::SUDO],
        "watch" => vec![watch::SH, watch::SUID, watch::SUDO, watch::LSUID],
        "wc" => vec![wc::FR, wc::SUID, wc::SUDO],
        "wget" => vec![
            wget::SH,
            wget::FU,
            wget::FD,
            wget::FW,
            wget::FR,
            wget::SUID,
            wget::SUDO,
        ],
        "whiptail" => vec![whiptail::FR, whiptail::SUID, whiptail::SUDO],
        "whois" => vec![whois::FU_1, whois::FU_2, whois::FD_1, whois::FD_2],
        "wireshark" => vec![wireshark::SUDO],
        "wish" => vec![wish::SH, wish::NIRS, wish::SUDO],
        "xargs" => vec![
            xargs::SH_1,
            xargs::SH_2,
            xargs::SH_3,
            xargs::FR,
            xargs::SUID,
            xargs::SUDO,
        ],
        "xdotool" => vec![xdotool::SH, xdotool::SUID, xdotool::SUDO],
        "xelatex" => vec![
            xelatex::SH,
            xelatex::FR,
            xelatex::SUDO_1,
            xelatex::SUDO_2,
            xelatex::LSUID,
        ],
        "xetex" => vec![xetex::SH, xetex::SUDO, xetex::LSUID],
        "xmodmap" => vec![xmodmap::FR, xmodmap::SUID, xmodmap::SUDO],
        "xmore" => vec![xmore::FR, xmore::SUID, xmore::SUDO],
        "xpad" => vec![xpad::FR, xpad::SUDO],
        "xxd" => vec![xxd::FW, xxd::FR, xxd::SUID, xxd::SUDO],
        "xz" => vec![xz::FR, xz::SUID, xz::SUDO],
        "yarn" => vec![yarn::SH_1, yarn::SH_2, yarn::SUDO],
        "yash" => vec![yash::SH, yash::SUID, yash::SUDO],
        "yelp" => vec![yelp::FR],
        "yum" => vec![yum::FD, yum::SUDO_1, yum::SUDO_2],
        "zathura" => vec![zathura::SH, zathura::SUDO],
        "zip" => vec![zip::SH, zip::FR, zip::SUDO, zip::LSUID],
        "zsh" => vec![zsh::SH, zsh::FW, zsh::FR, zsh::SUID, zsh::SUDO],
        "zsoelim" => vec![zsoelim::FR, zsoelim::SUID, zsoelim::SUDO],
        "zypper" => vec![zypper::SH_1, zypper::SH_2, zypper::SUDO_1, zypper::SUDO_2],
        _ => unreachable!(),
    }
}

pub fn get_tags<'a>(bin_name: &str) -> Vec<&'a str> {
    let sh = "SHELL";
    let cmd = "CMD";
    let rs = "RS";
    let nirs = "NIRS";
    let bs = "BS";
    let nibs = "NIBS";
    let fu = "FU";
    let fd = "FD";
    let fw = "FW";
    let fr = "FR";
    let ll = "LL";
    let suid = "SUID";
    let sudo = "SUDO";
    let cb = "CB";
    let lsuid = "LSUID";
    match bin_name {
        "7z" => vec![fr, sudo],
        "ab" => vec![fu, fd, suid, sudo],
        "agetty" => vec![suid],
        "alpine" => vec![fr, suid, sudo],
        "ansible-playbook" => vec![sh, sudo],
        "aoss" => vec![sh, sudo],
        "apt-get" => vec![sh, sudo],
        "apt" => vec![sh, sudo],
        "ar" => vec![fr, suid, sudo],
        "aria2c" => vec![cmd, fd, sudo, lsuid],
        "arj" => vec![fw, fr, suid, sudo],
        "arp" => vec![fr, suid, sudo],
        "as" => vec![fr, suid, sudo],
        "ascii-xfr" => vec![fr, suid, sudo],
        "ascii85" => vec![fr, sudo],
        "ash" => vec![sh, fw, suid, sudo],
        "aspell" => vec![fr, suid, sudo],
        "at" => vec![sh, cmd, sudo],
        "atobm" => vec![fr, suid, sudo],
        "awk" => vec![sh, nirs, nibs, fw, fr, suid, sudo, lsuid],
        "aws" => vec![sh, sudo],
        "base32" => vec![fr, suid, sudo],
        "base58" => vec![fr, sudo],
        "base64" => vec![fr, suid, sudo],
        "basenc" => vec![fr, suid, sudo],
        "basez" => vec![fr, suid, sudo],
        "bash" => vec![sh, rs, fu, fd, fw, fr, ll, suid, sudo],
        "batcat" => vec![sh, sudo, lsuid],
        "bc" => vec![fr, suid, sudo],
        "bconsole" => vec![sh, fr, sudo],
        "bpftrace" => vec![sudo],
        "bridge" => vec![fr, suid, sudo],
        "bundle" => vec![sh, sudo],
        "bundler" => vec![sh, sudo],
        "busctl" => vec![sh, sudo],
        "busybox" => vec![sh, fu, fw, fr, suid, sudo],
        "byebug" => vec![sh, sudo, lsuid],
        "bzip2" => vec![fr, suid, sudo],
        "c89" => vec![sh, fw, fr, sudo],
        "c99" => vec![sh, fw, fr, sudo],
        "cabal" => vec![sh, suid, sudo],
        "cancel" => vec![fu],
        "capsh" => vec![sh, suid, sudo],
        "cat" => vec![fr, suid, sudo],
        "cdist" => vec![sh, sudo],
        "certbot" => vec![sh, sudo],
        "check_by_ssh" => vec![sh, sudo],
        "check_cups" => vec![fr, sudo],
        "check_log" => vec![fw, fr, sudo],
        "check_memory" => vec![fr, sudo],
        "check_raid" => vec![fr, sudo],
        "check_ssl_cert" => vec![cmd, sudo],
        "check_statusfile" => vec![fr, sudo],
        "chmod" => vec![suid, sudo],
        "choom" => vec![sh, suid, sudo],
        "chown" => vec![suid, sudo],
        "chroot" => vec![suid, sudo],
        "cmp" => vec![fr, suid, sudo],
        "cobc" => vec![sh, sudo],
        "column" => vec![fr, suid, sudo],
        "comm" => vec![fr, suid, sudo],
        "composer" => vec![sh, sudo, lsuid],
        "cowsay" => vec![sh, sudo],
        "cowthink" => vec![sh, sudo],
        "cp" => vec![fw, fr, suid, sudo],
        "cpan" => vec![sh, rs, fu, fd, sudo],
        "cpio" => vec![sh, fw, fr, suid, sudo],
        "cpulimit" => vec![sh, suid, sudo],
        "crash" => vec![sh, cmd, sudo],
        "crontab" => vec![cmd, sudo],
        "csh" => vec![sh, fw, suid, sudo],
        "csplit" => vec![fw, fr, suid, sudo],
        "csvtool" => vec![sh, fw, fr, suid, sudo],
        "cupsfilter" => vec![fr, suid, sudo],
        "curl" => vec![fu, fd, fw, fr, suid, sudo],
        "cut" => vec![fr, suid, sudo],
        "dash" => vec![sh, fw, suid, sudo],
        "date" => vec![fr, suid, sudo],
        "dd" => vec![fw, fr, suid, sudo],
        "debugfs" => vec![sh, suid, sudo],
        "dialog" => vec![fr, suid, sudo],
        "diff" => vec![fr, suid, sudo],
        "dig" => vec![fr, suid, sudo],
        "distcc" => vec![sh, suid, sudo],
        "dmesg" => vec![sh, fr, sudo],
        "dmidecode" => vec![sudo],
        "dmsetup" => vec![suid, sudo],
        "dnf" => vec![sudo],
        "docker" => vec![sh, fw, fr, suid, sudo],
        "dosbox" => vec![fw, fr, suid, sudo],
        "dotnet" => vec![sh, fr, sudo],
        "dpkg" => vec![sh, sudo],
        "dstat" => vec![sh, sudo],
        "dvips" => vec![sh, sudo, lsuid],
        "easy_install" => vec![sh, rs, fu, fd, fw, fr, ll, sudo],
        "eb" => vec![sh, sudo],
        "ed" => vec![sh, fw, fr, suid, sudo, lsuid],
        "efax" => vec![suid, sudo],
        "emacs" => vec![sh, fw, fr, suid, sudo],
        "env" => vec![sh, suid, sudo],
        "eqn" => vec![fr, suid, sudo],
        "espeak" => vec![fr, suid, sudo],
        "ex" => vec![sh, fw, fr, sudo],
        "exiftool" => vec![fw, fr, sudo],
        "expand" => vec![fr, suid, sudo],
        "expect" => vec![sh, fr, suid, sudo],
        "facter" => vec![sh, sudo],
        "file" => vec![fr, suid, sudo],
        "find" => vec![sh, suid, sudo],
        "finger" => vec![fu, fd],
        "fish" => vec![sh, suid, sudo],
        "flock" => vec![sh, suid, sudo],
        "fmt" => vec![fr, suid, sudo],
        "fold" => vec![fr, suid, sudo],
        "fping" => vec![fr, sudo],
        "ftp" => vec![sh, fu, fd, sudo],
        "gawk" => vec![sh, nirs, nibs, fw, fr, suid, sudo, lsuid],
        "gcc" => vec![sh, fw, fr, sudo],
        "gcloud" => vec![sh, sudo],
        "gcore" => vec![fr, suid, sudo],
        "gdb" => vec![sh, rs, fu, fd, fw, fr, ll, suid, sudo, cb],
        "gem" => vec![sh, sudo],
        "genie" => vec![sh, suid, sudo],
        "genisoimage" => vec![fr, suid, sudo],
        "ghc" => vec![sh, sudo],
        "ghci" => vec![sh, sudo],
        "gimp" => vec![sh, rs, fu, fd, fw, fr, ll, suid, sudo],
        "ginsh" => vec![sh, sudo, lsuid],
        "git" => vec![sh, fr, sudo, lsuid],
        "grc" => vec![sh, sudo],
        "grep" => vec![fr, suid, sudo],
        "gtester" => vec![sh, fw, suid, sudo],
        "gzip" => vec![fr, suid, sudo],
        "hd" => vec![fr, suid, sudo],
        "head" => vec![fr, suid, sudo],
        "hexdump" => vec![fr, suid, sudo],
        "highlight" => vec![fr, suid, sudo],
        "hping3" => vec![sh, suid, sudo],
        "iconv" => vec![fw, fr, suid, sudo],
        "iftop" => vec![sh, sudo, lsuid],
        "install" => vec![suid, sudo],
        "ionice" => vec![sh, suid, sudo],
        "ip" => vec![fr, suid, sudo],
        "irb" => vec![sh, rs, fu, fd, fw, fr, ll, sudo],
        "ispell" => vec![sh, suid, sudo],
        "jjs" => vec![sh, rs, fd, fw, fr, suid, sudo],
        "joe" => vec![sh, sudo, lsuid],
        "join" => vec![fr, suid, sudo],
        "journalctl" => vec![sh, sudo],
        "jq" => vec![fr, suid, sudo],
        "jrunscript" => vec![sh, rs, fd, fw, fr, suid, sudo],
        "jtag" => vec![sh, sudo],
        "knife" => vec![sh, sudo],
        "ksh" => vec![sh, rs, fu, fd, fw, fr, suid, sudo],
        "ksshell" => vec![fr, suid, sudo],
        "ksu" => vec![sudo],
        "kubectl" => vec![fu, suid, sudo],
        "latex" => vec![sh, fr, sudo, lsuid],
        "latexmk" => vec![sh, fr, sudo],
        "ld.so" => vec![sh, suid, sudo],
        "ldconfig" => vec![sudo, lsuid],
        "less" => vec![sh, fw, fr, suid, sudo],
        "lftp" => vec![sh, sudo, lsuid],
        "ln" => vec![sudo],
        "loginctl" => vec![sh, sudo],
        "logsave" => vec![sh, suid, sudo],
        "look" => vec![fr, suid, sudo],
        "lp" => vec![fu],
        "ltrace" => vec![sh, fw, fr, sudo],
        "lua" => vec![sh, nirs, nibs, fu, fd, fw, fr, suid, sudo, lsuid],
        "lualatex" => vec![sh, sudo, lsuid],
        "luatex" => vec![sh, sudo, lsuid],
        "lwp-download" => vec![fd, fw, fr, sudo],
        "lwp-request" => vec![fr, sudo],
        "mail" => vec![sh, sudo],
        "make" => vec![sh, fw, suid, sudo],
        "man" => vec![sh, fr, sudo],
        "mawk" => vec![sh, fw, fr, suid, sudo, lsuid],
        "more" => vec![sh, fr, suid, sudo],
        "mosquitto" => vec![fr, suid, sudo],
        "mount" => vec![sudo],
        "msfconsole" => vec![sh, sudo],
        "msgattrib" => vec![fr, suid, sudo],
        "msgcat" => vec![fr, suid, sudo],
        "msgconv" => vec![fr, suid, sudo],
        "msgfilter" => vec![sh, fr, suid, sudo],
        "msgmerge" => vec![fr, suid, sudo],
        "msguniq" => vec![fr, suid, sudo],
        "mtr" => vec![fr, sudo],
        "multitime" => vec![sh, suid, sudo],
        "mv" => vec![suid, sudo],
        "mysql" => vec![sh, ll, sudo, lsuid],
        "nano" => vec![sh, fw, fr, sudo, lsuid],
        "nasm" => vec![fr, suid, sudo],
        "nawk" => vec![sh, nirs, nibs, fw, fr, suid, sudo, lsuid],
        "nc" => vec![rs, bs, fu, fd, sudo, lsuid],
        "neofetch" => vec![sh, fr, sudo],
        "nft" => vec![fr, suid, sudo],
        "nice" => vec![sh, suid, sudo],
        "nl" => vec![fr, suid, sudo],
        "nm" => vec![fr, suid, sudo],
        "nmap" => vec![sh, nirs, nibs, fu, fd, fw, fr, suid, sudo, lsuid],
        "node" => vec![sh, rs, bs, fu, fd, fw, fr, suid, sudo, cb],
        "nohup" => vec![sh, cmd, suid, sudo],
        "npm" => vec![sh, sudo],
        "nroff" => vec![sh, fr, sudo],
        "nsenter" => vec![sh, sudo],
        "octave" => vec![sh, fw, fr, sudo, lsuid],
        "od" => vec![fr, suid, sudo],
        "openssl" => vec![rs, fu, fd, fw, fr, ll, suid, sudo],
        "openvpn" => vec![sh, fr, suid, sudo],
        "openvt" => vec![sudo],
        "opkg" => vec![sudo],
        "paste" => vec![fr, suid, sudo],
        "pax" => vec![fr],
        "pdb" => vec![sh, sudo],
        "pdflatex" => vec![sh, fr, sudo, lsuid],
        "pdftex" => vec![sh, sudo, lsuid],
        "perf" => vec![sh, suid, sudo],
        "perl" => vec![sh, rs, fr, suid, sudo, cb],
        "perlbug" => vec![sh, sudo],
        "pexec" => vec![sh, suid, sudo],
        "pg" => vec![sh, fr, suid, sudo],
        "php" => vec![sh, cmd, rs, fu, fd, fw, fr, suid, sudo, cb],
        "pic" => vec![sh, fr, sudo, lsuid],
        "pico" => vec![sh, fw, fr, sudo, lsuid],
        "pidstat" => vec![cmd, suid, sudo],
        "pip" => vec![sh, rs, fu, fd, fw, fr, ll, sudo],
        "pkexec" => vec![sudo],
        "pkg" => vec![sudo],
        "posh" => vec![sh, sudo, lsuid],
        "pr" => vec![fr, suid, sudo],
        "pry" => vec![sh, sudo, lsuid],
        "psftp" => vec![sh, sudo, lsuid],
        "psql" => vec![sh, sudo],
        "ptx" => vec![fr, suid, sudo],
        "puppet" => vec![sh, fw, fr, sudo],
        "python" => vec![sh, rs, fu, fd, fw, fr, ll, suid, sudo, cb],
        "rake" => vec![sh, fr, sudo, lsuid],
        "readelf" => vec![fr, suid, sudo],
        "red" => vec![fw, fr, sudo],
        "redcarpet" => vec![fr, sudo],
        "redis" => vec![fw],
        "restic" => vec![fu, suid, sudo],
        "rev" => vec![fr, suid, sudo],
        "rlogin" => vec![fu],
        "rlwrap" => vec![sh, fw, suid, sudo],
        "rpm" => vec![sh, sudo, lsuid],
        "rpmdb" => vec![sh, sudo, lsuid],
        "rpmquery" => vec![sh, sudo, lsuid],
        "rpmverify" => vec![sh, sudo, lsuid],
        "rsync" => vec![sh, suid, sudo],
        "rtorrent" => vec![sh, suid],
        "ruby" => vec![sh, rs, fu, fd, fw, fr, ll, sudo, cb],
        "run-mailcap" => vec![sh, fw, fr, sudo],
        "run-parts" => vec![sh, suid, sudo],
        "rview" => vec![sh, rs, nirs, nibs, fu, fw, fr, ll, suid, sudo, cb, lsuid],
        "rvim" => vec![
            sh, rs, nirs, nibs, fu, fd, fw, fr, ll, suid, sudo, cb, lsuid,
        ],
        "sash" => vec![sh, suid, sudo],
        "scanmem" => vec![sh, suid, sudo],
        "scp" => vec![sh, fu, fd, sudo, lsuid],
        "screen" => vec![sh, fw, sudo],
        "script" => vec![sh, fw, sudo],
        "scrot" => vec![sh, sudo, lsuid],
        "sed" => vec![sh, cmd, fw, fr, suid, sudo],
        "service" => vec![sh, sudo],
        "setarch" => vec![sh, suid, sudo],
        "setfacl" => vec![suid, sudo],
        "setlock" => vec![sh, suid, sudo],
        "sftp" => vec![sh, fu, fd, sudo],
        "sg" => vec![sh, sudo],
        "shuf" => vec![fw, fr, suid, sudo],
        "slsh" => vec![sh, sudo, lsuid],
        "smbclient" => vec![sh, fu, fd, sudo],
        "snap" => vec![sudo],
        "socat" => vec![sh, rs, bs, fu, fd, fw, fr, sudo, lsuid],
        "socket" => vec![rs, bs],
        "soelim" => vec![fr, suid, sudo],
        "softlimit" => vec![sh, suid, sudo],
        "sort" => vec![fr, suid, sudo],
        "split" => vec![sh, cmd, fw, fr, sudo],
        "sqlite3" => vec![sh, fw, fr, suid, sudo, lsuid],
        "sqlmap" => vec![sh, sudo],
        "ss" => vec![fr, suid, sudo],
        "ssh_keygen" => vec![ll, suid, sudo],
        "ssh_keyscan" => vec![fr, suid, sudo],
        "ssh" => vec![sh, fu, fd, fr, sudo],
        "sshpass" => vec![sh, suid, sudo],
        "start_stop_daemon" => vec![sh, suid, sudo],
        "stdbuf" => vec![sh, suid, sudo],
        "strace" => vec![sh, fw, suid, sudo],
        "strings" => vec![fr, suid, sudo],
        "su" => vec![sudo],
        "sysctl" => vec![cmd, fr, suid, sudo],
        "systemctl" => vec![suid, sudo],
        "systemd_resolve" => vec![sudo],
        "tac" => vec![fr, suid, sudo],
        "tail" => vec![fr, suid, sudo],
        "tar" => vec![sh, fu, fd, fw, fr, sudo, lsuid],
        "task" => vec![sh, sudo],
        "taskset" => vec![sh, suid, sudo],
        "tasksh" => vec![sh, sudo, lsuid],
        "tbl" => vec![fr, suid, sudo],
        "tclsh" => vec![sh, nirs, suid, sudo],
        "tcpdump" => vec![cmd, sudo],
        "tdbtool" => vec![sh, sudo, lsuid],
        "tee" => vec![fw, suid, sudo],
        "telnet" => vec![sh, rs, sudo, lsuid],
        "tex" => vec![sh, sudo, lsuid],
        "tftp" => vec![fu, fd, suid, sudo],
        "tic" => vec![fr, suid, sudo],
        "time" => vec![sh, suid, sudo],
        "timedatectl" => vec![sh, sudo],
        "timeout" => vec![sh, suid, sudo],
        "tmate" => vec![sh, sudo, lsuid],
        "tmux" => vec![sh, fr, sudo],
        "top" => vec![sh, sudo],
        "torify" => vec![sh, sudo],
        "torsocks" => vec![sh, sudo],
        "troff" => vec![fr, suid, sudo],
        "tshark" => vec![sh],
        "ul" => vec![fr, suid, sudo],
        "unexpand" => vec![fr, suid, sudo],
        "uniq" => vec![fr, suid, sudo],
        "unshare" => vec![sh, suid, sudo],
        "unzip" => vec![suid, sudo],
        "update_alternatives" => vec![suid, sudo],
        "uudecode" => vec![fr, suid, sudo],
        "uuencode" => vec![fr, suid, sudo],
        "valgrind" => vec![sh, sudo],
        "vi" => vec![sh, fw, fr, sudo],
        "view" => vec![
            sh, rs, nirs, nibs, fu, fd, fw, fr, ll, suid, sudo, cb, lsuid,
        ],
        "vigr" => vec![suid, sudo],
        "vim" => vec![
            sh, rs, nirs, nibs, fu, fd, fw, fr, ll, suid, sudo, cb, lsuid,
        ],
        "vimdiff" => vec![
            sh, rs, nirs, nibs, fu, fd, fw, fr, ll, suid, sudo, cb, lsuid,
        ],
        "vipw" => vec![suid, sudo],
        "virsh" => vec![fw, fr, sudo],
        "volatility" => vec![sh],
        "w3m" => vec![fr, suid, sudo],
        "wall" => vec![sudo],
        "watch" => vec![sh, suid, sudo, lsuid],
        "wc" => vec![fr, suid, sudo],
        "wget" => vec![sh, fu, fd, fw, fr, suid, sudo],
        "whiptail" => vec![fr, suid, sudo],
        "whois" => vec![fu, fd],
        "wireshark" => vec![sudo],
        "wish" => vec![sh, nirs, sudo],
        "xargs" => vec![sh, fr, suid, sudo],
        "xdotool" => vec![sh, suid, sudo],
        "xelatex" => vec![sh, fr, sudo, lsuid],
        "xetex" => vec![sh, sudo, lsuid],
        "xmodmap" => vec![fr, suid, sudo],
        "xmore" => vec![fr, suid, sudo],
        "xpad" => vec![fr, sudo],
        "xxd" => vec![fw, fr, suid, sudo],
        "xz" => vec![fr, suid, sudo],
        "yarn" => vec![sh, sudo],
        "yash" => vec![sh, suid, sudo],
        "yelp" => vec![fr],
        "yum" => vec![fd, sudo],
        "zathura" => vec![sh, sudo],
        "zip" => vec![sh, fr, sudo, lsuid],
        "zsh" => vec![sh, fw, fr, suid, sudo],
        "zsoelim" => vec![fr, suid, sudo],
        "zypper" => vec![sh, sudo],
        _ => vec![sh, sudo],
    }
}

pub mod ab;
pub mod agetty;
pub mod alpine;
pub mod ansible_playbook; // ansible-playbook
pub mod aoss;
pub mod apt;
pub mod apt_get; // apt-get
pub mod ar;
pub mod aria2c;
pub mod arj;
pub mod arp;
pub mod as_; // as
pub mod ascii85;
pub mod ascii_xfr; // ascii_xfr
pub mod ash;
pub mod aspell;
pub mod at;
pub mod atobm;
pub mod awk;
pub mod aws;
pub mod base32;
pub mod base58;
pub mod base64;
pub mod basenc;
pub mod basez;
pub mod bash;
pub mod batcat;
pub mod bc;
pub mod bconsole;
pub mod bpftrace;
pub mod bridge;
pub mod bundle;
pub mod bundler;
pub mod busctl;
pub mod busybox;
pub mod byebug;
pub mod bzip2;
pub mod c89;
pub mod c99;
pub mod cabal;
pub mod cancel;
pub mod capsh;
pub mod cat;
pub mod cdist;
pub mod certbot;
pub mod check_by_ssh;
pub mod check_cups;
pub mod check_log;
pub mod check_memory;
pub mod check_raid;
pub mod check_ssl_cert;
pub mod check_statusfile;
pub mod chmod;
pub mod choom;
pub mod chown;
pub mod chroot;
pub mod cmp;
pub mod cobc;
pub mod column;
pub mod comm;
pub mod composer;
pub mod cowsay;
pub mod cowthink;
pub mod cp;
pub mod cpan;
pub mod cpio;
pub mod cpulimit;
pub mod crash;
pub mod crontab;
pub mod csh;
pub mod csplit;
pub mod csvtool;
pub mod cupsfilter;
pub mod curl;
pub mod cut;
pub mod dash;
pub mod date;
pub mod dd;
pub mod debugfs;
pub mod dialog;
pub mod diff;
pub mod dig;
pub mod distcc;
pub mod dmesg;
pub mod dmidecode;
pub mod dmsetup;
pub mod dnf;
pub mod docker;
pub mod dosbox;
pub mod dotnet;
pub mod dpkg;
pub mod dstat;
pub mod dvips;
pub mod easy_install;
pub mod eb;
pub mod ed;
pub mod efax;
pub mod emacs;
pub mod env;
pub mod eqn;
pub mod espeak;
pub mod ex;
pub mod exiftool;
pub mod expand;
pub mod expect;
pub mod facter;
pub mod file;
pub mod find;
pub mod finger;
pub mod fish;
pub mod flock;
pub mod fmt;
pub mod fold;
pub mod fping;
pub mod ftp;
pub mod gawk;
pub mod gcc;
pub mod gcloud;
pub mod gcore;
pub mod gdb;
pub mod gem;
pub mod genie;
pub mod genisoimage;
pub mod ghc;
pub mod ghci;
pub mod gimp;
pub mod ginsh;
pub mod git;
pub mod grc;
pub mod grep;
pub mod gtester;
pub mod gzip;
pub mod hd;
pub mod head;
pub mod hexdump;
pub mod highlight;
pub mod hping3;
pub mod iconv;
pub mod iftop;
pub mod install;
pub mod ionice;
pub mod ip;
pub mod irb;
pub mod ispell;
pub mod jjs;
pub mod joe;
pub mod join;
pub mod journalctl;
pub mod jq;
pub mod jrunscript;
pub mod jtag;
pub mod knife;
pub mod ksh;
pub mod ksshell;
pub mod ksu;
pub mod kubectl;
pub mod latex;
pub mod latexmk;
pub mod ld_so; // ld.so
pub mod ldconfig;
pub mod less;
pub mod lftp;
pub mod ln;
pub mod loginctl;
pub mod logsave;
pub mod look;
pub mod lp;
pub mod ltrace;
pub mod lua;
pub mod lualatex;
pub mod luatex;
pub mod lwp_download; // lwp-download
pub mod lwp_request; // lwp-request
pub mod mail;
pub mod make;
pub mod man;
pub mod mawk;
pub mod more;
pub mod mosquitto;
pub mod mount;
pub mod msfconsole;
pub mod msgattrib;
pub mod msgcat;
pub mod msgconv;
pub mod msgfilter;
pub mod msgmerge;
pub mod msguniq;
pub mod mtr;
pub mod multitime;
pub mod mv;
pub mod mysql;
pub mod nano;
pub mod nasm;
pub mod nawk;
pub mod nc;
pub mod neofetch;
pub mod nft;
pub mod nice;
pub mod nl;
pub mod nm;
pub mod nmap;
pub mod node;
pub mod nohup;
pub mod npm;
pub mod nroff;
pub mod nsenter;
pub mod octave;
pub mod od;
pub mod openssl;
pub mod openvpn;
pub mod openvt;
pub mod opkg;
pub mod pandoc;
pub mod paste;
pub mod pax;
pub mod pdb;
pub mod pdflatex;
pub mod pdftex;
pub mod perf;
pub mod perl;
pub mod perlbug;
pub mod pexec;
pub mod pg;
pub mod php;
pub mod pic;
pub mod pico;
pub mod pidstat;
pub mod pip;
pub mod pkexec;
pub mod pkg;
pub mod posh;
pub mod pr;
pub mod pry;
pub mod psftp;
pub mod psql;
pub mod ptx;
pub mod puppet;
pub mod python;
pub mod rake;
pub mod readelf;
pub mod red;
pub mod redcarpet;
pub mod redis;
pub mod restic;
pub mod rev;
pub mod rlogin;
pub mod rlwrap;
pub mod rpm;
pub mod rpmdb;
pub mod rpmquery;
pub mod rpmverify;
pub mod rsync;
pub mod rtorrent;
pub mod ruby;
pub mod run_mailcap; // run-mailcap
pub mod run_parts;
pub mod rview;
pub mod rvim;
pub mod sash;
pub mod scanmem;
pub mod scp;
pub mod screen;
pub mod script;
pub mod scrot;
pub mod sed;
pub mod service;
pub mod setarch;
pub mod setfacl;
pub mod setlock;
pub mod sevenz;
pub mod sftp;
pub mod sg;
pub mod shuf;
pub mod slsh;
pub mod smbclient;
pub mod snap;
pub mod socat;
pub mod socket;
pub mod soelim;
pub mod softlimit;
pub mod sort;
pub mod split;
pub mod sqlite3;
pub mod sqlmap;
pub mod ss;
pub mod ssh;
pub mod ssh_keygen; // ssh-keygen
pub mod ssh_keyscan; // ssh-keyscan
pub mod sshpass;
pub mod start_stop_daemon; // start-stop-daemon
pub mod stdbuf;
pub mod strace;
pub mod strings;
pub mod su;
pub mod sysctl;
pub mod systemctl;
pub mod systemd_resolve; // systemd-resolve
pub mod tac;
pub mod tail;
pub mod tar;
pub mod task;
pub mod taskset;
pub mod tasksh;
pub mod tbl;
pub mod tclsh;
pub mod tcpdump;
pub mod tdbtool;
pub mod tee;
pub mod telnet;
pub mod tex;
pub mod tftp;
pub mod tic;
pub mod time;
pub mod timedatectl;
pub mod timeout;
pub mod tmate;
pub mod tmux;
pub mod top;
pub mod torify;
pub mod torsocks;
pub mod troff;
pub mod tshark;
pub mod ul;
pub mod unexpand;
pub mod uniq;
pub mod unshare;
pub mod unzip;
pub mod update_alternatives; // update-alternatives
pub mod uudecode;
pub mod uuencode;
pub mod valgrind;
pub mod vi;
pub mod view;
pub mod vigr;
pub mod vim;
pub mod vimdiff;
pub mod vipw;
pub mod virsh;
pub mod volatility;
pub mod w3m;
pub mod wall;
pub mod watch;
pub mod wc;
pub mod wget;
pub mod whiptail;
pub mod whois;
pub mod wireshark;
pub mod wish;
pub mod xargs;
pub mod xdotool;
pub mod xelatex;
pub mod xetex;
pub mod xmodmap;
pub mod xmore;
pub mod xpad;
pub mod xxd;
pub mod xz;
pub mod yarn;
pub mod yash;
pub mod yelp;
pub mod yum;
pub mod zathura;
pub mod zip;
pub mod zsh;
pub mod zsoelim;
pub mod zypper;
