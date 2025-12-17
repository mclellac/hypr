-- themes/amateur/colors/amateur.lua

-- Set the colorscheme name
vim.g.colors_name = "amateur"

-- Get the color palette
local colors = {
  bg = "#012e21",
  fg = "#e2e8f0",
  gray = "#4a5568",
  red = "#f56565",
  green = "#04c755",
  yellow = "#ff9d26",
  blue = "#63ed91",
  magenta = "#4fd19d",
  cyan = "#4fd181",
  white = "#f7fafc",
  orange = "#d68700",
  dark_blue = "#2c5282",
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
s("DiffAdd", { bg = "#226449" })
s("DiffChange", { bg = "#696b3e" })
s("DiffDelete", { bg = "#7d2929" })
s("DiffText", { bg = "#2c5282" })
