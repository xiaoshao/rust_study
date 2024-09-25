#[cfg(test)]
#[allow(non_snake_case)]
mod test_mode {
    use std::fmt::Error;
    use datafusion::prelude::*;
    #[tokio::test]
    pub async fn read_kafka_csv() -> Result<(), Error> {
        let ctx = SessionContext::new();
        ctx.register_csv("input_csv", "test_data/csv_input/input.csv", CsvReadOptions::new().has_header(true)).await.unwrap();

        let df = ctx.sql("SELECT id, name FROM input_csv LIMIT 100").await.unwrap();

        // execute and print results
        df.show().await.unwrap();
        Ok(())
    }


}
