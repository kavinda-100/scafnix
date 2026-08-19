import type { Request, Response, NextFunction } from "express";
import { z } from "@{{project_name}}/schema";
import { sendError } from "../lib/api-responce";
import { HTTPStatusCodes } from "../constants/http-status-codes";

export const validateHeaders = (headers: Record<string, z.ZodType>) => {
  return (req: Request, res: Response, next: NextFunction) => {
    const missingHeaders: string[] = [];

    for (const [header, schema] of Object.entries(headers)) {
      const value = req.get(header);

      if (!value) {
        missingHeaders.push(header);
        continue;
      }

      const result = schema.safeParse(value);

      if (!result.success) {
        sendError(
          res,
          HTTPStatusCodes.BAD_REQUEST,
          `Invalid required header: ${header}`,
        );
        return;
      }
    }

    if (missingHeaders.length > 0) {
      sendError(
        res,
        HTTPStatusCodes.BAD_REQUEST,
        `Missing required headers: ${missingHeaders.join(", ")}`,
      );
      return;
    }

    next();
  };
};
