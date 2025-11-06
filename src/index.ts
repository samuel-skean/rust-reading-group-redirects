import assert from "node:assert";
import { desc } from "drizzle-orm";
import { drizzle } from "drizzle-orm/d1";
import { Hono } from "hono";
import * as schema from "#/src/db/schema";
import { DEFAULT_URL } from "./constants";
import { ScheduleValidator } from "./types-and-validators";
import { getCurrentUrlFromSchedule } from "./utils/getCurrentUrlFromSchedule";

const app = new Hono<{ Bindings: CloudflareBindings }>();

app.get("/schedule", async (c) => {
  const db = drizzle(c.env.db);

  const result = await db
    .select()
    .from(schema.schedules)
    .orderBy(desc(schema.schedules.id))
    .limit(1);

  if (result.length < 1) {
    return c.redirect(DEFAULT_URL);
  }
  // TODO: Fix!! This produces JSON with escapes in it because the escapes are part of how its stored in the database (I think).
  return c.json(result[0].jsonData);
});

app.put("/schedule", async (c) => {
  const db = drizzle(c.env.db);

  const parseResult = ScheduleValidator.safeParse(await c.req.json());

  if (parseResult.error) {
    return c.json(JSON.parse(parseResult.error.message), 400);
  }

  const result = await db
    .insert(schema.schedules)
    .values([{ jsonData: await (await c.req.blob()).text() }])
    .returning();
  // TODO: Fix!! This produces JSON with escapes in it because the escapes are part of how its stored in the database (I think).
  return c.json(result[0].jsonData);
});

app.get("/redirect", async (c) => {
  const db = drizzle(c.env.db);

  const currentSchedule = (
    await db.select().from(schema.schedules).orderBy(desc(schema.schedules.id))
  ).at(0);

  if (!currentSchedule) {
    return c.redirect(DEFAULT_URL);
  }

  const scheduleParseResult = ScheduleValidator.safeParse(
    JSON.parse(currentSchedule.jsonData),
  );

  if (scheduleParseResult.error) {
    c.json(JSON.parse(scheduleParseResult.error.message), 400);
  }

  if (scheduleParseResult.success) {
    return c.redirect(getCurrentUrlFromSchedule(scheduleParseResult.data));
  }

  assert(false);
});

export default app;
