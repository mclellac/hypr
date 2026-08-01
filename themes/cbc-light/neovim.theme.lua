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
          variant = "light",
          colors = {
        bg = "#FFFFFF",
            fg = "#121212",
            red = "#ff4444",
            green = "#00ffcc",
            yellow = "#ffcc00",
            blue = "#00aaff",
            magenta = "#d670d6",
            cyan = "#00e5ff",
            grey = "#253D86",
          },
        },
      })
      vim.cmd("colorscheme cyberdream")
    end,
  },
}