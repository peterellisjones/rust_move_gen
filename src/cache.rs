#[derive(Copy, Clone)]
struct Entry {
  key: u64,
  count: u32,
  depth: i16,
}

pub struct Cache {
  entries: Box<[Entry]>,
  mask: usize,
}

impl Cache {
  pub fn new(size: usize) -> Result<Cache, String> {
    if size.count_ones() != 1 {
      return Err("Cache size must be 2^N".to_string());
    }

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

  pub fn probe(&self, key: u64, depth: usize) -> Option<usize> {
    let idx = (key as usize) & self.mask;
    let entry = self.entries[idx];

    if entry.key == key && entry.depth == (depth as i16) {
      Some(entry.count as usize)
    } else {
      None
    }
  }

  pub fn save(&mut self, key: u64, count: usize, depth: i16) {
    let idx = (key as usize) & self.mask;
    self.entries[idx] = Entry {
      key: key,
      count: count as u32,
      depth: depth,
    }
  }
}
