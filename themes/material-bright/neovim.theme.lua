return {
  "marko-cerovac/material.nvim",
  name = "material",
  lazy = false,
  priority = 1000,
  config = function(_, opts)
    vim.g.material_style = "lighter"
    require("material").setup(opts)
    vim.cmd.colorscheme("material")
  end,
  opts = {
    contrast = {
      terminal = false,
      sidebars = false,
      floating_windows = false,
      cursor_line = false,
      lsp_virtual_text = false,
      non_current_windows = false,
      filetypes = {},
    },
    styles = {
      comments = { italic = false },
      strings = {},
      keywords = {},
      functions = {},
      variables = {},
      operators = {},
      types = {},
    },
    disable = {
      colored_cursor = false,
      borders = false,
      background = false,
      term_colors = false,
      eob_lines = false
    },
    high_visibility = {
      lighter = false,
      darker = false
    },
    lualine_style = "stealth",
    async_loading = true,
    custom_highlights = {},
  },
}