use std::collections::{HashMap, HashSet};
use std::pin::Pin;
use std::sync::Arc;
use compact_str::CompactString;
use tokio::sync::RwLock;
use crate::file_meta::FileMeta;

#[derive(Debug, Clone)]
pub enum FsNode {
    File(FileMeta),
    Directory{
        name: CompactString,
        children: HashMap<CompactString, Arc<RwLock<FsNode>>>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum JsonFsNode {
    File(FileMeta),
    Directory{
        name: CompactString,
        children: Vec<JsonFsNode>,
    },
}


impl FsNode {
    pub fn name(&self) -> &str {
        match self {
            FsNode::File(meta) => meta.file_name.as_str(),
            FsNode::Directory{ name, ..} => name.as_str(),
        }
    }

    pub async fn get_node(&self, name: &str) -> Option<Arc<RwLock<FsNode>>> {
        match self {
            FsNode::File(_) => None,
            FsNode::Directory{children, ..} => children.get(name).cloned(),
        }
    }
    
    pub async fn to_json(&self) -> JsonFsNode {
        match self {
            FsNode::File(meta) => JsonFsNode::File(meta.clone()),
            FsNode::Directory{ name, children} => {
                let mut ser_children = Vec::new();
                for child in children.values() {
                    ser_children.push(Box::pin(child.read().await.to_json()).await);
                }
                JsonFsNode::Directory{
                    name: name.clone(),
                    children: ser_children,
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct FsTree {
    root: Arc<RwLock<FsNode>>,
}

impl FsTree {
    pub fn new(root: FsNode) -> Self {
        Self {
            root: Arc::new(RwLock::new(root)),
        }
    }
    
    pub async fn get_node(&self, path: &[CompactString]) -> Option<Arc<RwLock<FsNode>>> {
        let mut current = self.root.clone();
        for name in path {
            let owner = current;
            current = owner.read().await.get_node(name.as_str()).await?;
        }
        Some(current)
    }
    
    pub async fn insert_node(&mut self, path: &[CompactString], node: FsNode) -> bool {
        let mut current = self.root.clone();
        for name in path {
            let owner = current;
            let next = owner.read().await.get_node(name.as_str()).await;
            current = match next {
                Some(next) => next,
                None => {
                    let new_dir = Arc::new(RwLock::new(FsNode::Directory{
                        name: name.clone(),
                        children: HashMap::new(),
                    }));
                    let mut owner = owner.write().await;
                    match &mut *owner {
                        FsNode::File(_) => return false,
                        FsNode::Directory{children, ..} => {
                            children.insert(name.clone(), new_dir.clone());
                        }
                    }
                    new_dir
                },
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_meta::FileMeta;
    use std::time::SystemTime;
    use compact_str::CompactString;
    use smallvec::SmallVec;

    #[tokio::test]
    async fn test_fs_tree() {
        let root = FsNode::Directory{
            name: CompactString::new("root"),
            children: HashMap::new(),
        };
        let mut tree = FsTree::new(root);
        assert!(tree.insert_node(&[CompactString::new("dir1")], FsNode::Directory{
            name: CompactString::new("dir1"),
            children: HashMap::new(),
        }).await);
        assert!(tree.insert_node(&[CompactString::new("dir1"), CompactString::new("dir2")], FsNode::Directory{
            name: CompactString::new("dir2"),
            children: HashMap::new(),
        }).await);
        assert!(tree.insert_node(&[CompactString::new("dir1"), CompactString::new("dir2"), CompactString::new("dir3")], FsNode::Directory{
            name: CompactString::new("dir3"),
            children: HashMap::new(),
        }).await);
        assert!(tree.insert_node(&[CompactString::new("dir1"), CompactString::new("dir2"), CompactString::new("dir3"), CompactString::new("file1")], FsNode::File(FileMeta{
            categories: SmallVec::new(),
            file_name: CompactString::new("file1"),
            file_size: 100,
            upload_timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            path: "/dir1/dir2/dir3/file1".to_string(),
        })).await);
        let node = tree.get_node(&[CompactString::new("dir1"), CompactString::new("dir2"), CompactString::new("dir3"), CompactString::new("file1")]).await.unwrap();
        assert_eq!(node.read().await.name(), "file1");
    }
}