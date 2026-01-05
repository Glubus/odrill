use super::encode::ArchivablePackage;
use anyhow::{Context, Result};
use pkg::ModPackage;
use rkyv::Deserialize;

pub fn decode(compressed: &[u8]) -> Result<ModPackage> {
    // Zstd decompress
    let decompressed = zstd::stream::decode_all(std::io::Cursor::new(compressed))
        .context("Failed to decompress package")?;

    // Rkyv deserialize
    let archived = unsafe { rkyv::archived_root::<ArchivablePackage>(&decompressed) };

    // Convert back to ModPackage
    let deserialized: ArchivablePackage = archived.deserialize(&mut rkyv::Infallible).unwrap();

    let mut pkg = ModPackage::new(deserialized.name, deserialized.version);
    for (path, content) in deserialized.files {
        pkg.add_file(path, content);
    }

    Ok(pkg)
}
