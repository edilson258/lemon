#![allow(dead_code, unused_variables)]

use crate::{diag::DiagGroup, source::Source};

use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
  thread,
};

pub struct Loader {
  cwd: PathBuf,
  pub files: Vec<Source>,
  std_path: PathBuf,
  pub diags: DiagGroup,
  cache: Arc<Mutex<HashMap<PathBuf, Source>>>,
}

impl Loader {
  pub fn new(cwd: PathBuf) -> Self {
    let files = Vec::new();
    let std_path = std::env::current_dir().unwrap().join("src/evaluator");
    let diags = DiagGroup::new();
    let cache = Arc::new(Mutex::new(HashMap::new()));
    Self { cwd, files, std_path, diags, cache }
  }

  pub fn load_module_str(&mut self, path: &str, curr_path: &PathBuf) -> Result<Source, String> {
    // remove ./
    let mut fmt_path = if path.ends_with(".lemon") { path.to_owned() } else { format!("{}.lemon", path) };
    fmt_path = self.remove_dot_dot(&fmt_path);
    let abs_path = if path.starts_with(".") { curr_path.join(fmt_path) } else { self.std_path.join(fmt_path) };
    self.load_module(&abs_path)
  }

  pub fn remove_dot_dot(&mut self, path: &str) -> String {
    return path.replace("::", "/");
  }

  pub fn load_module(&mut self, path: &Path) -> Result<Source, String> {
    let abs_path = self.resolve_path(path);

    if let Some(source) = self.get_from_cache(&abs_path) {
      return Ok(source);
    }
    match self.load_source(&abs_path) {
      Ok(source) => {
        self.cache_module(abs_path, source.clone());
        Ok(source)
      }
      Err(e) => Err(e),
    }
  }

  fn resolve_path(&self, path: &Path) -> PathBuf {
    if path.is_absolute() {
      path.to_path_buf()
    } else {
      let fmt_path = path.to_str().unwrap().trim_start_matches("./").to_owned();
      self.cwd.join(fmt_path)
    }
  }

  fn cache_module(&self, path: PathBuf, source: Source) {
    let mut cache = self.cache.lock().unwrap();
    cache.insert(path, source);
  }

  fn get_from_cache(&self, path: &Path) -> Option<Source> {
    let cache = self.cache.lock().unwrap();
    cache.get(path).cloned()
  }

  fn load_source(&self, path: &Path) -> Result<Source, String> {
    let raw = fs::read_to_string(path).map_err(|err| format!("failed to load module"))?;
    let filename = path.display().to_string();
    Ok(Source::new(raw.as_str(), filename.as_str()))
  }

  pub fn load_modules_in_parallel(&mut self, paths: Vec<PathBuf>) -> Vec<Result<Source, String>> {
    let cache = Arc::clone(&self.cache);
    let handles: Vec<_> = paths
      .into_iter()
      .map(|path| {
        let cache = Arc::clone(&cache);
        thread::spawn(move || {
          let abs_path = if path.is_absolute() { path } else { std::env::current_dir().unwrap().join(path) };
          let source = {
            let cache = cache.lock().unwrap();
            cache.get(&abs_path).cloned()
          };
          match source {
            Some(src) => Ok(src),
            None => {
              let raw = fs::read_to_string(&abs_path).map_err(|err| format!("failed to load module"))?;
              let filename = abs_path.display().to_string();
              let source = Source::new(raw.as_str(), filename.as_str());
              {
                let mut cache = cache.lock().unwrap();
                cache.insert(abs_path.clone(), source.clone());
              }
              Ok(source)
            }
          }
        })
      })
      .collect();
    handles.into_iter().map(|handle| handle.join().unwrap()).collect()
  }

  pub fn load_modules(&mut self, paths: Vec<PathBuf>) -> Vec<Result<Source, String>> {
    paths.into_iter().map(|path| self.load_module(&path)).collect()
  }

  pub fn laod_root(&mut self, paths: Vec<String>) -> Result<Source, String> {
    let path_text = format!("{}.lemon", paths.join("/").trim());
    let abs_path = self.std_path.join(path_text);
    self.load_module(&abs_path)
  }

  pub fn get_native(&self, text: &str) -> Option<Vec<String>> {
    if text.starts_with(".") {
      return None;
    }
    let parts: Vec<String> = text.split("::").map(|s| s.to_owned()).collect();

    if parts.is_empty() || !parts[0].eq("core") {
      return None;
    }
    return Some(parts);
  }
}
