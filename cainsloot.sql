BEGIN;
CREATE TABLE "users" (
  "id" SERIAL PRIMARY KEY,
  "char_name" varchar UNIQUE,
  "class" varchar
);

CREATE TABLE "classes" (
  "name" varchar PRIMARY KEY
);

CREATE TABLE "items" (
  "id" SERIAL PRIMARY KEY,
  "wowhead_id" int UNIQUE,
  "raid" varchar,
  "name" varchar UNIQUE
);

CREATE TABLE "raids" (
  "name" varchar PRIMARY KEY
);

CREATE TABLE "past_raids" (
  "id" SERIAL PRIMARY KEY,
  "raid" varchar,
  "date" date
);

CREATE TABLE "archived_loot" (
  "id" SERIAL PRIMARY KEY,
  "received_user_id" int,
  "received_item_id" int,
  "raid_id" int
);

CREATE TABLE "attendence" (
  "user_id" int,
  "raid_id" int,
  "late" boolean DEFAULT false,
  "bench" boolean DEFAULT false,
  PRIMARY KEY ("user_id", "raid_id")
);

CREATE TABLE "loot_prios" (
  "id" SERIAL PRIMARY KEY,
  "raid" varchar,
  "user_id" int,
  "priority" int,
  "col_1_item_id" int,
  "col_2_item_id" int,
  UNIQUE (raid,user_id,priority)
);

ALTER TABLE "users" ADD FOREIGN KEY ("class") REFERENCES "classes" ("name");

ALTER TABLE "items" ADD FOREIGN KEY ("raid") REFERENCES "raids" ("name");

ALTER TABLE "past_raids" ADD FOREIGN KEY ("raid") REFERENCES "raids" ("name");

ALTER TABLE "archived_loot" ADD FOREIGN KEY ("received_user_id") REFERENCES "users" ("id");

ALTER TABLE "archived_loot" ADD FOREIGN KEY ("received_item_id") REFERENCES "items" ("id");

ALTER TABLE "archived_loot" ADD FOREIGN KEY ("raid_id") REFERENCES "past_raids" ("id");

ALTER TABLE "attendence" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "attendence" ADD FOREIGN KEY ("raid_id") REFERENCES "past_raids" ("id");

ALTER TABLE "loot_prios" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "loot_prios" ADD FOREIGN KEY ("col_1_item_id") REFERENCES "items" ("id");

ALTER TABLE "loot_prios" ADD FOREIGN KEY ("col_2_item_id") REFERENCES "items" ("id");

INSERT INTO "classes"(name) VALUES ('Rogue'), ('Warrior'), ('Mage'), ('Shaman'), ('Paladin'), ('Druid'), ('Warlock'), ('Hunter'), ('Priest');

INSERT INTO "raids"(name) VALUES ('Gruul'), ('Mag'), ('SSC'), ('TK'), ('BT'), ('Hyjaal'), ('Sunwell');
COMMIT;
