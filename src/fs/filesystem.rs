use bson::*;
use std::fs::File;
use std::io::{Cursor, Read, Write};

pub struct Fs {
    path: String,
    pub tables: Vec<Tables>,
}

#[derive(Debug)]
pub struct Tables {
    pub name: String,
    pub rows: Vec<Document>,
}

impl Fs {
    pub fn db_open(path: String) -> Result<Fs, de::Error> {
        let mut file: File = File::open(&path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let document = bson::Document::from_reader(&mut Cursor::new(&buf[..]))?;
        let tables = Self::extract_tables(&document)?;

        Ok(Fs { path, tables })
    }

    fn extract_tables(document: &Document) -> Result<Vec<Tables>, bson::de::Error> {
        let mut tables = Vec::new();

        for (key, value) in document {
            if let Bson::Document(subdoc) = value {
                let rows = vec![subdoc.clone()];
                let name = key.to_string();

                let table = Tables { name, rows };
                tables.push(table);
            }
        }

        Ok(tables)
    }

    pub fn db_write(self, table_name: String, data: Document) -> Result<(), de::Error> {
        let mut doc = Document::new();

        for table in self.tables {
            let mut test: Vec<Document> = table.rows.clone();
            if table.name == table_name {
                test.push(data.clone());
            }

            let bson_array: Vec<Bson> = test.into_iter().map(Bson::Document).collect();
            doc.insert(table.name, Bson::Array(bson_array));
        }

        if doc.is_empty() {
            doc.insert(table_name, Bson::Document(data));
        }

        let mut buf = Vec::new();
        doc.to_writer(&mut buf).unwrap();

        let mut file = File::open(self.path)?;
        file.write_all(&buf)?;

        let doc = Document::from_reader(&mut Cursor::new(&buf[..]))?;
        println!("Deserialized: {:#?}", doc);

        Ok(())
    }
}
