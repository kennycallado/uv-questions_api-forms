// @generated automatically by Diesel CLI.

diesel::table! {
    form_questions (id) {
        id -> Int4,
        form_id -> Int4,
        question_id -> Int4,
    }
}

diesel::table! {
    forms (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        q_type -> Varchar,
        question -> Varchar,
    }
}

diesel::joinable!(form_questions -> forms (form_id));
diesel::joinable!(form_questions -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(form_questions, forms, questions,);
