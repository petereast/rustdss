const test  = require('ava');
const Redis = require('ioredis');

const clientA = new Redis("6379", "redis")
const clientB = new Redis("6379", "rustdss")

test("it connects to both clients", async (t) => {
  t.truthy(await clientA.ping());
  t.truthy(await clientB.ping());
})

test.todo("get and set behaviour is the same");

test.todo("it can incr and decr a value");

// Test get/set behaviour
