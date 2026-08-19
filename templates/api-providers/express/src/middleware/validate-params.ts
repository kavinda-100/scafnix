import type { NextFunction, Request, Response } from "express";
import { z } from "@{{project_name}}/schema";
import { HTTPStatusCodes } from "../constants/http-status-codes";
import { sendError } from "../lib/api-responce";

export const validateParams = (schema: z.ZodSchema) => {
  return (req: Request, res: Response, next: NextFunction) => {
    const result = schema.safeParse(req.params);

    if (!result.success) {
      const errorMessages = result.error.issues
        .map((issue) => `${issue.path.join(".")} - ${issue.message}`)
        .join(", ");
      sendError(
        res,
        HTTPStatusCodes.BAD_REQUEST,
        "Invalid route parameters",
        errorMessages,
      );
      return;
    }

    Object.assign(req.params, result.data);
    next();
  };
};
