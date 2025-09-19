-- themes/vancouver/colors/vancouver.lua

-- Set the colorscheme name
vim.g.colors_name = "vancouver"

-- Get the color palette
local colors = {
	bg = "#2a2a2a",
	fg = "#f0f0f0",
	gray = "#888888",
	red = "#f5b7b1",
	green = "#a3e4d7",
	yellow = "#f9e79f",
	blue = "#a9cce3",
	magenta = "#d7bde2",
	cyan = "#a2d9ce",
	white = "#f0f0f0",
	orange = "#f5cba7",
	dark_blue = "#2a2a2a",
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
s("DiffAdd", { bg = "#a3e4d7" })
s("DiffChange", { bg = "#a9cce3" })
s("DiffDelete", { bg = "#f5b7b1" })
s("DiffText", { bg = "#f9e79f" })
