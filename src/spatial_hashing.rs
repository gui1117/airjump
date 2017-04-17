extern crate fnv;

use physics::Body;

pub struct SpatialHashing {
    map: fnv::FnvHashMap<[i32; 2], Vec<(usize, Body)>>,
    unit: f64
}

impl SpatialHashing {
    pub fn new(unit: f64, bodies: &Vec<Body>) -> SpatialHashing {
        let mut map = fnv::FnvHashMap::default();
        for (id, body) in bodies.iter().enumerate() {
            for cell in body.cells(unit) {
                let mut vec = map.entry(cell).or_insert(vec!());
                vec.push((id,body.clone()));
            }
        }
        SpatialHashing {
            map: map,
            unit: unit,
        }
    }
    pub fn get_on_body(&self, body: &Body) -> Vec<Body> {
        let mut res = Vec::new();
        let mut ids = fnv::FnvHashSet::default();

        for cell in body.cells(self.unit) {
            if let Some(vec) = self.map.get(&cell) {
                for &(id, ref body) in vec.iter() {
                    if !ids.contains(&id) {
                        ids.insert(id);
                        res.push(body.clone());
                    }
                }
            }
        }
        res
    }
}
