use anyhow::{Context, Result};
use pkg::ModPackage;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize)]
#[archive(check_bytes)] // Needed for validation
pub struct ArchivablePackage {
    pub name: String,
    pub version: String,
    pub files: Vec<(String, Vec<u8>)>,
}

impl From<&ModPackage> for ArchivablePackage {
    fn from(pkg: &ModPackage) -> Self {
        let mut files: Vec<_> = pkg
            .files
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        files.sort_by(|a, b| a.0.cmp(&b.0)); // Deterministic order

        Self {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
            files,
        }
    }
}

pub fn encode(pkg: &ModPackage) -> Result<Vec<u8>> {
    let archivable = ArchivablePackage::from(pkg);
    let bytes = rkyv::to_bytes::<_, 1024>(&archivable).context("Failed to serialize package")?;

    // Zstd compress
    let compressed = zstd::stream::encode_all(std::io::Cursor::new(bytes.as_slice()), 3)
        .context("Failed to compress package")?;

    Ok(compressed)
}
