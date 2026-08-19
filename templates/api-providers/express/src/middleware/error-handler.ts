import type { ErrorRequestHandler, RequestHandler } from "express";
import { HTTPStatusCodes } from "../constants/http-status-codes";
import { AppError } from "../lib/app-error";
import { sendError } from "../lib/api-responce";

export const notFoundHandler: RequestHandler = (request, _response, next) => {
  next(
    new AppError(
      HTTPStatusCodes.NOT_FOUND,
      `Route ${request.method} ${request.originalUrl} was not found.`,
    ),
  );
};

export const errorHandler: ErrorRequestHandler = (
  error,
  request,
  response,
  _next,
) => {
  if (error instanceof AppError) {
    sendError(response, error.statusCode, error.message, error.errors);
    return;
  }

  request.log.error({ err: error }, "Unhandled request error");
  sendError(
    response,
    HTTPStatusCodes.INTERNAL_SERVER_ERROR,
    "An unexpected error occurred.",
  );
};
