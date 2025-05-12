use once_cell::sync::Lazy;
use sqlx::{Pool, Postgres};

use crate::{
    module::model::SqlResult,
    util::{database::database_connect, error::AppError},
};

use super::model::User;

static POOL: Lazy<&Pool<Postgres>> = Lazy::new(|| database_connect());

pub async fn user_info_get_by_id(id: &i64) -> SqlResult<User> {
    let sql = "
    select
        *
    from
        \"user\"
    where
        user_id = $1";
    let res: Option<User> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(*POOL)
        .await
        .unwrap();
    match res {
        Some(user) => Ok(user),
        None => Err(AppError::UserNotExist),
    }
}

pub async fn user_search_by_name(name: &str) -> SqlResult<Vec<User>> {
    let sql = "
    select
        *
    from
        \"user\"
    where
        user_name like '%$1%'";

    let users: Vec<User> = sqlx::query_as(sql)
        .bind(name)
        .fetch_all(*POOL)
        .await
        .unwrap();
    Ok(users)
}

pub async fn user_name_is_exist(name: &str) -> SqlResult<()> {
    let sql = "
    select
        user_id
    from
        \"user\"
    where
        user_name = $1";
    let affected_row = sqlx::query(sql)
        .bind(name)
        .execute(*POOL)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserNameExist);
    }
    Ok(())
}

pub async fn user_email_is_exist(email: &str) -> SqlResult<()> {
    let sql = "
        select
            user_id
        from
            \"user\"
        where
            user_email = $1";
    let affected_row = sqlx::query(sql)
        .bind(email)
        .execute(*POOL)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserEmailExist);
    }
    Ok(())
}

pub async fn user_create(
    name: &str,
    password: &str,
    email: &str,
    avatar_url: &str,
) -> SqlResult<()> {
    let sql = "
    insert into
        \"user\" (user_name, user_password, user_email, user_avatar_url)
    values
        ($1,$2,$3,$4)";
    let _affected_row = sqlx::query(sql)
        .bind(name)
        .bind(password)
        .bind(email)
        .bind(avatar_url)
        .execute(*POOL)
        .await
        .unwrap()
        // .map_err(|_| AppError::UserCreateFailure)?
        .rows_affected();
    Ok(())
}

pub async fn user_delete(id: &i64) -> SqlResult<()> {
    let sql = "
    upadte
        \"user\"
    set
        user_status = 2
    where
        user_id = $1";
    let _ = sqlx::query(sql).bind(id).execute(*POOL).await.unwrap();
    Ok(())
}

pub async fn update_avatar_url(id: &i64, avatar_url: &str) -> SqlResult<()> {
    let sql = "
    update
        \"user\"
    set
        user_avatar_url = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(avatar_url)
        .bind(id)
        .execute(*POOL)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_email(id: &i64, email: &str) -> SqlResult<()> {
    let sql = "
    update
        \"user\"
    set
        user_email = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(email)
        .bind(id)
        .execute(*POOL)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_password(id: &i64, password: &str) -> SqlResult<()> {
    let sql = "
    update
        \"user\"
    set
        user_password = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(password)
        .bind(id)
        .execute(*POOL)
        .await
        .unwrap();
    Ok(())
}
