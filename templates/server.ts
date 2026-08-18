import express from "express";
import type { Request, Response } from "express";

const app = express();

app.get("/", (_req: Request, res: Response) => {
  res.status(200).json({
    message: "Hello from Scafnix",
  });
});

app.listen(3000, () => {
  console.log("Server running on http://localhost:3000");
});
