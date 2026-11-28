use super::{Look, Shade};
use crate::eye::Eye;
use crate::resolve::Context;
use crate::{Model, ModelLayer, Param, Quality, Value};
use std::borrow::Cow;

pub(crate) fn eye(cx: &Context) -> (Eye, Option<super::paths::Flight>) {
    let set = cx.script.stage.and_then(|s| s.camera3d).unwrap_or_default();
    let base = cx.scene.camera3d.clone().unwrap_or_default();
    let flight = super::paths::flight(cx, &base);
    let fly = flight.as_ref().map(|f| f.eye);
    let eye = Eye {
        at: set.eye.or(fly.map(|e| e.at)).unwrap_or_else(|| base.eye.get(cx).0),
        center: set.center.or(fly.map(|e| e.center)).unwrap_or_else(|| base.center.get(cx).0),
        up: set.up.or(fly.map(|e| e.up)).unwrap_or(base.up.0),
        fov: set.fov.or(fly.map(|e| e.fov)).unwrap_or_else(|| base.fov.get(cx)).clamp(1.0, 179.0),
        near: base.near,
        far: base.far,
        aspect: 1.0,
    };
    (eye, flight)
}

fn number(cx: &Context, v: Option<&Value<Param>>) -> Option<f32> {
    match v?.get(cx) {
        Param::Number(n) => Some(n),
        _ => None,
    }
}

fn numbers<const N: usize>(cx: &Context, mat: &crate::Material, ids: [&str; N]) -> Option<[f32; N]> {
    let mut out = [0.0; N];
    for (slot, id) in out.iter_mut().zip(ids) {
        *slot = number(cx, mat.params.get(id))?;
    }
    Some(out)
}

fn color(cx: &Context, v: Option<&Value<Param>>) -> Option<[f32; 3]> {
    match v?.get(cx) {
        Param::Color(c) => Some([c.0[0], c.0[1], c.0[2]]),
        _ => None,
    }
}

pub(super) fn looks<'a>(
    cx: &Context<'a>,
    model: &Model,
    m: &'a ModelLayer,
    quality: Quality,
    out: &mut Vec<Look<'a>>,
) {
    let scene: &'a crate::Scene = cx.scene;
    for (name, mat) in &m.materials {
        let Some(index) = model.material(name) else { continue };
        let param = |id: &str| number(cx, mat.params.get(id));
        let [r, g, b, _] = mat.emissive.as_ref().map_or([0.0; 4], |e| e.get(cx).0);
        let shader = mat.shader.as_str();
        let (mut extra, mut slots, mut package) = ([0.0; 8], [[0.0; 4]; 8], 0);
        let mut images = [None; crate::MAX_MATERIAL_IMAGES];
        let shade = match shader {
            "pbr" => Shade::Pbr,
            "unlit" => Shade::Unlit,
            "fur" => {
                let Some([layers, length, density, x, y, z]) =
                    numbers(cx, mat, ["layers", "length", "density", "comb_x", "comb_y", "comb_z"])
                else {
                    continue;
                };
                let shells = crate::model::fur_shells(layers.clamp(1.0, 32.0) as u32, quality) as f32;
                extra = [shells, length * 0.01 * model.radius(), density, x, y, z, 0.0, 0.0];
                Shade::Fur
            }
            "foliage" => {
                let Some(both) = numbers(cx, mat, ["strength", "speed"]) else { continue };
                extra[..2].copy_from_slice(&both);
                Shade::Foliage
            }
            "chroma" => {
                let (Some(key), Some([t, s])) =
                    (color(cx, mat.params.get("key")), numbers(cx, mat, ["threshold", "softness"]))
                else {
                    continue;
                };
                extra = [key[0], key[1], key[2], t, s, 0.0, 0.0, 0.0];
                Shade::Chroma
            }
            id => match scene.effect(id).filter(|d| d.stage == crate::effects::Stage::Material) {
                Some(d) => {
                    images = pictures(d, mat);
                    let found = |k: &str| mat.params.get(k).map(|v| Cow::Owned(v.get(cx)));
                    let all = d.pack(|k| d.setting(quality, k, found(k)));
                    slots.iter_mut().zip(all).for_each(|(s, a)| *s = a);
                    package = d.key;
                    Shade::Package
                }
                None => Shade::Pbr,
            },
        };
        out.push(Look {
            material: index as u16,
            color: mat.color.as_ref().map(|c| c.get(cx).0),
            emissive: mat.emissive.as_ref().map(|_| [r, g, b]),
            unlit: shader == "unlit",
            lit: mat.lit,
            double_sided: mat.double_sided,
            cull: mat.cull,
            depth_test: mat.depth_test,
            depth_write: mat.depth_write,
            blend: mat.blend,
            numbers: [param("metallic"), param("roughness"), param("normal_scale"), param("occlusion")],
            shade,
            extra,
            slots,
            package,
            images,
            cast: mat.cast_shadows,
            receive: mat.receive_shadows,
        });
    }
}

fn pictures<'a>(
    d: &'a crate::Descriptor,
    mat: &'a crate::Material,
) -> [Option<&'a str>; crate::MAX_MATERIAL_IMAGES] {
    let mut out = [None; crate::MAX_MATERIAL_IMAGES];
    let named = |p: &'a Param| match p {
        Param::Text(t) => Some(t.as_str()),
        _ => None,
    };
    for (slot, id) in out.iter_mut().zip(crate::material_images(d)) {
        let given = match mat.params.get(id) {
            Some(Value::Fixed(p)) => named(p),
            _ => None,
        };
        let default = d.params.iter().find(|p| p.id == id).and_then(|p| named(&p.prefill));
        *slot = given.or(default);
    }
    out
}
