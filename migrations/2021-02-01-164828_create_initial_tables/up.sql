CREATE TABLE "users"
(
    "id"            SERIAL       NOT NULL PRIMARY KEY,
    "username"      VARCHAR(255) NOT NULL,
    "email"         VARCHAR(255) NOT NULL UNIQUE,
    "password"      VARCHAR(255) NOT NULL,
    "login_session" VARCHAR(32),
    "created_at"    TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at"    TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "tasks"
(
    "id"           SERIAL       NOT NULL PRIMARY KEY,
    "user_id"      INTEGER      NOT NULL,
    "project_id"   INTEGER      NOT NULL,
    "section_id"   INTEGER      NOT NULL,
    "parent_id"    INTEGER      NOT NULL,
    "title"        VARCHAR(255) NOT NULL,
    "description"  VARCHAR(255) NOT NULL,
    "due_date"     DATE NULL,
    "due_datetime" TIMESTAMP(0) WITHOUT TIME ZONE NULL,
    "priority"     INTEGER      NOT NULL,
    "order"        INTEGER      NOT NULL,
    "is_completed" BOOLEAN      NOT NULL,
    "is_favorite"  BOOLEAN      NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "projects"
(
    "id"          SERIAL       NOT NULL PRIMARY KEY,
    "user_id"     INTEGER      NOT NULL,
    "color_id"    INTEGER      NOT NULL,
    "parent_id"   INTEGER      NOT NULL,
    "name"        VARCHAR(255) NOT NULL,
    "order"       INTEGER      NOT NULL,
    "is_inbox"    BOOLEAN      NOT NULL,
    "is_favorite" BOOLEAN      NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "labels"
(
    "id"          SERIAL       NOT NULL PRIMARY KEY,
    "user_id"     INTEGER      NOT NULL,
    "color_id"    INTEGER      NOT NULL,
    "name"        VARCHAR(255) NOT NULL,
    "order"       INTEGER      NOT NULL,
    "is_favorite" BOOLEAN      NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "sections"
(
    "id"          SERIAL       NOT NULL PRIMARY KEY,
    "user_id"    INTEGER      NOT NULL,
    "project_id" INTEGER      NOT NULL,
    "name"       VARCHAR(255) NOT NULL,
    "order"      INTEGER      NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "comments"
(
    "id"          SERIAL       NOT NULL PRIMARY KEY,
    "user_id"    INTEGER      NOT NULL,
    "project_id" INTEGER      NOT NULL,
    "task_id"    INTEGER      NOT NULL,
    "parent_id"  INTEGER      NOT NULL,
    "title"      VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "colors"
(
    "id"          SERIAL       NOT NULL PRIMARY KEY,
    "name"       VARCHAR(255) NOT NULL,
    "color_code" VARCHAR(255) NOT NULL,
    "order"      INTEGER      NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "tasks_labels"
(
    "task_id"  INTEGER NOT NULL,
    "label_id" INTEGER NOT NULL,
    PRIMARY KEY ("task_id", "label_id")
);

ALTER TABLE
    "comments"
    ADD CONSTRAINT "comments_task_id_foreign" FOREIGN KEY ("task_id") REFERENCES "tasks" ("id");
ALTER TABLE
    "tasks"
    ADD CONSTRAINT "tasks_parent_id_foreign" FOREIGN KEY ("parent_id") REFERENCES "tasks" ("id");
ALTER TABLE
    "tasks"
    ADD CONSTRAINT "tasks_user_id_foreign" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE
    "labels"
    ADD CONSTRAINT "labels_user_id_foreign" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE
    "projects"
    ADD CONSTRAINT "projects_user_id_foreign" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE
    "sections"
    ADD CONSTRAINT "sections_user_id_foreign" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE
    "comments"
    ADD CONSTRAINT "comments_user_id_foreign" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE
    "sections"
    ADD CONSTRAINT "sections_project_id_foreign" FOREIGN KEY ("project_id") REFERENCES "projects" ("id");
ALTER TABLE
    "tasks"
    ADD CONSTRAINT "tasks_project_id_foreign" FOREIGN KEY ("project_id") REFERENCES "projects" ("id");
ALTER TABLE
    "comments"
    ADD CONSTRAINT "comments_project_id_foreign" FOREIGN KEY ("project_id") REFERENCES "projects" ("id");
ALTER TABLE
    "projects"
    ADD CONSTRAINT "projects_parent_id_foreign" FOREIGN KEY ("parent_id") REFERENCES "projects" ("id");
ALTER TABLE
    "tasks"
    ADD CONSTRAINT "tasks_section_id_foreign" FOREIGN KEY ("section_id") REFERENCES "sections" ("id");
ALTER TABLE
    "comments"
    ADD CONSTRAINT "comments_parent_id_foreign" FOREIGN KEY ("parent_id") REFERENCES "comments" ("id");
ALTER TABLE
    "projects"
    ADD CONSTRAINT "projects_color_id_foreign" FOREIGN KEY ("color_id") REFERENCES "colors" ("id");
ALTER TABLE
    "labels"
    ADD CONSTRAINT "labels_color_id_foreign" FOREIGN KEY ("color_id") REFERENCES "colors" ("id");
