import express from "express";
import helloRoute from "./modules/hello/hello.route";

const V1Router = express.Router();

V1Router.use("/hello", helloRoute);

export default V1Router;
