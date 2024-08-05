use mod3d_gltf::{Error, Gltf};
#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;

#[cfg(feature = "serde_json")]
#[test]
fn simple() -> Result<(), Error> {
    const JSON: &str = r##"
{
    "asset" : {
        "generator" : "Khronos glTF Blender I/O v1.4.40",
        "version" : "2.0"
    },
    "scene" : 0,
    "scenes" : [
        {
            "name" : "Scene",
            "nodes" : [
                0,
                1,
                2
            ]
        }
    ],
    "nodes" : [
        {
            "mesh" : 0,
            "name" : "Cube"
        },
        {
            "name" : "Light",
            "rotation" : [
                0.16907575726509094,
                0.7558803558349609,
                -0.27217137813568115,
                0.570947527885437
            ],
            "translation" : [
                4.076245307922363,
                5.903861999511719,
                -1.0054539442062378
            ]
        },
        {
            "name" : "Camera",
            "rotation" : [
                0.483536034822464,
                0.33687159419059753,
                -0.20870360732078552,
                0.7804827094078064
            ],
            "translation" : [
                7.358891487121582,
                4.958309173583984,
                6.925790786743164
            ]
        }
    ],
    "materials" : [
        {
            "doubleSided" : true,
            "name" : "Material",
            "pbrMetallicRoughness" : {
                "baseColorFactor" : [
                    0.800000011920929,
                    0.800000011920929,
                    0.800000011920929,
                    1
                ],
                "metallicFactor" : 0,
                "roughnessFactor" : 0.4000000059604645
            }
        }
    ],
    "meshes" : [
        {
            "name" : "Cube",
            "primitives" : [
                {
                    "attributes" : {
                        "POSITION" : 0,
                        "NORMAL" : 1,
                        "TEXCOORD_0" : 2
                    },
                    "indices" : 3,
                    "material" : 0
                }
            ]
        }
    ],
    "accessors" : [
        {
            "bufferView" : 0,
            "componentType" : 5126,
            "count" : 24,
            "max" : [
                1,
                1,
                1
            ],
            "min" : [
                -1,
                -1,
                -1
            ],
            "type" : "VEC3"
        },
        {
            "bufferView" : 1,
            "componentType" : 5126,
            "count" : 24,
            "type" : "VEC3"
        },
        {
            "bufferView" : 2,
            "componentType" : 5126,
            "count" : 24,
            "type" : "VEC2"
        },
        {
            "bufferView" : 3,
            "componentType" : 5123,
            "count" : 36,
            "type" : "SCALAR"
        }
    ],
    "bufferViews" : [
        {
            "buffer" : 0,
            "byteLength" : 288
        },
        {
            "buffer" : 0,
            "byteLength" : 288,
            "byteOffset" : 288
        },
        {
            "buffer" : 0,
            "byteLength" : 192,
            "byteOffset" : 576
        },
        {
            "buffer" : 0,
            "byteLength" : 72,
            "byteOffset" : 768
        }
    ],
    "buffers" : [
        {
            "byteLength" : 840,
            "uri" : "data:application/octet-stream;base64,AACAPwAAgD8AAIC/AACAPwAAgD8AAIC/AACAPwAAgD8AAIC/AACAPwAAgL8AAIC/AACAPwAAgL8AAIC/AACAPwAAgL8AAIC/AACAPwAAgD8AAIA/AACAPwAAgD8AAIA/AACAPwAAgD8AAIA/AACAPwAAgL8AAIA/AACAPwAAgL8AAIA/AACAPwAAgL8AAIA/AACAvwAAgD8AAIC/AACAvwAAgD8AAIC/AACAvwAAgD8AAIC/AACAvwAAgL8AAIC/AACAvwAAgL8AAIC/AACAvwAAgL8AAIC/AACAvwAAgD8AAIA/AACAvwAAgD8AAIA/AACAvwAAgD8AAIA/AACAvwAAgL8AAIA/AACAvwAAgL8AAIA/AACAvwAAgL8AAIA/AAAAAAAAAAAAAIC/AAAAAAAAgD8AAAAAAACAPwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIC/AACAPwAAAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAAAgD8AAAAAAACAPwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIA/AACAPwAAAAAAAAAAAACAvwAAAAAAAAAAAAAAAAAAAAAAAIC/AAAAAAAAgD8AAAAAAACAvwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIC/AACAvwAAAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAAAgD8AAAAAAACAvwAAAAAAAAAAAAAAAAAAgL8AAACAAAAAAAAAAAAAAIA/AAAgPwAAAD8AACA/AAAAPwAAID8AAAA/AADAPgAAAD8AAMA+AAAAPwAAwD4AAAA/AAAgPwAAgD4AACA/AACAPgAAID8AAIA+AADAPgAAgD4AAMA+AACAPgAAwD4AAIA+AAAgPwAAQD8AACA/AABAPwAAYD8AAAA/AADAPgAAQD8AAAA+AAAAPwAAwD4AAEA/AAAgPwAAgD8AACA/AAAAAAAAYD8AAIA+AADAPgAAgD8AAAA+AACAPgAAwD4AAAAAAQAOABQAAQAUAAcACgAGABMACgATABcAFQASAAwAFQAMAA8AEAADAAkAEAAJABYABQACAAgABQAIAAsAEQANAAAAEQAAAAQA"
        }
    ]
}

"##;
    let jv = serde_json::from_str::<JsonValue>(JSON)?;
    let mut gltf = Gltf::of_json_value(jv)?;
    let h = gltf.node_hierarchy();
    assert_eq!(h.len(), 3);
    assert_eq!(h.borrow_roots().len(), 3);
    assert!(gltf.get_node("0").is_some());
    assert_eq!(gltf.get_node("0"), gltf.get_node("Cube"));
    assert_eq!(gltf.get_node("1"), gltf.get_node("Light"));
    assert_eq!(gltf.get_node("2"), gltf.get_node("Camera"));

    let mut od = mod3d_gltf::ObjectData::new(&gltf);
    od.add_object(&gltf, gltf.get_node("Cube").unwrap());
    od.derive_uses(&gltf);
    let buffers = od.gen_byte_buffers(&mut gltf, &mod3d_gltf::buf_parse_fail, None)?;
    let buffer_data =
        od.gen_buffer_data::<_, _, mod3d_base::example_client::Renderable>(&|x| &buffers[x]);
    let buffer_accessors = od.gen_accessors(&gltf, &|x| &buffer_data[x]);
    let vertices = od.gen_vertices(&gltf, &|x| &buffer_accessors[x]);
    let _object: mod3d_base::Object<mod3d_base::BaseMaterial, _> =
        od.gen_object(&gltf, &vertices, &[], &[]);
    println!("{od:?}");
    println!("{buffers:?}");
    println!("{buffer_data:?}");
    println!("{buffer_accessors:?}");
    println!("{vertices:?}");
    // println!("{object:?}");
    // assert!(false);
    Ok(())
}
