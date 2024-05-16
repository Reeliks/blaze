use bson::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Write};

pub struct Fs {
    path: String,
    tables: HashMap<String, Vec<Document>>,
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

    fn extract_tables(
        document: &Document,
    ) -> Result<HashMap<String, Vec<Document>>, bson::de::Error> {
        let mut tables: HashMap<String, Vec<Document>> = HashMap::new();

        for (key, value) in document {
            if let Bson::Array(subdoc) = value {
                let rows: Vec<Document> = subdoc
                    .iter()
                    .filter_map(|bson| {
                        if let Bson::Document(doc) = bson {
                            Some(doc.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                let name = key.to_string();

                tables.insert(name, rows);
            }
        }

        Ok(tables)
    }

    pub fn db_write(mut self, table_name: String, data: Document) -> Result<(), de::Error> {
        let mut doc = Document::new();

        for (key, value) in &mut self.tables {
            if *key == table_name {
                value.push(data.clone());
            } else {
                doc.insert(table_name.clone(), Bson::Array(vec![Bson::from(data.clone())]));
            }
        }

        for (key, value) in self.tables {
            let bson_array: Vec<Bson> = value.iter().map(Bson::from).collect();
            doc.insert(key.to_string(), Bson::Array(bson_array));
        }

        if doc.is_empty() {
            doc.insert(table_name, Bson::Array(vec![Bson::from(data.clone())]));
        }

        let mut buf = Vec::new();
        doc.to_writer(&mut buf).unwrap();

        let mut file = File::create(self.path)?;
        file.write_all(&buf)?;

        let doc = Document::from_reader(&mut Cursor::new(&buf[..]))?;
        println!("Deserialized: {:#?}", doc);

        Ok(())
    }
}
