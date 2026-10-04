"""Apply the G0 UCPD fix after DIE47C, retaining reversible, hash-bound provenance."""
import copy
import hashlib
import json

PATCH = "data/patches/embassy-stm32/ucpd-port-cfg.json"


def encode(value):
    return (json.dumps(value, indent=2) + "\n").encode()


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def inventory(files):
    return {n: sha(raw) for n, raw in sorted(files.items())}


def load(root):
    raw = (root / PATCH).read_bytes()
    spec = json.loads(raw)
    baseline = spec["baseline_manifest"]
    if (spec["schema_version"] != 1 or spec["base_revision"] != baseline["revision"]
            or sha(encode(baseline)) != spec["baseline_manifest_sha256"]
            or [r["path"] for r in spec["files"]] != ["src/ucpd.rs"]
            or spec.get("new_files") or baseline.get("post_patches")):
        raise ValueError("UCPD source patch baseline or scope differs")
    return spec, sha(raw)


def apply(files, manifest, root):
    from vendor_embassy import patch_source

    spec, patch_sha = load(root)
    if (sha(encode(manifest)) != spec["baseline_manifest_sha256"]
            or inventory(files) != manifest["files"]):
        raise ValueError("UCPD patch baseline inventory/provenance differs")
    result = dict(files)
    for record in spec["files"]:
        result[record["path"]] = patch_source(result[record["path"]], record)
    output = copy.deepcopy(manifest)
    output["post_patches"] = [{"path": PATCH, "sha256": patch_sha}]
    output["changes"] += "; G0 UCPD dead-battery strobe cfg follows each available peripheral"
    output["files"] = inventory(result)
    return result, output


def restore(files, manifest, root):
    if not manifest.get("post_patches"):
        return files, encode(manifest)
    spec, patch_sha = load(root)
    if manifest["post_patches"] != [{"path": PATCH, "sha256": patch_sha}]:
        raise ValueError("UCPD patch provenance differs")
    if inventory(files) != manifest["files"]:
        raise ValueError("UCPD patched inventory differs")
    original = dict(files)
    for record in reversed(spec["files"]):
        name = record["path"]
        if sha(original[name]) != record["patched_sha256"]:
            raise ValueError("UCPD patch is partially published or modified")
        text = original[name].decode("utf-8")
        for replacement in reversed(record["replacements"]):
            if text.count(replacement["new"]) != 1:
                raise ValueError("UCPD patch reverse hunk is ambiguous")
            text = text.replace(replacement["new"], replacement["old"])
        original[name] = text.encode("utf-8")
        if sha(original[name]) != record["upstream_sha256"]:
            raise ValueError("UCPD restored source differs from the baseline")
    rebuilt, rebuilt_manifest = apply(original, spec["baseline_manifest"], root)
    if rebuilt != files or rebuilt_manifest != manifest:
        raise ValueError("UCPD reverse replay differs from the published composition")
    return original, encode(spec["baseline_manifest"])
