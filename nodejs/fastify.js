const fastify = require("fastify")({
  logger: false,
});

fastify.get("/", function (request, reply) {
  reply.send("Hello World!");
});

fastify.listen({ port: 3000 }, function (err, address) {
  console.log(`Server listening on ${address}`);
  if (err) {
    fastify.log.error(err);
    process.exit(1);
  }
});
