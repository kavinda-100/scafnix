use std::fmt;

#[derive(Debug, Clone)]
pub enum ApiFramework {
    Express,
    // Hono,
    // Fastify,
}

impl fmt::Display for ApiFramework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Express => write!(f, "Express"),
            // Self::Hono => write!(f, "Hono"),
            // Self::Fastify => write!(f, "Fastify"),
        }
    }
}
