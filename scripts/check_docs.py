"""Check maintained Markdown links, fences and paired handbook code blocks.

Uses only the Python standard library. Third-party vendored documentation and
patch source archives retain their original upstream references and hashes.
"""
import argparse
from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]


def headings(text):
    counts = {}
    result = set()
    for title in re.findall(r"(?m)^#{1,6}\s+(.+?)\s*#*\s*$", text):
        slug = re.sub(r"[^\w\-\s]", "", title.lower()).replace(" ", "-")
        count = counts.get(slug, 0)
        counts[slug] = count + 1
        result.add(slug if count == 0 else f"{slug}-{count}")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--preview-cleanup", action="store_true",
                        help="check the planned file tree, without deleting anything")
    args = parser.parse_args()
    excluded = set()
    if args.preview_cleanup:
        script = (ROOT / "scripts/clean-delivery.ps1").read_text(encoding="utf-8-sig")
        block = script.split("# BEGIN REMOVAL PATHS\n", 1)[1].split("# END REMOVAL PATHS", 1)[0]
        excluded = {p for p in re.findall(r"^\s*'([^']+)'[,]?\s*$", block, re.M)}

    def absent(path):
        relative = path.relative_to(ROOT).as_posix()
        return any(relative == item or relative.startswith(item + "/") for item in excluded)

    files = [*ROOT.glob("*.md")]
    for folder in ("docs", "App", "xtask/templates"):
        files.extend((ROOT / folder).rglob("*.md"))
    files = sorted(p for p in files if not absent(p))
    errors = []
    links = 0
    for path in files:
        text = path.read_text(encoding="utf-8")
        if len(re.findall(r"(?m)^```", text)) % 2:
            errors.append(f"{path.relative_to(ROOT)}: unclosed code fence")
        for match in re.finditer(r"\[[^\]\n]+\]\(([^)\n]+)\)", text):
            target = match.group(1).strip("<>")
            url = urlsplit(target)
            if url.scheme or target.startswith("//"):
                continue
            linked = (path.parent / unquote(url.path)).resolve() if url.path else path
            if not linked.is_relative_to(ROOT):
                errors.append(f"{path.relative_to(ROOT)}: outside repository: {target}")
                continue
            links += 1
            if not linked.exists() or absent(linked):
                errors.append(f"{path.relative_to(ROOT)}: missing link: {target}")
            elif url.fragment and linked.suffix == ".md":
                if unquote(url.fragment) not in headings(linked.read_text(encoding="utf-8")):
                    errors.append(f"{path.relative_to(ROOT)}: missing heading: {target}")

    def blocks(path):
        text = path.read_text(encoding="utf-8")
        return re.findall(r"(?ms)^```([^\n]*)\n(.*?)^```", text)

    for zh in (ROOT / "docs/zh-CN").glob("*.md"):
        en = ROOT / "docs/en" / zh.name
        if not en.is_file():
            errors.append(f"missing English chapter: {zh.name}")
        elif blocks(zh) != blocks(en):
            errors.append(f"bilingual code blocks differ: {zh.name}")
    for error in errors:
        print(error, file=sys.stderr)
    mode = "planned cleanup preview" if args.preview_cleanup else "current tree"
    print(f"{mode}: {len(files)} documents, {links} local links, {len(errors)} errors")
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
