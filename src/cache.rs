#[derive(Copy, Clone)]
struct Entry {
  key: u64,
  count: u64,
  depth: i16,
}

pub struct Cache {
  entries: Box<[Entry]>,
  mask: usize,
}

impl Cache {
  pub fn new(size_bytes: usize) -> Result<Cache, String> {
    if size_bytes.count_ones() != 1 {
      return Err("Cache size must be 2^N".to_string());
    }

    if size_bytes < 1024 {
      return Err("Cache size must be at least 1024 bytes".to_string());
    }

    let size = size_bytes / 16;

    let vec = vec![
      Entry {
        key: 0,
        count: 0,
        depth: -1,
      };
      size
    ];

    Ok(Cache {
      entries: vec.into_boxed_slice(),
      mask: size - 1,
    })
  }

  pub fn probe(&self, key: u64, depth: usize) -> Option<u64> {
    let idx = (key as usize) & self.mask;
    let entry = unsafe { self.entries.get_unchecked(idx) };

    if entry.key == key && entry.depth == (depth as i16) {
      Some(entry.count)
    } else {
      None
    }
  }

  pub fn save(&mut self, key: u64, count: u64, depth: i16) {
    let idx = (key as usize) & self.mask;
    let entry = unsafe { self.entries.get_unchecked_mut(idx) };

    *entry = Entry { key, count, depth }
  }
}
