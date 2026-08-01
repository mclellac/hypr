-- themes/green-monster/colors/green-monster.lua

-- Set the colorscheme name
vim.g.colors_name = "green-monster"

-- Get the color palette
local colors = {
  bg = "#05100D",
  fg = "#D0F4EA",
  gray = "#2A3834",
  red = "#FF5C5C",
  green = "#00FF9F",
  yellow = "#E9FF70",
  blue = "#00D998",
  magenta = "#7CFFCB",
  cyan = "#00CCFF",
  white = "#D0F4EA",
  orange = "#E9FF70",
  dark_blue = "#1F4D43",
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
s("DiffAdd", { bg = "#0A2E22" })
s("DiffChange", { bg = "#142A38" })
s("DiffDelete", { bg = "#3D1A1A" })
s("DiffText", { bg = "#1F4D43" })
