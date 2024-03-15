use std::collections::HashMap;

use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Res,
    math::primitives::{Cuboid, Sphere},
    pbr::StandardMaterial,
    render::{color::Color, mesh::Mesh, texture::Image},
};
use common::{resources::render::StandardMaterialData, ResourceData, WorldResource};

use crate::util::strip_res_prefix;

pub enum MeshInfo {
    ArrayMesh(Handle<Mesh>),
    Mesh(Mesh),
}

pub enum MaterialInfo {
    Texture(Handle<Image>),
    Material(StandardMaterial),
}

pub struct MeshData {
    pub mesh: MeshInfo,
    pub material: MaterialInfo,
}

pub fn create_mesh_from_data(
    resource: &ResourceData,

    resources: &HashMap<String, WorldResource>,
    asset_server: &Res<AssetServer>,
) -> MeshData {
    let (mesh, material): (MeshInfo, Option<String>) = match resource {
        ResourceData::BoxMesh(bm) => (
            MeshInfo::Mesh(Cuboid::new(bm.size[0], bm.size[1], bm.size[2]).into()),
            bm.material.clone(),
        ),
        ResourceData::SphereMesh(sm) => {
            let mesh = Sphere { radius: sm.radius }.try_into().unwrap();

            (MeshInfo::Mesh(mesh), sm.material.clone())
        }
        ResourceData::ArrayMesh(am) => {
            let path = strip_res_prefix(&am.path);
            let res: Handle<Mesh> = asset_server.load(path);

            (MeshInfo::ArrayMesh(res), None)
        }
        _ => panic!("is not mesh"),
    };

    let mut material_info: MaterialInfo = MaterialInfo::Material(Color::WHITE.into());
    if let Some(mat) = material {
        let material_data = resources.get(&mat).unwrap();
        let material = get_material_from_resource(material_data);

        if let Some(albedo_texture) = material.albedo_texture {
            let res = resources.get(&albedo_texture).unwrap();

            if let ResourceData::Texture2D(tex) = &res.data {
                let texture_handle: Handle<Image> = asset_server.load(strip_res_prefix(&tex.path));
                material_info = MaterialInfo::Texture(texture_handle);
            }
        } else {
            material_info = MaterialInfo::Material(
                Color::rgba(
                    material.albedo_color[0],
                    material.albedo_color[1],
                    material.albedo_color[2],
                    material.albedo_color[3],
                )
                .into(),
            );
        }
    }

    return MeshData {
        mesh,
        material: material_info,
    };
}

pub fn create_mesh_from_resource(
    mesh_name: String,
    resources: &HashMap<String, WorldResource>,
    asset_server: &Res<AssetServer>,
) -> MeshData {
    let resource = if let Some(ok) = resources.get(&mesh_name) {
        ok
    } else {
        panic!("unable to get mesh");
    };

    return create_mesh_from_data(&resource.data, resources, asset_server);
}

pub fn get_material_from_resource(resource: &WorldResource) -> StandardMaterialData {
    // TODO: the actual parsed data could be cached somewhere...
    if let ResourceData::StandardMaterial(material) = &resource.data {
        return material.clone();
    }

    panic!();
}
