use super::*;

#[test]
fn values_written_out_at_zero_have_a_budget_across_the_whole_module() {
    let zeroed = format!("sets more than {} KiB of values to zero", MAX_ZEROED >> 10);
    let unused = [
        "fn unused(i: u32) -> f32 { return array<vec4<f32>, 4000000>()[i].x; }",
        "fn unused(i: u32) -> f32 { var a: array<vec4<f32>, 4000000>; a[i] = vec4<f32>(1.0); return a[i / 2u].x; }",
    ];
    for f in unused {
        refused(format!("{f}\n{}", entry("return tap(in.uv);")), &zeroed);
    }
    let many: String = (0..12)
        .map(|n| format!("fn u{n}(i: u32) -> f32 {{ return array<vec4<f32>, 400>()[i].x; }}\n"))
        .collect();
    refused(format!("{many}{}", entry("return tap(in.uv);")), &zeroed);
    let few: String = (0..2)
        .map(|n| format!("fn u{n}(i: u32) -> f32 {{ return array<vec4<f32>, 64>()[i].x; }}\n"))
        .collect();
    pack(&format!("{few}{}", entry("return tap(in.uv) + u0(1u) + u1(2u);"))).unwrap();
}
