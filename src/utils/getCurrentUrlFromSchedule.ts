import assert from "node:assert";
import { Temporal } from "@js-temporal/polyfill";
import { DEFAULT_URL } from "#/src/constants";
import type * as typesAndValidators from "#/src/types-and-validators";

export const getCurrentUrlFromSchedule = (
  schedule: typesAndValidators.Schedule,
): string => {
  const now = Temporal.Now.instant();
  console.log(schedule);
  const currentlyActiveRanges = schedule.filter((range) => {
    return (
      Temporal.Instant.compare(
        now,
        Temporal.Instant.fromEpochMilliseconds(range.start),
      ) >= 0 &&
      Temporal.Instant.compare(
        now,
        Temporal.Instant.fromEpochMilliseconds(range.end),
      ) < 0
    );
  });
  assert(currentlyActiveRanges.length <= 1);

  return currentlyActiveRanges[0]?.value ?? DEFAULT_URL;
};
