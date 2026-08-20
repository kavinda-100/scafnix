import express, { type Router } from "express";
import helloRoute from "./modules/hello/hello.route";

const V1Router: Router = express.Router();

V1Router.use("/hello", helloRoute);

export default V1Router;
