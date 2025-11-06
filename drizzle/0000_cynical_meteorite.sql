CREATE TABLE `schedules` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`jsonData` text NOT NULL,
	`date` text DEFAULT current_timestamp NOT NULL
);
