use futures::future;
use sqlx::MySqlPool;

use crate::typecho::{self, schema::GenericName};

pub async fn get_all_posts(pool: &MySqlPool) -> Result<Vec<typecho::model::Post>, sqlx::Error> {
    let posts: Vec<typecho::schema::Post> = sqlx::query_as(
        r#"
        SELECT
            cid,
            title,
            slug,
            text,
            created,
            modified
        FROM
            typecho_contents
        WHERE
            type = 'post' AND status = 'publish'
        ORDER BY
            created DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let posts_futures: Vec<_> = posts
        .into_iter()
        .map(|post| async move {
            let categories = fetch_categories_by_cid(pool, post.cid).await.unwrap();
            let tags = fetch_tags_by_cid(pool, post.cid).await.unwrap();
            typecho::model::Post {
                cid: post.cid,
                title: post.title,
                slug: post.slug,
                text: post.text,
                created: post.created,
                modified: post.modified,
                object_id: post.cid.to_string(),
                categories,
                tags,
            }
        })
        .collect();

    let posts: Vec<_> = future::join_all(posts_futures).await;

    Ok(posts)
}

pub async fn fetch_categories_by_cid(
    pool: &MySqlPool,
    cid: u32,
) -> Result<Vec<String>, sqlx::Error> {
    let categories: Vec<GenericName> = sqlx::query_as(
        r#"
        SELECT
            m.name
        FROM
            typecho_metas m
        JOIN
            typecho_relationships r
        ON
            m.mid = r.mid
        JOIN
            typecho_contents c
        ON
            r.cid = c.cid
        WHERE
            c.type = 'post' AND c.status = 'publish' AND m.type = 'category' AND c.cid = ?
        ORDER BY
            c.created DESC
        "#,
    )
    .bind(cid)
    .fetch_all(pool)
    .await?;

    Ok(categories
        .iter()
        .map(|category| category.name.clone())
        .collect())
}

pub async fn fetch_tags_by_cid(pool: &MySqlPool, cid: u32) -> Result<Vec<String>, sqlx::Error> {
    let tags: Vec<GenericName> = sqlx::query_as(
        r#"
        SELECT
            m.name
        FROM
            typecho_metas m
        JOIN
            typecho_relationships r
        ON
            m.mid = r.mid
        JOIN
            typecho_contents c
        ON
            r.cid = c.cid
        WHERE
            c.type = 'post' AND c.status = 'publish' AND m.type = 'tag' AND c.cid = ?
        ORDER BY
            c.created DESC
        "#,
    )
    .bind(cid)
    .fetch_all(pool)
    .await?;

    Ok(tags.iter().map(|tag| tag.name.clone()).collect())
}
