package io.github.reiwand

const val MEDIA_SCENE = """{ "schema": "reiwand.scene/1", "canvas": { "size": [64, 64] },
    "layers": [{ "blend_space": "canvas", "id": "back", "type": "solid", "color": "#000000" },
        { "blend_space": "canvas", "id": "tint", "type": "solid", "color": { "media": "primary" }, "opacity": { "media": "playing" } }] }"""
const val MEDIA_MANIFEST = """{ "schema": "reiwand.wpak/2", "id": "org.example.tinted", "version": "1.0.0",
    "title": "Tinted", "license": "CC0-1.0", "kind": "scene", "entry": "scene.json", "origin": { "kind": "original" },
    "content": { "rating": "general" }, "photosensitive": false, "inputs": ["media"],
    "requires": { "features": ["inputs.media", "media_info"] } }"""
