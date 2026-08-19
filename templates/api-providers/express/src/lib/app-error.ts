import type { HTTPStatusCode } from "../constants/http-status-codes";

export class AppError extends Error {
  constructor(
    public readonly statusCode: HTTPStatusCode,
    message: string,
    public readonly errors?: unknown,
  ) {
    super(message);
    this.name = "AppError";
  }
}
