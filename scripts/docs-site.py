"""Builds the documentation website from docs/ (see scripts/docs-site.sh, which runs it).

Each Markdown page goes through pandoc (with packaging/docs/links.lua) and into
packaging/docs/page.html, with the side navigation, breadcrumbs, previous and next pages, a table
of contents and a search index (search.json) made here. The navigation follows the sections, their
sub-levels and the order of docs/README.md; pages it does not list go to the section of their
folder. The build fails on a link to a page or a heading that is not there.
"""

import html
import json
import os
import re
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DOCS = os.path.join(ROOT, "docs")
KIT = os.path.join(ROOT, "packaging", "docs")
FOLDERS = {"user": "Using Reiwand", "authoring": "Creating wallpapers", "specs": "Specifications",
           "development": "Development"}


def pages():
    found = []
    for base, dirs, files in os.walk(DOCS):
        dirs.sort()
        found += [os.path.relpath(os.path.join(base, f), DOCS) for f in sorted(files) if f.endswith(".md")]
    return found


def title(rel):
    with open(os.path.join(DOCS, rel), encoding="utf-8") as f:
        for line in f:
            if line.startswith("# "):
                return re.sub(r"[`*]", "", line[2:].strip())
    return os.path.basename(rel)[:-3].replace("-", " ").capitalize()


def url(rel):
    out = rel[:-3] + ".html"
    return re.sub(r"(^|/)README\.html$", r"\1index.html", out)


def navigation(all_pages):
    """[(section, [(sub-level, [page, ...]), ...])] in the order of docs/README.md (`##` a section, `###`
    a sub-level of it, "" for the pages right under the section), then the rest by folder."""
    order, seen = [], set()
    with open(os.path.join(DOCS, "README.md"), encoding="utf-8") as f:
        for line in f:
            if line.startswith("## "):
                order.append((line[3:].strip(), [("", [])]))
            elif line.startswith("### ") and order:
                order[-1][1].append((line[4:].strip(), []))
            for target in re.findall(r"\]\(([^)#]+\.md)\)", line):
                rel = os.path.normpath(target)
                if order and rel in all_pages and rel not in seen:
                    order[-1][1][-1][1].append(rel)
                    seen.add(rel)
    for _, subs in order:
        for _, group in subs:
            for rel in group:
                seen.update(children(rel, all_pages))
    named = {s: subs for s, subs in order}
    for rel in all_pages:
        if rel in seen or rel == "README.md":
            continue
        folder = rel.split("/")[0] if "/" in rel else ""
        section = FOLDERS.get(folder, "Other pages")
        if section not in named:
            named[section] = [("", [])]
            order.append((section, named[section]))
        named[section][0][1].append(rel)
    return [(s, [(n, p) for n, p in subs if p]) for s, subs in order if any(p for _, p in subs)]


def children(rel, all_pages):
    """The pages of the folder named after `rel` (a/b.md owns a/b/*.md), in the order `rel` links to them."""
    folder = rel[:-3] + "/"
    with open(os.path.join(DOCS, rel), encoding="utf-8") as f:
        linked = [os.path.normpath(os.path.join(os.path.dirname(rel), t))
                  for t in re.findall(r"\]\(([^)#\s]+\.md)", f.read())]
    own = [p for p in all_pages if p.startswith(folder) and "/" not in p[len(folder):]]
    return [p for p in dict.fromkeys(linked) if p in own] + sorted(p for p in own if p not in linked)


def checked(out):
    """Every link of the site to a page of it reaches that page, and its #anchor a heading there."""
    ids, links = {}, []
    for base, _, files in os.walk(out):
        for name in files:
            if name.endswith(".html"):
                path = os.path.join(base, name)
                with open(path, encoding="utf-8") as f:
                    page = f.read()
                ids[path] = set(re.findall(r'\sid="([^"]+)"', page))
                links += [(path, h) for h in re.findall(r'<a\s[^>]*href="([^"]+)"', page)]
    broken = []
    for path, href in links:
        if re.match(r"^[a-z][\w+.-]*:", href) or href.startswith("/"):
            continue
        target, _, anchor = html.unescape(href).partition("#")
        to = os.path.normpath(os.path.join(os.path.dirname(path), target)) if target else path
        if os.path.isdir(to):
            to = os.path.join(to, "index.html")
        if not os.path.exists(to) or (anchor and to.endswith(".html") and anchor not in ids.get(to, ())):
            broken.append(f"{os.path.relpath(path, out)}: {href}")
    return sorted(set(broken))


