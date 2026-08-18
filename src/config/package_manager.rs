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
}
