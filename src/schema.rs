table! {
    articles (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        author -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        favorites_count -> Int4,
    }
}

table! {
    article_tag (tag, article) {
        tag -> Int4,
        article -> Int4,
    }
}

table! {
    favorites (user, article) {
        user -> Int4,
        article -> Int4,
    }
}

table! {
    follows (follower, followed) {
        follower -> Int4,
        followed -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(article_tag -> articles (article));
joinable!(article_tag -> tags (tag));
joinable!(articles -> users (author));
joinable!(favorites -> articles (article));
joinable!(favorites -> users (user));

allow_tables_to_appear_in_same_query!(
    articles,
    article_tag,
    favorites,
    follows,
    tags,
    users,
);
