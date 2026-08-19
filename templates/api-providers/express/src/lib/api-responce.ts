import type { Response } from "express";
import type { HTTPStatusCode } from "../constants/http-status-codes";

export type ApiSuccessResponse<T> = {
  success: true;
  message: string;
  data: T;
};

export type ApiErrorResponse = {
  success: false;
  message: string;
  errors?: unknown;
};

export function sendSuccess<T>(
  response: Response,
  statusCode: HTTPStatusCode,
  message: string,
  data: T,
): void {
  response
    .status(statusCode)
    .json({ success: true, message, data } satisfies ApiSuccessResponse<T>);
}

export function sendError(
  response: Response,
  statusCode: HTTPStatusCode,
  message: string,
  errors?: unknown,
): void {
  response.status(statusCode).json({
    success: false,
    message,
    ...(errors === undefined ? {} : { errors }),
  } satisfies ApiErrorResponse);
}
