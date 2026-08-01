return {
  "scottmckendry/cyberdream.nvim",
  lazy = false,
  priority = 1000,
  config = function()
    require("cyberdream").setup({
      transparent = {{ 'true' if theme.kitty.opacity < 1.0 else 'false' }},
      italic_comments = true,
      hide_fillchars = true,
      borderless_telescope = true,
      terminal_colors = true,
      theme = {
        variant = "{{ 'light' if theme.gtk.color_scheme == 'prefer-light' else 'dark' }}",
        colors = {
          bg = "{{ theme.colors.background }}",
          fg = "{{ theme.colors.foreground }}",
          grey = "{{ theme.colors.color8 }}",
          magenta = "{{ theme.colors.color5 }}",
          cyan = "{{ theme.colors.color6 }}",
          blue = "{{ theme.colors.color4 }}",
          green = "{{ theme.colors.color2 }}",
          yellow = "{{ theme.colors.color3 }}",
          orange = "{{ theme.colors.color1 }}",
          red = "{{ theme.colors.alert }}",
        },
      },
    })
    vim.cmd("colorscheme cyberdream")
  end,
}
