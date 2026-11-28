-- Turns the links of a docs/ page into links of the site: another page becomes its .html (a
-- README its index.html); a file outside docs/ points to the source on GitHub.
local source = ""
local titles = {}
local blob = "https://github.com/reiwand/reiwand/blob/master/"

local function split(path)
  local parts = {}
  for part in path:gmatch("[^/]+") do
    if part == ".." then
      if #parts == 0 then return nil end
      table.remove(parts)
    elseif part ~= "." then
      table.insert(parts, part)
    end
  end
  return parts
end

local function page(path)
  path = path:gsub("README%.md$", "index.md")
  return (path:gsub("%.md$", ".html"))
end

function Meta(m)
  source = pandoc.utils.stringify(m.source or "")
  local file = io.open(pandoc.utils.stringify(m.titles or ""))
  if file then
    titles = pandoc.json.decode(file:read("a"))
    file:close()
  end
end

-- A link whose text is only the file name or path of a page shows that page's title instead.
local function retitle(l, rel)
  local shown = pandoc.utils.stringify(l.content)
  local name = titles[rel]
  if name and (shown:match("%.md$") or shown:match("/$") or shown:match("^README") or #l.content == 1 and l.content[1].t == "Code") then
    l.content = { pandoc.Str(name) }
  end
end



function Link(l)
  local target, frag = l.target:match("^([^#]*)(#?.*)$")
  if target == "" or target:match("^%a[%w+.-]*:") or target:match("^/") then return l end
  local dir = source:match("^(.*)/[^/]*$") or ""
  local parts = split((dir ~= "" and dir .. "/" or "") .. target)
  if parts == nil then
    local whole = split("docs/" .. (dir ~= "" and dir .. "/" or "") .. target)
    if whole then
      local path = table.concat(whole, "/")
      local file = path:match("%.md$") and io.open(path)
      if file then
        local first = file:read("l") or ""
        file:close()
        local name = first:match("^# (.+)$")
        local shown = pandoc.utils.stringify(l.content)
        if name and (shown:match("%.md$") or shown:match("^README") or #l.content == 1 and l.content[1].t == "Code") then
          l.content = { pandoc.Str(name) }
        end
      end
      l.target = blob .. path .. frag
    end
    return l
  end
  if target:match("%.md$") then
    retitle(l, table.concat(parts, "/"))
    l.target = page(target) .. frag
  elseif target:match("/$") or (#parts > 0 and not parts[#parts]:match("%.")) then
    if not target:match("/$") then target = target .. "/" end
    local readme = io.open("docs/" .. table.concat(parts, "/") .. "/README.md")
    if readme then
      readme:close()
      retitle(l, table.concat(parts, "/") .. "/README.md")
      l.target = target .. "index.html" .. frag
    else
      l.target = blob:gsub("/blob/", "/tree/") .. "docs/" .. table.concat(parts, "/") .. frag
    end
  end
  return l
end

return { { Meta = Meta }, { Link = Link } }
