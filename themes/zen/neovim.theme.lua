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
            bg = "#e6e4da",
            fg = "#2e2e2e",
            grey = "#2e2e2e",
            magenta = "#f76f53",
            cyan = "#f76f53",
            blue = "#f76f53",
            green = "#f76f53",
            yellow = "#f76f53",
            orange = "#f76f53",
            red = "#f76f53",
          },
          highlights = {
            Visual = { bg = "#f76f53", fg = "#e6e4da" },
            CursorLine = { bg = "#e6e4da" },
            CursorColumn = { bg = "#e6e4da" },
            ColorColumn = { bg = "#e6e4da" },
          },
        },
      })
      vim.cmd("colorscheme cyberdream")
    end,
  },
}
