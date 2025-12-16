-- themes/amateur/colors/amateur.lua

-- Set the colorscheme name
vim.g.colors_name = "amateur"

-- Get the color palette
local colors = {
  bg = "#1A202C",
  fg = "#E2E8F0",
  gray = "#4A5568",
  red = "#F56565",
  green = "#48BB78",
  yellow = "#F6AD55",
  blue = "#63edcd",
  magenta = "#4FD1C5",
  cyan = "#4FD1C5",
  white = "#F7FAFC",
  orange = "#F6AD55",
  dark_blue = "#2C5282",
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
s("Keyword", { fg = colors.blue, italic = true })
s("Function", { fg = colors.yellow })
s("Type", { fg = colors.cyan })
s("Statement", { fg = colors.blue })
s("Identifier", { fg = colors.fg })
s("Title", { fg = colors.blue, bold = true })
s("LineNr", { fg = colors.gray })
s("CursorLineNr", { fg = colors.yellow, bold = true })
s("Visual", { bg = colors.dark_blue })
s("Search", { bg = colors.yellow, fg = colors.bg })
s("IncSearch", { bg = colors.orange, fg = colors.bg })
s("DiffAdd", { bg = "#1B4232" })
s("DiffChange", { bg = "#2D3748" })
s("DiffDelete", { bg = "#5A2B2B" })
s("DiffText", { bg = "#2C5282" })
