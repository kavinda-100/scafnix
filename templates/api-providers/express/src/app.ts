import cors from "cors";
import express from "express";
import type { Application, Request, Response } from "express";

import { HTTPStatusCodes } from "./constants/http-status-codes";
import { requestLogger } from "./lib/logger";
import { errorHandler, notFoundHandler } from "./middleware/error-handler";
import { sendSuccess } from "./lib/api-responce";
import { env } from "./env";
import V1Router from "./route";

export const app: Application = express();

app.use(
  cors({
    origin: env.WEB_URL || "http://localhost:3000",
    credentials: true,
  }),
);
app.use(requestLogger);
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.get("/api/v1/health", (_request: Request, response: Response) => {
  sendSuccess(response, HTTPStatusCodes.OK, "API is healthy.", {
    status: "ok",
  });
});

app.use("/api/v1", V1Router);

app.use(notFoundHandler);
app.use(errorHandler);
