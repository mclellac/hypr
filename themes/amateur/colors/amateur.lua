-- themes/amateur/colors/amateur.lua

-- Set the colorscheme name
vim.g.colors_name = "amateur"

-- Get the color palette
local colors = {
  bg = "#05080d",
  fg = "#cdd6f4",
  gray = "#3b4252",
  red = "#ff4444",
  green = "#00ffcc",
  yellow = "#ffcc00",
  blue = "#00aaff",
  magenta = "#d670d6",
  cyan = "#00e5ff",
  white = "#cdd6f4",
  orange = "#ffaa00",
  dark_blue = "#1e293b",
}

-- Helper function to set highlights
local function s(group, opts)
  vim.api.nvim_set_hl(0, group, opts)
end

-- Set the highlight groups
s("Normal", { fg = colors.fg, bg = colors.bg })
s("Comment", { fg = colors.gray, italic = true })
s("String", { fg = colors.green })
s("Number", { fg = colors.orange })
s("Keyword", { fg = colors.cyan, italic = true })
s("Function", { fg = colors.blue })
s("Type", { fg = colors.magenta })
s("Statement", { fg = colors.cyan })
s("Identifier", { fg = colors.fg })
s("Title", { fg = colors.blue, bold = true })
s("LineNr", { fg = colors.gray })
s("CursorLineNr", { fg = colors.cyan, bold = true })
s("Visual", { bg = colors.dark_blue })
s("Search", { bg = colors.yellow, fg = colors.bg })
s("IncSearch", { bg = colors.orange, fg = colors.bg })
s("DiffAdd", { bg = "#00ffcc", fg = colors.bg })
s("DiffChange", { bg = "#ffcc00", fg = colors.bg })
s("DiffDelete", { bg = "#ff4444", fg = colors.bg })
s("DiffText", { bg = "#00aaff", fg = colors.bg })
