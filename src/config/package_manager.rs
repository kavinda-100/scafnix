use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PackageManager {
    Pnpm,
    Bun,
}

#[allow(dead_code)]
impl PackageManager {
    pub fn executable(&self) -> &'static str {
        match self {
            Self::Pnpm => "pnpm",
            Self::Bun => "bun",
        }
    }

    pub fn install_args(&self) -> &'static [&'static str] {
        match self {
            Self::Pnpm => &["install"],
            Self::Bun => &["install"],
        }
    }
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pnpm => write!(f, "pnpm"),
            Self::Bun => write!(f, "bun"),
        }
    }
}
