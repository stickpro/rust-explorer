use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE blocks (
                    height integer PRIMARY KEY NOT NULL,
                    hash varchar(65) NOT NULL,
                    block_created_at timestamp NOT NULL,
                    size integer NOT NULL,
                    weight integer NOT NULL,
                    tx_count integer NOT NULL,
                    coinbase_raw text,
                    difficulty bigint NOT NULL,
                    pool_id integer DEFAULT -1,
                    fees double precision NOT NULL,
                    fee_span json NOT NULL,
                    median_fee double precision NOT NULL
                )",
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE `blocks`")
            .await?;

        Ok(())
    }
}

