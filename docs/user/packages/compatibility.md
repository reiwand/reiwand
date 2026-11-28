# Compatibility and versions

Packages say which engine features they use, and each Reiwand version decides what it can play.
This page explains how features are computed, what older versions see, and where the reference
tables are.

## Engine features

`reiwand-ctl pack build` computes the engine features that the package uses (`requires.features`)
and writes them to the manifest.

If a package asks for a feature or a version that this Reiwand does not know, it says that a newer
version is needed (on the desktop, with its thumbnail).

The validator checks that the declared inputs include those that the scene can use with any value
of its properties, and says where it uses them.

## Older versions of Reiwand

- Versions published before version 2 of the manifest do not open their packages, but keep reading
  the repositories that have them. They only see in them those of the first version.
- Web pages, presets and resource packages go only in the new repository indexes, which those
  versions do not read, so they do not see them.
- Repository clients skip the versions of a package type they do not know instead of failing to
  read the index.

## After updating Reiwand

After updating Reiwand, the library reads its packages again once. Packages that asked for a newer
version start working, and the earlier ones receive their facets.

On the phone, the page of a package warns of what does not work there.

## Reference

- The table of profiles (what the desktop and the phone do with each feature) is in
  `docs/specs/package.md`.
- `docs/specs/scene.md` ends with a guide to remake by hand a scene made with another program:
  where each part goes in the format and how to port HLSL and GLSL functions to WGSL.
