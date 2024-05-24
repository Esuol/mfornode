use std::path::Path;

#[cfg(unix)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
  std::os::unix::fs::symlink(from, to)?;
  ok(())
}