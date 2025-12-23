-- themes/amateur/colors/amateur.lua

-- Set the colorscheme name
vim.g.colors_name = "amateur"

-- Get the color palette
local colors = {
  bg = "#161616",
  fg = "#e6e6e6",
  gray = "#333333",
  red = "#d63031",
  green = "#27ae60",
  yellow = "#e67e22",
  blue = "#1f66c5",
  magenta = "#9b59b6",
  cyan = "#4db6eb",
  white = "#ffffff",
  orange = "#e67e22",
  dark_blue = "#1f66c5",
  light_blue = "#4db6eb",
}

-- Helper function to set highlights
local function s(group, opts)
  vim.api.nvim_set_hl(0, group, opts)
end

-- Set the highlight groups
s("Normal", { fg = colors.fg, bg = colors.bg })
s("Comment", { fg = "#666666", italic = true })
s("String", { fg = colors.green })
s("Number", { fg = colors.orange })
s("Keyword", { fg = colors.light_blue, italic = true })
s("Function", { fg = colors.cyan })
s("Type", { fg = colors.light_blue })
s("Statement", { fg = colors.light_blue })
s("Identifier", { fg = colors.fg })
s("Title", { fg = colors.cyan, bold = true })
s("LineNr", { fg = "#444444" })
s("CursorLineNr", { fg = colors.yellow, bold = true })
s("Visual", { bg = colors.blue, fg = colors.white })
s("Search", { bg = colors.yellow, fg = colors.bg })
s("IncSearch", { bg = colors.orange, fg = colors.bg })
s("DiffAdd", { bg = "#0d3321" })
s("DiffChange", { bg = "#332200" })
s("DiffDelete", { bg = "#330d0d" })
s("DiffText", { bg = colors.blue })
