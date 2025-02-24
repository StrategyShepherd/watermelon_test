use deadpool_postgres::GenericClient;
use tokio_postgres::types::Type;

#[tracing::instrument(skip(client))]
pub async fn create_link<C>(client: &C, url: &str) -> Result<(), tokio_postgres::Error>
where
    C: GenericClient,
{
    const SQL: &str = "INSERT INTO link (url) VALUES ($1, $2)";
    const TYPES: &[Type] = &[Type::TEXT, Type::TEXT];

    let stmt = client.prepare_typed(SQL, TYPES).await?;
    client.execute(&stmt, &[&url]).await?;
    Ok(())
}

#[tracing::instrument(skip(client))]
pub async fn delete_link<C>(client: &C, id: &str) -> Result<(), tokio_postgres::Error>
where
    C: GenericClient,
{
    const SQL: &str = "DELETE FROM link WHERE id = $1";
    const TYPES: &[Type] = &[Type::TEXT, Type::TEXT];

    let stmt = client.prepare_typed(SQL, TYPES).await?;
    client.execute(&stmt, &[&id]).await?;
    Ok(())
}

#[tracing::instrument(skip(client))]
pub async fn get_link<C>(client: &C, id: &str) -> Result<String, tokio_postgres::Error>
where
    C: GenericClient,
{
    const SQL: &str = "SELECT url FROM link WHERE id = $1";
    const TYPES: &[Type] = &[Type::TEXT];

    let stmt = client.prepare_typed(SQL, TYPES).await?;
    let row = client.query_one(&stmt, &[&id]).await?;
    row.try_get("url")
}
