import dotenv from "dotenv";
import findConfig from "find-config";
import { ApiEnvSchema } from "@{{project_name}}/schema";

const envPath = findConfig(".env");

if (envPath) {
  dotenv.config({ path: envPath });
} else {
  throw new Error(
    "No .env file found in the project root. Please create one at the root. [from api]",
  );
}

const validatedEnv = ApiEnvSchema.safeParse(process.env);

if (!validatedEnv.success) {
  throw new Error(
    `Invalid environment variables on API: ${validatedEnv.error.issues.map((issue) => issue.message).join(", ")}`,
  );
}

export const env = validatedEnv.data;
