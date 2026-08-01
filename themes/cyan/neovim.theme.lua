return {
  "scottmckendry/cyberdream.nvim",
  name = "cyan",
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
          bg = "#274242",
          fg = "#00ffff",
          cyan = "#00ffff",
          blue = "#008b8b", -- Mapping blue to teal to eliminate blue
          green = "#00f2f2",
          yellow = "#00a3a3",
          magenta = "#00ebeb",
          red = "#00e6e6",
          grey = "#008b8b", -- Using secondary teal for grey to improve contrast
        },
      },
    })
    vim.cmd("colorscheme cyberdream")
  end,
}
