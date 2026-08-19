import { app } from "./app";
import { env } from "./env";
import { logger } from "./lib/logger";

const server = app.listen(env.PORT, () => {
  logger.info(
    { port: env.PORT },
    `API server listening on http://localhost:${env.PORT}`,
  );
});

function shutdown(): void {
  server.closeAllConnections();
  server.closeIdleConnections();

  server.close((error) => {
    if (error) {
      logger.error({ err: error }, "Graceful shutdown failed.");
      logger.flush();
      process.exit(1);
    }

    logger.info("Graceful shutdown completed.");
    logger.flush();
    process.exit(0);
  });
}

process.on("SIGINT", () => shutdown());
process.on("SIGTERM", () => shutdown());
