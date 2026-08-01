return {
  "scottmckendry/cyberdream.nvim",
  name = "cyberpunk",
  lazy = false,
  priority = 1000,
  config = function()
    require("cyberdream").setup({
      transparent = false,
      italic_comments = true,
      hide_fillchars = true,
      borderless_telescope = true,
      terminal_colors = true,
      theme = {
        variant = "dark",
        colors = {
        grey = "#00FF7F",
        },
      },
    })
    vim.cmd("colorscheme cyberdream")
  end,
}