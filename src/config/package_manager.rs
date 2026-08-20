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

    pub fn upgrade_args(&self) -> &'static [&'static str] {
        match self {
            Self::Pnpm => &["up", "--latest", "--recursive"],
            Self::Bun => &["update", "--latest"],
        }
    }

    pub fn start_command(&self) -> &'static str {
        match self {
            Self::Pnpm => r#"pnpm --parallel --filter \"./apps/*\" start"#,
            Self::Bun => r#"bun --filter \"./apps/*\" start"#,
        }
    }

    pub fn predev_command(&self) -> &'static str {
        match self {
            Self::Pnpm => r#"pnpm --filter \"./packages/*\" build"#,
            Self::Bun => r#"bun --filter \"./packages/*\" build"#,
        }
    }

    pub fn dev_command(&self) -> &'static str {
        match self {
            Self::Pnpm => r#"pnpm --parallel --filter \"./apps/*\" dev"#,
            Self::Bun => r#"bun --filter \"./apps/*\" dev"#,
        }
    }

    pub fn build_command(&self) -> &'static str {
        match self {
            Self::Pnpm => {
                r#"pnpm --filter \"./packages/*\" build && pnpm --filter \"./apps/*\" build"#
            }
            Self::Bun => {
                r#"bun --filter \"./packages/*\" build && bun --filter \"./apps/*\" build"#
            }
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
