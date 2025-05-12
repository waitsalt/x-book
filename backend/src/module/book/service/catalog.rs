use axum::extract::Path;

use crate::{
    module::{
        book::model::BookCatalogResponse,
        chapter::model::Chapter,
        roll::model::{Roll, RollResponse},
    },
    util::{AppResult, database::database_connect, response::AppResponse},
};

pub async fn catalog(Path(book_id): Path<i64>) -> AppResult<BookCatalogResponse> {
    let roll_list_sql = "
        select *
        from \"roll\"
        where book_id = $1
        order by roll_id asc
        ";
    let pool = database_connect();
    let roll_list: Vec<Roll> = sqlx::query_as(roll_list_sql)
        .bind(book_id)
        .fetch_all(pool)
        .await
        .unwrap();
    let mut book_catalog_response = BookCatalogResponse::new();
    for roll in roll_list {
        let chapter_list_sql = "
            select *
            from \"chapter\"
            where roll_id = $1
            order by chapter_id asc
            ";
        let chapter_list: Vec<Chapter> = sqlx::query_as(chapter_list_sql)
            .bind(roll.roll_id)
            .fetch_all(pool)
            .await
            .unwrap();
        let roll_response = RollResponse::new(roll, chapter_list);
        book_catalog_response.roll_list.push(roll_response);
    }
    Ok(AppResponse::success(Some(book_catalog_response)))
}
