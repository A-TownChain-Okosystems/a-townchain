//! Media-library metadata, collections, search and deterministic ordering.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    TitleAscending,
    TitleDescending,
    Newest,
    Oldest,
    SizeAscending,
    SizeDescending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MimeType {
    Audio,
    Video,
    Image,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaMetadata {
    pub id: u64,
    pub uri: String,
    pub title: String,
    pub mime: MimeType,
    pub size_bytes: u64,
    pub modified_unix_ms: u64,
    pub content_hash: Option<[u8; 32]>,
    pub duration_ms: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Default)]
pub struct MediaCatalog {
    items: Vec<MediaMetadata>,
    favorites: Vec<u64>,
    albums: Vec<(String, Vec<u64>)>,
}

impl MediaCatalog {
    pub fn upsert(&mut self, item: MediaMetadata) {
        self.items.retain(|x| x.id != item.id);
        self.items.push(item);
    }
    pub fn items(&self) -> &[MediaMetadata] {
        &self.items
    }
    pub fn set_favorite(&mut self, id: u64, favorite: bool) {
        self.favorites.retain(|x| *x != id);
        if favorite && self.items.iter().any(|x| x.id == id) {
            self.favorites.push(id);
        }
    }
    pub fn is_favorite(&self, id: u64) -> bool {
        self.favorites.contains(&id)
    }
    pub fn add_to_album(&mut self, name: impl Into<String>, id: u64) {
        let name = name.into();
        if let Some((_, ids)) = self.albums.iter_mut().find(|(n, _)| *n == name) {
            if !ids.contains(&id) {
                ids.push(id);
            }
        } else {
            self.albums.push((name, vec![id]));
        }
    }
    pub fn search(&self, query: &str) -> Vec<&MediaMetadata> {
        let q = query.to_ascii_lowercase();
        self.items
            .iter()
            .filter(|x| {
                x.title.to_ascii_lowercase().contains(&q) || x.uri.to_ascii_lowercase().contains(&q)
            })
            .collect()
    }
    pub fn sorted(&self, order: SortOrder) -> Vec<&MediaMetadata> {
        let mut out: Vec<_> = self.items.iter().collect();
        match order {
            SortOrder::TitleAscending => out.sort_by(|a, b| a.title.cmp(&b.title)),
            SortOrder::TitleDescending => out.sort_by(|a, b| b.title.cmp(&a.title)),
            SortOrder::Newest => out.sort_by_key(|x| std::cmp::Reverse(x.modified_unix_ms)),
            SortOrder::Oldest => out.sort_by_key(|x| x.modified_unix_ms),
            SortOrder::SizeAscending => out.sort_by_key(|x| x.size_bytes),
            SortOrder::SizeDescending => out.sort_by_key(|x| std::cmp::Reverse(x.size_bytes)),
        };
        out
    }
    pub fn duplicate_ids(&self) -> Vec<Vec<u64>> {
        let mut groups = Vec::new();
        for item in &self.items {
            if let Some(hash) = item.content_hash {
                let mut ids = vec![item.id];
                for other in &self.items {
                    if other.id != item.id
                        && other.content_hash == Some(hash)
                        && !ids.contains(&other.id)
                    {
                        ids.push(other.id);
                    }
                }
                if ids.len() > 1
                    && !groups
                        .iter()
                        .any(|g: &Vec<u64>| g.iter().any(|id| ids.contains(id)))
                {
                    groups.push(ids);
                }
            }
        }
        groups
    }
}

pub fn mime_from_extension(path: &str) -> MimeType {
    match path
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "mp3" | "flac" | "opus" | "aac" => MimeType::Audio,
        "mp4" | "mkv" | "webm" | "mov" => MimeType::Video,
        "jpg" | "jpeg" | "png" | "webp" | "avif" => MimeType::Image,
        _ => MimeType::Unknown,
    }
}
