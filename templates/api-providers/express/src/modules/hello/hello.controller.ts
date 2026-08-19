import type { Request, Response, NextFunction } from "express";
import { getHelloService } from "./hello.service";
import { sendSuccess } from "../../lib/api-responce";
import { logger } from "../../lib/logger";
import { HTTPStatusCodes } from "../../constants/http-status-codes";

export const getHelloController = (
  _req: Request,
  res: Response,
  _next: NextFunction,
) => {
  logger.info("getHelloController called");

  const message = getHelloService();
  sendSuccess(res, HTTPStatusCodes.OK, "success", message);
};
