use crate::serialization::{IdConstructor, IdReference};
use ray_tracing_core::hit_able::instancing;
use ray_tracing_core::hit_able::HitAble;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FlipNormals {
    pub id: IdConstructor,
    pub node: IdReference,
}

impl FlipNormals {
    pub fn from_hit_able(i: &instancing::FlipNormals) -> Result<FlipNormals, Box<dyn Error>> {
        Ok(FlipNormals {
            id: IdConstructor::Single(i.id),
            node: IdReference::Single(i.node.get_id()),
        })
    }

    pub fn to_hit_able(
        &self,
        index: usize,
        node: Arc<dyn HitAble>,
    ) -> Result<instancing::FlipNormals, Box<dyn Error>> {
        Ok(instancing::FlipNormals {
            id: self.id.get_id(index),
            node,
        })
    }
}

#[cfg(test)]
mod flip_normal_test {
    use super::*;
    use ray_tracing_core::hit_able::shape;
    use ray_tracing_core::material;
    use ray_tracing_core::types::Point3;

    #[test]
    fn flip_normal_test_from_flip_normal() {
        let s = shape::Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(material::NoMaterial::new()),
        );
        let s_id = s.id;
        let i = instancing::FlipNormals::new(Arc::new(s));
        let n = FlipNormals::from_hit_able(&i).unwrap();
        assert_eq!(n.node, IdReference::Single(s_id));
    }

    #[test]
    fn flip_normal_test_to_flip_normal() {
        let f = FlipNormals {
            id: IdConstructor::Single(0),
            node: IdReference::Single(1),
        };
        let _i = f
            .to_hit_able(
                0,
                Arc::new(shape::Sphere::new(
                    Point3::new(0.0, 0.0, 0.0),
                    1.0,
                    Arc::new(material::NoMaterial::new()),
                )),
            )
            .unwrap();
    }
}
