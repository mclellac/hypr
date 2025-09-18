-- themes/vancouver/colors/vancouver.lua

-- Set the colorscheme name
vim.g.colors_name = "vancouver"

-- Get the color palette
local colors = {
	bg = "#004170",
	fg = "#ffffff",
	gray = "#ced4d8",
	red = "#F56565",
	green = "#2d4e00",
	yellow = "#F6AD55",
	blue = "#32668c",
	magenta = "#32668c",
	cyan = "#32668c",
	white = "#ffffff",
	orange = "#F6AD55",
	dark_blue = "#004170",
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
