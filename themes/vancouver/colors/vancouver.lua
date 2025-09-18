-- themes/vancouver/colors/vancouver.lua

-- Set the colorscheme name
vim.g.colors_name = "vancouver"

-- Get the color palette
local colors = {
	bg = "#0a0f14",
	fg = "#e6e6e6",
	gray = "#4a4a4a",
	red = "#a50f15",
	green = "#66a61e",
	yellow = "#fec44f",
	blue = "#2c5282",
	magenta = "#d95f02",
	cyan = "#00ffff",
	white = "#e6e6e6",
	orange = "#d95f02",
	dark_blue = "#0a0f14",
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
s("DiffAdd", { bg = "#2d4e00" })
s("DiffChange", { bg = "#2c5282" })
s("DiffDelete", { bg = "#a50f15" })
s("DiffText", { bg = "#fec44f" })
