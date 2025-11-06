import { sql } from "drizzle-orm";
import { int, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const schedules = sqliteTable("schedules", {
  id: int().primaryKey({ autoIncrement: true }),
  jsonData: text().notNull(),
  date: text().notNull().default(sql`current_timestamp`),
});
