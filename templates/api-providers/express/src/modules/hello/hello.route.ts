import express from "express";
import { getHelloController } from "./hello.controller";

const router = express.Router();

router.get("/", getHelloController);

export default router;
