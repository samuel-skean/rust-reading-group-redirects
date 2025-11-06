import "dotenv/config";
import { defineConfig } from "drizzle-kit";

export default defineConfig({
  schema: "./src/db/schema.ts",
  out: "./drizzle",
  dialect: "sqlite",
  driver: "d1-http",
  dbCredentials: {
    // biome-ignore lint/style/noNonNullAssertion: TODO: I dunno how to do this right...
    accountId: process.env.CLOUDFLARE_ACCOUNT_ID!,
    // biome-ignore lint/style/noNonNullAssertion: TODO: I dunno how to do this right...
    databaseId: process.env.CLOUDFLARE_DATABASE_ID!,
    // biome-ignore lint/style/noNonNullAssertion: TODO: I dunno how to do this right...
    token: process.env.CLOUDFLARE_D1_TOKEN!,
  },
});
