-- themes/vancouver/colors/vancouver.lua

-- Set the colorscheme name
vim.g.colors_name = "vancouver"

-- Get the color palette
local colors = {
	bg = "#2a2a2a",
	fg = "#f0f0f0",
	gray = "#5a5a5a",
	red = "#f04a4a",
	green = "#1a3a1a",
	yellow = "#f0c04a",
	blue = "#5a3a1a",
	magenta = "#f04a4a",
	cyan = "#4af0f0",
	white = "#f0f0f0",
	orange = "#d98c4a",
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
s("DiffAdd", { bg = "#1a3a1a" })
s("DiffChange", { bg = "#5a3a1a" })
s("DiffDelete", { bg = "#f04a4a" })
s("DiffText", { bg = "#f0c04a" })
