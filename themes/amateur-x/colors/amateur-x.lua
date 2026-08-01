-- themes/amateur-x/colors/amateur-x.lua

-- Set the colorscheme name
vim.g.colors_name = "amateur-x"

-- Get the color palette
local colors = {
  bg = "#24333a",
  fg = "#b3fff9",
  gray = "#3c4f57",
  red = "#f37f7f",
  green = "#52dca6",
  yellow = "#ffd15c",
  blue = "#2bc4c2",
  magenta = "#cf7cd1",
  cyan = "#2af7dc",
  white = "#d9fffc",
  orange = "#ffe185",
  dark_blue = "#107369",
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
s("DiffAdd", { bg = colors.green, fg = colors.bg })
s("DiffChange", { bg = colors.yellow, fg = colors.bg })
s("DiffDelete", { bg = colors.red, fg = colors.bg })
s("DiffText", { bg = colors.blue, fg = colors.bg })
