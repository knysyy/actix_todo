alter table labels
drop
constraint labels_color_id_foreign;

alter table projects
drop
constraint projects_color_id_foreign;

alter table comments
drop
constraint comments_parent_id_foreign;

alter table tasks
drop
constraint tasks_section_id_foreign;

alter table projects
drop
constraint projects_parent_id_foreign;

alter table comments
drop
constraint comments_project_id_foreign;

alter table tasks
drop
constraint tasks_project_id_foreign;

alter table sections
drop
constraint sections_project_id_foreign;

alter table comments
drop
constraint comments_user_id_foreign;

alter table projects
drop
constraint projects_user_id_foreign;

alter table labels
drop
constraint labels_user_id_foreign;

alter table tasks
drop
constraint tasks_user_id_foreign;

alter table comments
drop
constraint comments_task_id_foreign;

drop table labels;

drop table comments;

drop table tasks;

drop table sections;

drop table projects;

drop table colors;

drop table users;

drop table tasks_labels;
