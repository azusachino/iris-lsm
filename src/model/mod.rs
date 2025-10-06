//! memtable -> wal -> manifest -> sst
//!
use anyhow::Result;

pub trait KVOps<K, V> {
    fn put(&mut self, k: K, v: V);

    fn delete(&mut self, k: K);

    fn get(&self, k: K) -> Result<V>;

    // range, aka start -> end
    fn scan(&self, range: [K; 2]);
}

#[derive(Debug, Default)]
pub struct MemTable {}

pub struct WAL {}

pub struct Manifest {}

#[derive(Debug)]
pub struct SST {}
