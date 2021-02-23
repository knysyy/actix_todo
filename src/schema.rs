table! {
    colors (id) {
        id -> Int4,
        name -> Varchar,
        color_code -> Varchar,
        order -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
        task_id -> Int4,
        parent_id -> Int4,
        title -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    labels (id) {
        id -> Int4,
        user_id -> Int4,
        color_id -> Int4,
        name -> Varchar,
        order -> Int4,
        is_favorite -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Int4,
        user_id -> Int4,
        color_id -> Int4,
        parent_id -> Int4,
        name -> Varchar,
        order -> Int4,
        is_inbox -> Bool,
        is_favorite -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sections (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
        name -> Varchar,
        order -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
        section_id -> Int4,
        parent_id -> Int4,
        title -> Varchar,
        description -> Varchar,
        due_date -> Nullable<Date>,
        due_datetime -> Nullable<Timestamp>,
        priority -> Int4,
        order -> Int4,
        is_completed -> Bool,
        is_favorite -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tasks_labels (task_id, label_id) {
        task_id -> Int4,
        label_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(comments -> projects (project_id));
joinable!(comments -> tasks (task_id));
joinable!(comments -> users (user_id));
joinable!(labels -> colors (color_id));
joinable!(labels -> users (user_id));
joinable!(projects -> colors (color_id));
joinable!(projects -> users (user_id));
joinable!(sections -> projects (project_id));
joinable!(sections -> users (user_id));
joinable!(tasks -> projects (project_id));
joinable!(tasks -> sections (section_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    colors,
    comments,
    labels,
    projects,
    sections,
    tasks,
    tasks_labels,
    users,
);
