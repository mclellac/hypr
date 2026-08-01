-- themes/cbc-dark/colors/cbc-dark.lua

-- Set the colorscheme name
vim.g.colors_name = "cbc-dark"

-- Get the color palette
local colors = {
  bg = "#121212",
  fg = "#FFFFFF",
  gray = "#494949",
  red = "#EE0000",
  green = "#4CAF50",
  yellow = "#FBC02D",
  blue = "#253D86",
  magenta = "#9C27B0",
  cyan = "#00BCD4",
  white = "#FFFFFF",
  orange = "#FBC02D",
  dark_blue = "#253D86",
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
s("Keyword", { fg = colors.red, italic = true })
s("Function", { fg = colors.blue })
s("Type", { fg = colors.cyan })
s("Statement", { fg = colors.red })
s("Identifier", { fg = colors.fg })
s("Title", { fg = colors.red, bold = true })
s("LineNr", { fg = colors.gray })
s("CursorLineNr", { fg = colors.red, bold = true })
s("Visual", { bg = colors.gray })
s("Search", { bg = colors.yellow, fg = colors.bg })
s("IncSearch", { bg = colors.orange, fg = colors.bg })
s("DiffAdd", { bg = "#1B4232" })
s("DiffChange", { bg = "#2D3748" })
s("DiffDelete", { bg = "#5A2B2B" })
s("DiffText", { bg = "#2C5282" })
