"""Unchanged Raw access postprocessor from the verified native PAC generation."""
import ast
import re
from pathlib import Path
HERE=Path(__file__).resolve().parent

def guard_raw(pac, products):
    source = (HERE / "sources/generate.py").read_text(encoding="utf-8")
    assignment = next(n for n in ast.parse(source).body if isinstance(n, ast.Assign)
                      and any(isinstance(t, ast.Name) and t.id == "RAW_API" for t in n.targets))
    raw_api = ast.literal_eval(assignment.value)
    common = pac / "src/common.rs"
    common.write_text(common.read_text(encoding="utf-8") + raw_api, encoding="utf-8", newline="\n")
    metadata = pac / "src/metadata.rs"
    meta = metadata.read_text(encoding="utf-8")
    def append(match):
        body = match[1]
        if re.findall(r"\b(ReadWrite|Read|Write)\s*,", body) != ["ReadWrite", "Read", "Write"]:
            raise ValueError("Original Access variants changed")
        return "pub enum Access {" + body + "\n    Raw,\n}"
    meta, count = re.subn(r"pub enum Access \{([^}]+)\}", append, meta, count=1)
    if count != 1:
        raise ValueError("Missing Access enum")
    metadata.write_text(meta, encoding="utf-8", newline="\n")
    total = 0
    for family, p in products.items():
        for kind, names in p["raw_registers"].items():
            path = pac / "src/peripherals" / (kind + "_" + family.lower() + "_47a.rs")
            if not path.exists():
                continue  # unused native view, replaced by a verified shared IP
            source = path.read_text(encoding="utf-8")
            metadata_path = pac / "src/registers" / path.name
            meta = metadata_path.read_text(encoding="utf-8")
            for full in names:
                block, method = full.split(".")
                impls = list(re.finditer(r"impl (\w+) \{", source))
                target = next((m for m in impls if m[1].lower() == block.replace("_", "").lower()), None)
                if target is None:
                    raise ValueError("Missing generated block: " + full)
                end = next((m.start() for m in impls if m.start() > target.start()), len(source))
                section = source[target.start():end]
                pattern = r"(pub const fn " + re.escape(method.lower()) + r"\([^)]*\)\s*->\s*crate::common::Reg<.*?,\s*crate::common::)(RW|R|W)(>)"
                section, count = re.subn(pattern, r"\1Raw\3", section, count=1, flags=re.S)
                if count != 1:
                    raise ValueError("Missing generated raw register: " + full)
                source = source[:target.start()] + section + source[end:]
                blocks = list(re.finditer(r'Block \{\s*name: "(\w+)"', meta))
                target = next((m for m in blocks if m[1].lower() == block.replace("_", "").lower()), None)
                if target is None:
                    raise ValueError("Missing metadata block: " + full)
                end = next((m.start() for m in blocks if m.start() > target.start()), len(meta))
                section = meta[target.start():end]
                pattern = r'(BlockItem \{\s*name: "' + re.escape(method.lower()) + r'",.*?access: Access::)(ReadWrite|Read|Write)'
                section, count = re.subn(pattern, r"\1Raw", section, count=1, flags=re.S)
                if count != 1:
                    raise ValueError("Missing raw metadata item: " + full)
                meta = meta[:target.start()] + section + meta[end:]
                total += 1
            path.write_text(source, encoding="utf-8", newline="\n")
            metadata_path.write_text(meta, encoding="utf-8", newline="\n")
    return total
