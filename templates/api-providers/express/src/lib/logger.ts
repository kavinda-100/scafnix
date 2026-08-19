import { randomUUID } from "node:crypto";
import pino from "pino";
import pinoHttp from "pino-http";
import { env } from "../env";

const transport =
  env.NODE_ENV === "development"
    ? {
        target: "pino-pretty",
        options: {
          colorize: true,
          singleLine: true,
          translateTime: "SYS:standard",
        },
      }
    : undefined;

export const logger: pino.Logger = pino({
  level: env.LOG_LEVEL,
  redact: {
    paths: [
      "req.headers.authorization",
      "req.headers.cookie",
      "res.headers.set-cookie",
      "headers.authorization",
      "headers.cookie",
      "headers.set-cookie",
    ],
    censor: "[REDACTED]",
  },
  transport,
});

export const requestLogger = pinoHttp({
  logger,
  genReqId: (request, response) => {
    const requestId = request.headers["x-request-id"];
    const providedId = Array.isArray(requestId) ? requestId[0] : requestId;
    const id = providedId ?? randomUUID();

    response.setHeader("x-request-id", id);
    return id;
  },
  autoLogging: {
    ignore: (request) =>
      request.method === "GET" && request.url === "/api/health",
  },
  customLogLevel: (_request, response, error) => {
    if (error || response.statusCode >= 500) return "error";
    if (response.statusCode >= 400) return "warn";
    return "info";
  },
});
