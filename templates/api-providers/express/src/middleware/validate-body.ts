import type { Request, Response, NextFunction } from "express";
import { z } from "@{{project_name}}/schema";
import { sendError } from "../lib/api-responce";
import { HTTPStatusCodes } from "../constants/http-status-codes";

export const validateBody = (schema: z.ZodSchema) => {
  return (req: Request, res: Response, next: NextFunction) => {
    const result = schema.safeParse(req.body);

    if (!result.success) {
      const errorMessages = result.error.issues
        .map((issue) => `${issue.path.join(".")} - ${issue.message}`)
        .join(", ");
      sendError(
        res,
        HTTPStatusCodes.BAD_REQUEST,
        `Invalid request body`,
        errorMessages,
      );
      return;
    }

    req.body = result.data;
    next();
  };
};
