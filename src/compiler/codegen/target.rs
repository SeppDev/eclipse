use std::path::PathBuf;

#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum Arch {
    #[default]
    Unkown,
    ARM32,
    ARM64,
    x86_32,
    x86_64,
}
impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unkown => "unkown",
                Self::ARM32 => "arm",
                Self::ARM64 => "aarch64",
                Self::x86_32 => "x86",
                Self::x86_64 => "x86_64",
            }
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum Vendor {
    Unkown,
    #[default]
    PC,
}
impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unkown => "unkown",
                Self::PC => "pc",
            }
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum OS {
    #[default]
    Unkown,
    Linux,
    Windows,
}
impl std::fmt::Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unkown => "unkown",
                Self::Linux => "linux",
                Self::Windows => "windows",
            }
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum Environment {
    #[default]
    Unkown,
    GNU,
    MSVC,
}
impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unkown => "unkown",
                Self::GNU => "gnu",
                Self::MSVC => "msvc",
            }
        )
    }
}

fn current_arch() -> Arch {
    if cfg!(target_arch = "x86") {
        Arch::x86_32
    } else if cfg!(target_arch = "x86_64") {
        Arch::x86_64
    } else if cfg!(target_arch = "arm") {
        Arch::ARM32
    } else if cfg!(target_arch = "aarch64") {
        Arch::ARM64
    } else {
        Arch::default()
    }
}

fn current_vendor() -> Vendor {
    if cfg!(target_vendor = "pc") {
        Vendor::PC
    } else {
        Vendor::default()
    }
}

fn current_os() -> OS {
    if cfg!(target_os = "linux") {
        OS::Linux
    } else if cfg!(target_os = "windows") {
        OS::Windows
    } else {
        OS::default()
    }
}

fn current_env() -> Environment {
    if cfg!(target_env = "gnu") {
        Environment::GNU
    } else if cfg!(target_env = "msvc") {
        Environment::MSVC
    } else {
        Environment::default()
    }
}

#[derive(Default)]
pub struct Target {
    pub arch: Arch,
    pub vendor: Vendor,
    pub os: OS,
    pub env: Option<Environment>,
}
impl Target {
    pub fn new() -> Self {
        Self {
            arch: current_arch(),
            vendor: current_vendor(),
            os: current_os(),
            env: Some(current_env()),
        }
    }
    pub fn set_extension(&self, path: &mut PathBuf) {
        match self.os {
            OS::Windows => path.set_extension(".exe"),
            _ => return,
        };
    }
    pub fn pointer_width(&self) -> usize {
        use Arch::*;
        match self.arch {
            Unkown => 32,
            ARM32 => 32,
            ARM64 => 64,
            x86_32 => 32,
            x86_64 => 64,
        }
    }
}
impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let arch = &self.arch;
        let vendor = &self.vendor;
        let os = &self.os;
        let env = &self.env;

        match env {
            Some(env) => write!(f, "{arch}-{vendor}-{os}-{env}"),
            None => write!(f, "{arch}-{vendor}-{os}"),
        }
    }
}
