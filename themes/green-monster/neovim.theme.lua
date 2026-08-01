return {
  {
    "scottmckendry/cyberdream.nvim",
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
        bg = "#05100d",
            fg = "#d0f4ea",
            green = "#33FFB2",
            cyan = "#66E0FF",
            blue = "#00F2AA",
            magenta = "#A3FFE0",
            red = "#FF8585",
            yellow = "#F0FF99",
            grey = "#00d998",
          },
        },
      })
      vim.cmd("colorscheme cyberdream")
    end,
  },
}