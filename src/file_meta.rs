use std::sync::Arc;
use compact_str::CompactString;
use kanau::processor::Processor;
use smallvec::SmallVec;
use tantivy::query::QueryParser;
use tantivy::schema::document::{DeserializeError, DocumentDeserialize, DocumentDeserializer};
use tantivy::schema::Field;
use tantivy::{doc, IndexWriter};
use thiserror::Error;
use tokio::sync::{OnceCell};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileMeta {
    pub categories: SmallVec<[CompactString; 5]>,
    pub file_name: CompactString,
    pub file_size: u64,
    pub upload_timestamp: u64,
    pub path: String,
}

pub const NAME_FIELD: OnceCell<Field> = OnceCell::const_new();
pub const META_FIELD: OnceCell<Field> = OnceCell::const_new();

#[derive(Debug, Error)]
enum InitializeSearchEngineError {
    #[error("Failed to create index: {0}")]
    TantivyError(#[from] tantivy::TantivyError),
    
    #[error("Failed to serialize: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub fn initialize_search_engine(items: Box<[FileMeta]>) -> Result<tantivy::Index, InitializeSearchEngineError> {
    let mut schema_builder = tantivy::schema::Schema::builder();
    schema_builder.add_text_field("name", tantivy::schema::TEXT | tantivy::schema::STORED);
    schema_builder.add_text_field("meta", tantivy::schema::STORED);
    let schema = schema_builder.build();
    NAME_FIELD.set(schema.get_field("name")?).unwrap();
    META_FIELD.set(schema.get_field("meta")?).unwrap();

    let index = tantivy::Index::create_in_ram(schema);
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;
    for item in items {
        let doc = doc! {
            NAME_FIELD.get().unwrap().clone() => item.file_name.as_str(),
            META_FIELD.get().unwrap().clone() => serde_json::to_string(&item)?,
        };
        index_writer.add_document(doc)?;
    }
    index_writer.commit()?;
    Ok(index)
}

impl DocumentDeserialize for FileMeta {
    fn deserialize<'de, D>(mut deserializer: D) -> Result<Self, DeserializeError>
    where
        D: DocumentDeserializer<'de>
    {
        // find "meta" field
        let mut meta_field: Option<String> = None;
        while let Some((field, value)) = deserializer.next_field()? {
            if &field == META_FIELD.get().unwrap() {
                meta_field = Some(value);
                break;
            }
        }

        match meta_field {
            Some(meta_field) => {
                let meta: FileMeta = serde_json::from_str(&meta_field).map_err(|e| DeserializeError::Custom(e.to_string()))?;
                Ok(meta)
            }
            None => Err(DeserializeError::Custom("Failed to find meta field".to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchEngine{
    engine: Arc<tantivy::Index>,
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Failed to serialize: {0}")]
    DeserializeError(#[from] rkyv::rancor::Error),

    #[error("Failed to search: {0}")]
    SearchError(#[from] tantivy::TantivyError),
    
    #[error("Failed to parse query: {0}")]
    QueryParseError(#[from] tantivy::query::QueryParserError),
}

impl SearchEngine {
    pub fn new(engine: tantivy::Index) -> Self {
        Self {
            engine: Arc::new(engine),
        }
    }

    pub fn search_by_names(&self, query: &[CompactString]) -> Result<Box<[FileMeta]>, SearchError> {
        let reader = self.engine.reader()?;
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&self.engine, vec![NAME_FIELD.get().unwrap().clone()]);
        let query = query_parser.parse_query(query.join(" ").as_str())?;
        let result: Box<[FileMeta]> = searcher
            .search(&query, &tantivy::collector::TopDocs::with_limit(10))?
            .into_iter()
            .map(|(_, addr)| addr)
            .filter_map(|addr| searcher.doc(addr).ok())
            .collect();
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct SearchFileRequest {
    pub user_request: CompactString,
    pub ai_extended_request: Vec<CompactString>,
}

impl Processor<SearchFileRequest, Result<Box<[FileMeta]>, SearchError>> for SearchEngine {
    async fn process(&self, request: SearchFileRequest) -> Result<Box<[FileMeta]>, SearchError> {
        let ai_words: Vec<_> = request
            .ai_extended_request
            .into_iter()
            .take(3)
            .collect();
        let user_words: Vec<_> = request
            .user_request
            .split_whitespace()
            .take(3)
            .map(|s| CompactString::new(s))
            .collect();
        let words = [ai_words, user_words].concat();
        self.search_by_names(&words)
    }
}