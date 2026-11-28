// Search of the documentation site: search.json (built with the site) holds one entry per section
// of every page; matches rank by page title, then heading, then text. No outside service.
(() => {
  const root = document.currentScript.dataset.root;
  const input = document.getElementById("find");
  const box = document.getElementById("found");
  let index = null;
  let shown = [];
  let at = -1;

  const load = () => index ?? (index = fetch(root + "search.json").then((r) => r.json()));
  const words = (q) => q.toLowerCase().split(/\s+/).filter((w) => w.length > 1);
  const escape = (s) => s.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]);
  const marked = (s, ws) => ws.reduce((out, w) =>
    out.replace(new RegExp("(" + w.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + ")", "gi"), "<mark>$1</mark>"), escape(s));

  function score(e, ws) {
    const t = e.t.toLowerCase(), h = e.h.toLowerCase(), x = e.x.toLowerCase();
    let total = 0;
    for (const w of ws) {
      const s = (t.includes(w) ? 8 : 0) + (h.includes(w) ? 5 : 0) + (x.includes(w) ? 1 : 0);
      if (s === 0) return 0;
      total += s;
    }
    return total;
  }

  function snippet(text, ws) {
    const lower = text.toLowerCase();
    const first = Math.min(...ws.map((w) => lower.indexOf(w)).filter((i) => i >= 0));
    const from = Math.max(0, (Number.isFinite(first) ? first : 0) - 60);
    return (from > 0 ? "…" : "") + text.slice(from, from + 180) + "…";
  }

  function pick(i) {
    at = Math.max(0, Math.min(i, shown.length - 1));
    box.querySelectorAll("a").forEach((a, j) => a.setAttribute("aria-selected", String(j === at)));
    box.querySelectorAll("a")[at]?.scrollIntoView({ block: "nearest" });
  }

  async function find() {
    const ws = words(input.value);
    if (ws.length === 0) { box.hidden = true; return; }
    const all = await load();
    shown = all.map((e) => [score(e, ws), e]).filter(([s]) => s > 0).sort((a, b) => b[0] - a[0]).slice(0, 12).map(([, e]) => e);
    box.innerHTML = shown.length === 0
      ? '<p class="none">Nothing found for “' + escape(input.value) + "”.</p>"
      : shown.map((e) => '<a href="' + root + e.p + (e.a ? "#" + e.a : "") + '"><b>' + marked(e.h, ws) +
          "</b><small>" + escape(e.t) + "</small><p>" + marked(snippet(e.x, ws), ws) + "</p></a>").join("");
    box.hidden = false;
    at = -1;
  }

  input.addEventListener("focus", load);
  input.addEventListener("input", find);
  input.addEventListener("keydown", (ev) => {
    if (ev.key === "ArrowDown") { ev.preventDefault(); pick(at + 1); }
    else if (ev.key === "ArrowUp") { ev.preventDefault(); pick(at - 1); }
    else if (ev.key === "Enter" && shown.length) { location.href = box.querySelectorAll("a")[Math.max(at, 0)].href; }
    else if (ev.key === "Escape") { box.hidden = true; input.blur(); }
  });
  document.addEventListener("keydown", (ev) => {
    if (ev.key === "/" && document.activeElement !== input) { ev.preventDefault(); input.focus(); }
  });
  document.addEventListener("click", (ev) => { if (!box.contains(ev.target) && ev.target !== input) box.hidden = true; });

  // The table of contents marks the section being read.
  const links = [...document.querySelectorAll(".toc a")];
  const heads = links.map((a) => document.getElementById(decodeURIComponent(a.hash.slice(1)))).filter(Boolean);
  if (heads.length) {
    const seen = new IntersectionObserver(() => {
      const top = heads.filter((h) => h.getBoundingClientRect().top < 120).pop() ?? heads[0];
      links.forEach((a) => a.classList.toggle("here", a.hash === "#" + top.id));
    }, { rootMargin: "0px 0px -70% 0px" });
    heads.forEach((h) => seen.observe(h));
  }
})();
