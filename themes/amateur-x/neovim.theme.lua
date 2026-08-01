return {
	name = "amateur-x",
	dir = vim.fn.stdpath("config"),
	lazy = false,
	priority = 1000,
	config = function()
		-- Load the colorscheme file
		vim.cmd("colorscheme amateur-x")
	end,
}

