import dotenv from "dotenv";
import findConfig from "find-config";
import { DatabaseEnvSchema } from "@{{project_name}}/schemas";

// Automatically traverse upward until the root monorepo .env file is found
const envPath = findConfig(".env");

if (envPath) {
  dotenv.config({ path: envPath });
}

const validatedEnv = DatabaseEnvSchema.safeParse(process.env);

if (!validatedEnv.success)
  throw new Error(
    `Invalid environment variables on Database: ${validatedEnv.error.issues.map((i) => i.message).join(", ")}`,
  );

export const env = validatedEnv.data;
