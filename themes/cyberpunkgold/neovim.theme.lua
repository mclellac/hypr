return {
  "scottmckendry/cyberdream.nvim",
  name = "cyberpunkgold",
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
        bg = "#423300",
          fg = "#ffbb00",
          grey = "#665c00",
          magenta = "#e6b400",
          cyan = "#d69a15",
          blue = "#ffbf00",
          green = "#fcae00",
          yellow = "#ffd500",
          orange = "#ffbb00",
          red = "#ff4444",
        },
      },
    })
    vim.cmd("colorscheme cyberdream")
  end,
}