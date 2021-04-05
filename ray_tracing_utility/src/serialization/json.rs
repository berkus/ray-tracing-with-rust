use crate::serialization::core::{DeserializeOptions, Scene};
use crate::view::ViewModel;
use ray_tracing_core::core;
use std::error::Error;

pub fn serialize_scene(scene: &core::Scene) -> std::result::Result<String, Box<dyn Error>> {
    let serialized_scene = Scene::from_scene(&scene)?;
    //match serde_json::to_string(&serialized_scene) {
    match serde_json::to_string_pretty(&serialized_scene) {
        Ok(s) => Ok(s),
        _ => Err("json serialization failed".into()),
    }
}

pub fn deserialize_scene(scene_str: &str) -> std::result::Result<core::Scene, Box<dyn Error>> {
    deserialize_scene_with_options(scene_str, &DeserializeOptions::default())
}

pub fn deserialize_scene_with_options(
    scene_str: &str,
    deserialize_options: &DeserializeOptions,
) -> std::result::Result<core::Scene, Box<dyn Error>> {
    let deserialized_scene: Scene = serde_json::from_str(scene_str)?;
    deserialized_scene.to_scene_with_options(deserialize_options)
}

pub fn serialize_view_model(view_model: &ViewModel) -> std::result::Result<String, Box<dyn Error>> {
    match serde_json::to_string_pretty(view_model) {
        Ok(s) => Ok(s),
        _ => Err("json serialization failed".into()),
    }
}

pub fn deserialize_view_model(
    view_model_str: &str,
) -> std::result::Result<ViewModel, Box<dyn Error>> {
    Ok(serde_json::from_str(view_model_str)?)
}

#[cfg(test)]
mod serialization_map {
    use super::*;
    use ray_tracing_core::test::TestSceneSimple;

    static TEST_SCENE_STR: &str = r#"{
        "configuration_id": 14,
        "camera_id": 12,
        "sky_id": 13,
        "root_node_id": 7,
        "objects": [
          { "ConstantTexture": { "id": 1, "color": [0.5, 0.1, 0.1] } },
          { "Lambertian": { "id": 2, "albedo": 1 } },
          { "Sphere": { "id": 3, "center": [0.0, 0.0, -1.0], "radius": 0.5, "material": 2 } },
          { "ConstantTexture": { "id": 4, "color": [0.1, 0.1, 0.1] } },
          { "Lambertian": { "id": 5, "albedo": 4 } },
          { "Sphere": { "id": 6, "center": [0.0, -100.5, -1.0], "radius": 100.0, "material": 5 } },
          { "Collection": { "id": 7, "object_id_list": [6, 3] } },
          { "Camera": { 
              "id": 12,
              "lower_left_corner": [-2.0, -1.0, -1.0],
              "horizontal": [4.0, 0.0, 0.0],
              "vertical": [0.0, 2.0, 0.0],
              "origin": [0.0, 0.0, 0.0],
              "lense_radius": 0.0,
              "time_from": 0.0,
              "time_to": 0.0
            }
          },
          { "Sky": { "id": 13, "nadir_color": [1.0, 1.0, 1.0], "zenith_color": [0.5, 0.7, 1.0] } },
          { "Configuration": { "id": 14, "maximum_depth": 50 } }
        ]
      }"#;

    #[test]
    fn serialize_test() {
        let scene = TestSceneSimple::new().scene;
        let json_string = serialize_scene(&scene).unwrap();
        print!("{}", json_string);
    }

    #[test]
    fn deserialize_test() {
        let json_string = TEST_SCENE_STR;
        let scene = deserialize_scene(json_string).unwrap();
        assert_eq!(scene.configuration.maximum_depth, 50);
    }
}
