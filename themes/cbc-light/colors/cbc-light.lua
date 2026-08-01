-- themes/cbc-light/colors/cbc-light.lua

-- Set the colorscheme name
vim.g.colors_name = "cbc-light"

-- Get the color palette
local colors = {
  bg = "#FFFFFF",
  fg = "#121212",
  gray = "#E0E0E0",
  red = "#EE0000",
  green = "#2E7D32",
  yellow = "#F9A825",
  blue = "#253D86",
  magenta = "#7B1FA2",
  cyan = "#0097A7",
  white = "#121212",
  orange = "#F9A825",
  dark_blue = "#253D86",
}

-- Helper function to set highlights
local function s(group, opts)
  vim.api.nvim_set_hl(0, group, opts)
end

-- Set the highlight groups
s("Normal", { fg = colors.fg, bg = colors.bg })
s("Comment", { fg = "#757575", italic = true })
s("String", { fg = colors.green })
s("Number", { fg = colors.orange })
s("Keyword", { fg = colors.red, italic = true })
s("Function", { fg = colors.blue })
s("Type", { fg = colors.cyan })
s("Statement", { fg = colors.red })
s("Identifier", { fg = colors.fg })
s("Title", { fg = colors.red, bold = true })
s("LineNr", { fg = "#9E9E9E" })
s("CursorLineNr", { fg = colors.red, bold = true })
s("Visual", { bg = colors.gray })
s("Search", { bg = colors.yellow, fg = colors.bg })
s("IncSearch", { bg = colors.orange, fg = colors.bg })
s("DiffAdd", { bg = "#E8F5E9" })
s("DiffChange", { bg = "#E3F2FD" })
s("DiffDelete", { bg = "#FFEBEE" })
s("DiffText", { bg = "#BBDEFB" })
