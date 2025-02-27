// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Int4,
        unit -> Nullable<Varchar>,
        label -> Nullable<Varchar>,
    }
}

diesel::table! {
    instructions (id) {
        id -> Int4,
        instruction -> Varchar,
        display_order -> Int4,
        recipe_id -> Int4,
    }
}

diesel::table! {
    recipe_ingredients (id) {
        id -> Int4,
        amount -> Nullable<Float8>,
        recipe_id -> Int4,
        ingredient_id -> Nullable<Int4>,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
        servings -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(recipe_ingredients -> ingredients (ingredient_id));
diesel::joinable!(instructions -> recipes (recipe_id));
diesel::joinable!(recipe_ingredients -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    ingredients,
    instructions,
    recipe_ingredients,
    recipes,
    users,
);
