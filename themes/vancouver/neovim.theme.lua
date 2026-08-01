return {
  name = "vancouver",
  dir = vim.fn.stdpath("config"),
  lazy = false,
  priority = 1000,
  config = function()
    -- Load the colorscheme file
    vim.cmd("colorscheme vancouver")
  end,
}