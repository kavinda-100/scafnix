import express, { type Router } from "express";
import { getHelloController } from "./hello.controller";

const router: Router = express.Router();

router.get("/", getHelloController);

export default router;
