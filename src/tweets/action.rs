use diesel::prelude::*;

use crate::tweets::models::{NewTweet, Tweet};

type DbError = Box<dyn std::error::Error + Send + Sync>;

// Functions define
pub fn add_a_tweet(_message: &str, conn: &PgConnection) -> Result<Tweet, DbError> {
    use crate::schema::tweets::dsl::*;

    let new_tweet = NewTweet {
        message: _message,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(tweets)
        .values(&new_tweet)
        .get_result(conn)?;
    Ok(res)
}

pub fn find_all(conn: &PgConnection) -> Result<Vec<Tweet>, DbError> {
    use crate::schema::tweets::dsl::*;

    let items = tweets.load::<Tweet>(conn)?;
    Ok(items)
}

pub fn find_by_id(tweet_id: i32, conn: &PgConnection) -> Result<Option<Tweet>, DbError> {
    use crate::schema::tweets::dsl::*;

    let tweet = tweets
        .filter(id.eq(tweet_id))
        .first::<Tweet>(conn)
        .optional()?;

    Ok(tweet)
}

pub fn update_tweet(
    tweet_id: i32,
    _message: String,
    conn: &PgConnection,
) -> Result<Tweet, DbError> {
    use crate::schema::tweets::dsl::*;

    let tweet = diesel::update(tweets.find(tweet_id))
        .set(message.eq(_message))
        .get_result::<Tweet>(conn)?;
    Ok(tweet)
}

pub fn delete_tweet(tweet_id: i32, conn: &PgConnection) -> Result<usize, DbError> {
    use crate::schema::tweets::dsl::*;

    let count = diesel::delete(tweets.find(tweet_id)).execute(conn)?;
    Ok(count)
}
