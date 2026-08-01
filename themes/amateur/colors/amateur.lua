-- themes/amateur/colors/amateur.lua

-- Set the colorscheme name
vim.g.colors_name = "amateur"

-- Get the color palette
local colors = {
  bg = "#343F50",
  fg = "#CEE2E4",
  gray = "#4D5566",
  red = "#fc7672",
  green = "#00ff41",
  yellow = "#ffae00",
  blue = "#70e0ff",
  magenta = "#e393f6",
  cyan = "#33ffdd",
  white = "#F8F8F2",
  orange = "#FFFFA5",
  teal = "#00A8A8",
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
s("Visual", { bg = colors.teal, fg = colors.fg })
s("Search", { bg = colors.yellow, fg = colors.bg })
s("IncSearch", { bg = colors.orange, fg = colors.bg })
s("DiffAdd", { bg = colors.green, fg = colors.bg })
s("DiffChange", { bg = colors.yellow, fg = colors.bg })
s("DiffDelete", { bg = colors.red, fg = colors.bg })
s("DiffText", { bg = colors.blue, fg = colors.bg })