def pandoc(rel, titles_file):
    template = os.path.join(KIT, "fragment.html")
    out = subprocess.run(
        ["pandoc", "--from=gfm", "--to=html5", "--standalone", f"--template={template}", "--toc",
         "--toc-depth=3", f"--lua-filter={os.path.join(KIT, 'links.lua')}",
         f"--metadata=source:{rel}", f"--metadata=titles:{titles_file}", "--metadata=title:x",
         os.path.join(DOCS, rel)],
        check=True, capture_output=True, text=True, cwd=ROOT)
    toc, _, body = out.stdout.partition("<!--body-->")
    return toc.strip(), body.strip()


def anchored(body):
    """Every heading below the title gets a link to itself (after pandoc, so the contents stay clean)."""
    return re.sub(r'(<h([2-4]) id="([^"]+)"[^>]*>)(.*?)(</h\2>)',
                  r'\1\4<a class="anchor" href="#\3" aria-label="Link to this section">#</a>\5', body, flags=re.S)


def cards(body):
    """On the start page each listed page shows its title, and its line only when it adds something."""
    def card(m):
        link, rest = m.group(1), m.group(2).strip()
        rest = rest[:1].upper() + rest[1:]
        name = re.sub(r"<[^>]+>", "", link).lower().strip(" .")
        said = re.sub(r"<[^>]+>", "", rest).lower().strip(" .:")
        repeats = set(re.findall(r"\w+", said)) <= set(re.findall(r"\w+", name))
        return f"<li>{link}" + ("" if repeats else f"<span>{rest}</span>") + "</li>"
    return re.sub(r"<li>(<a [^>]+>.*?</a>):?\s*(.*?)</li>", card, body, flags=re.S)


def text(fragment):
    return html.unescape(re.sub(r"\s+", " ", re.sub(r"<[^>]+>", " ", fragment))).strip()


def entries(rel, page_title, body):
    """One search entry per section of the page: its heading, anchor and the start of its text."""
    found, parts = [], re.split(r"(<h[1-3][^>]*>.*?</h[1-3]>)", body, flags=re.S)
    heading, anchor = page_title, ""
    for part in parts:
        m = re.match(r'<h[1-3][^>]*id="([^"]+)"[^>]*>(.*?)</h[1-3]>', part, flags=re.S)
        if m:
            anchor, heading = m.group(1), text(m.group(2)).rstrip("#").strip()
            continue
        words = text(part)
        if words:
            found.append({"p": url(rel), "t": page_title, "h": heading, "a": anchor, "x": words[:400]})
    return found


def item(p, kids, rel, up, titles):
    """A page of the side navigation, with its own pages below it, open on them."""
    link = f'<a href="{up}{url(p)}"{" aria-current=page" if p == rel else ""}>{html.escape(titles[p])}</a>'
    if not kids:
        return f"<li>{link}</li>"
    inner = "".join(item(k, [], rel, up, titles) for k in kids)
    here = " open" if rel == p or rel in kids else ""
    return f"<li class=has><details{here}><summary>{link}</summary><ul>{inner}</ul></details></li>"


