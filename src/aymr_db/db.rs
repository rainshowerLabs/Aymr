use crate::btreemap::db::AymrBtreeMap;

#[derive(Debug)]
struct AymrDB<K,V> {
	#[feature(btreemap)]
	db: AymrBtreeMap<K, V>,
}
