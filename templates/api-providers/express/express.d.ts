/**
 * Extend the Express Request to add aditional properties with types
 */

// import type { Session } from './src/auth';

declare global {
  namespace Express {
    interface Request {
      // session: Session;
    }
  }
}
