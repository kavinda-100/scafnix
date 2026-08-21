use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OrmProvider {
    Prisma,
    Drizzle,
}

impl fmt::Display for OrmProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prisma => write!(f, "Prisma"),
            Self::Drizzle => write!(f, "Drizzle"),
        }
    }
}