def build(out):
    all_pages = pages()
    titles = {rel: title(rel) for rel in all_pages}
    titles["README.md"] = "Reiwand documentation"
    nav = navigation(all_pages)
    kids = {rel: children(rel, all_pages) for _, subs in nav for _, group in subs for rel in group}
    flat = ["README.md"] + [r for _, subs in nav for _, group in subs for rel in group for r in [rel] + kids[rel]]
    place = {rel: (s, n) for s, subs in nav for n, group in subs for rel in group}
    place.update({k: place[rel] + (titles[rel],) for rel, ks in kids.items() for k in ks})
    owner = {k: rel for rel, ks in kids.items() for k in ks}
    os.makedirs(out, exist_ok=True)
    titles_file = os.path.join(out, ".titles.json")
    with open(titles_file, "w", encoding="utf-8") as f:
        json.dump({rel: titles[rel] for rel in all_pages}, f)
    with open(os.path.join(KIT, "page.html"), encoding="utf-8") as f:
        page_template = f.read()
    index = []
    for i, rel in enumerate(flat):
        up = "../" * rel.count("/")
        toc, body = pandoc(rel, titles_file)
        body = anchored(cards(body) if rel == "README.md" else body)
        index += entries(rel, titles[rel], body)
        links = []
        for section, subs in nav:
            parts = []
            for sub, group in subs:
                items = "".join(item(p, kids[p], rel, up, titles) for p in group)
                if sub:
                    here = " open" if rel in group or owner.get(rel) in group else ""
                    parts.append(f"<li><details class=sub{here}><summary>{html.escape(sub)}</summary><ul>{items}</ul></details></li>")
                else:
                    parts.append(items)
            here = " open" if place.get(rel, ("",))[0] == section else ""
            links.append(f"<details{here}><summary>{html.escape(section)}</summary><ul>{''.join(parts)}</ul></details>")
        near = []
        for label, j in (("Previous", i - 1), ("Next", i + 1)):
            if 0 <= j < len(flat):
                near.append(f'<a class="{label.lower()}" href="{up}{url(flat[j])}"><span>{label}</span>{html.escape(titles[flat[j]])}</a>')
        crumbs = f'<a href="{up}index.html">Docs</a>' + "".join(
            f"<span>{html.escape(n)}</span>" for n in place.get(rel, ()) if n)
        if rel in owner:
            crumbs = crumbs.rsplit("<span>", 1)[0] + f'<span><a href="{up}{url(owner[rel])}">{html.escape(titles[owner[rel]])}</a></span>'

        filled = {"title": html.escape(titles[rel]), "root": up, "nav": "".join(links), "crumbs": crumbs,
                  "toc": toc if toc.count("<li") > 1 else "", "body": body, "near": "".join(near),
                  "source": rel, "kind": "start" if rel == "README.md" else "page"}
        page = re.sub(r"\{\{(\w+)\}\}", lambda m: filled[m.group(1)], page_template)
        target = os.path.join(out, url(rel))
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf-8") as f:
            f.write(page)
    os.remove(titles_file)
    for base, _, files in os.walk(DOCS):
        for name in files:
            if not name.endswith(".md"):
                rel = os.path.relpath(os.path.join(base, name), DOCS)
                os.makedirs(os.path.join(out, os.path.dirname(rel)), exist_ok=True)
                shutil.copyfile(os.path.join(base, name), os.path.join(out, rel))
    assets = os.path.join(out, "assets")
    os.makedirs(os.path.join(assets, "fonts"), exist_ok=True)
    for name in ("site.css", "search.js"):
        shutil.copyfile(os.path.join(KIT, name), os.path.join(assets, name))
    shutil.copyfile(os.path.join(ROOT, "packaging", "icons", "io.github.reiwand.svg"), os.path.join(assets, "reiwand.svg"))
    fonts = os.path.join(ROOT, "apps", "reiwand-server", "assets", "fonts")
    for name in ("RedHatDisplay.ttf", "Onest.ttf", "OFL-redhatdisplay.txt", "OFL-onest.txt"):
        shutil.copyfile(os.path.join(fonts, name), os.path.join(assets, "fonts", name))
    with open(os.path.join(out, "search.json"), "w", encoding="utf-8") as f:
        json.dump(index, f, ensure_ascii=False, separators=(",", ":"))
    broken = checked(out)
    if broken:
        sys.exit("docs-site: broken links:\n" + "\n".join(broken))
    print(f"docs-site: {len(flat)} pages, {len(index)} search entries in {out}")


if __name__ == "__main__":
    build(os.path.abspath(sys.argv[1]))
