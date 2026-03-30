use crate::configs::db::DBPool;
use crate::models::user::User;

pub async fn find_user_with_role_by_email(
    pool: &DBPool,
    email: &str
) -> sqlx::Result<Option<(User, Option<String>)>> {
    let user = sqlx
        ::query_as::<_, User>(
            r#"
        SELECT u.id, u.name, u.email, u.password, u.photo, u.phone, u.created_at, u.updated_at
        FROM users u
        WHERE u.email = $1
        "#
        )
        .bind(email)
        .fetch_optional(pool).await?;

    if let Some(user) = user {
        let role_name: Option<String> = sqlx
            ::query_scalar(
                r#"
        SELECT r.name
        FROM roles r
        JOIN user_role ur ON ur.role_id = r.id
        WHERE ur.user_id = $1
        LIMIT 1
        "#
            )
            .bind(user.id)
            .fetch_optional(pool).await?;

        Ok(Some((user, role_name)))
    } else {
        Ok(None)
    }
}
