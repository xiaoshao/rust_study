use std::fs::File;
use std::sync::Arc;
use parquet2::{
    error::Error,
    metadata::SchemaDescriptor,
    schema::types::{ParquetType, PhysicalType},
    write::{write_metadata_sidecar, FileWriter, Version, WriteOptions},
};
use parquet2::metadata::ColumnChunkMetaData;
use parquet2::write::RowGroupIter;

pub fn parquetDemo() -> Result<(), Error> {
// say we have 10 files with the same schema:
    let schema = SchemaDescriptor::new(
        "schema".to_string(),
        vec![ParquetType::from_physical(
            "c1".to_string(),
            PhysicalType::Int32,
        )],
    );

    // we can collect their metadata after they are written
    let mut metadatas = vec![];
    for i in 0..10 {
        let relative_path = format!("part-{i}.parquet");
        let writer = File::create(&relative_path).unwrap();
        let mut writer = FileWriter::new(
            writer,
            schema.clone(),
            WriteOptions {
                write_statistics: true,
                version: Version::V2,
            },
            None,
        );


        // // we write row groups to it
        // RowGroupIter
        // writer.write(row_group);

        // and the footer
        writer.end(None)?;
        let (_, mut metadata) = writer.into_inner_and_metadata();

        // once done, we write the relative path to the column chunks
        metadata.row_groups.iter_mut().for_each(|row_group| {
            row_group
                .columns
                .iter_mut()
                .for_each(|column| column.file_path = Some(relative_path.clone()))
        });
        // and collect the metadata
        metadatas.push(metadata);
    }

    // we can then merge their row groups
    let first = metadatas.pop().unwrap();
    let sidecar = metadatas.into_iter().fold(first, |mut acc, metadata| {
        acc.row_groups.extend(metadata.row_groups.into_iter());
        acc
    });

    // and write the metadata on a separate file
    let mut writer = File::create(format!("part-100-1.parquet")).unwrap();
    write_metadata_sidecar(&mut writer, &sidecar)?;

    Ok(())
}