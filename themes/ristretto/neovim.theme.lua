return {
  {
    "gthelding/monokai-pro.nvim",
    config = function()
      require("monokai-pro").setup({
        filter = "ristretto",
        override = function()
          return {
            NonText = { fg = "#7c6c6b" },
            MiniIconsGrey = { fg = "#7c6c6b" },
            MiniIconsRed = { fg = "#e06c75" },
            MiniIconsBlue = { fg = "#6c9bc2" },
            MiniIconsGreen = { fg = "#98c379" },
            MiniIconsYellow = { fg = "#f7c244" },
            MiniIconsOrange = { fg = "#ffaa3b" },
            MiniIconsPurple = { fg = "#c694c8" },
            MiniIconsAzure = { fg = "#c694c8" },
            MiniIconsCyan = { fg = "#75c4b6" }, -- same value as MiniIconsBlue for consistency
          }
        end,
      })
      vim.cmd([[colorscheme monokai-pro]])
    end,
  },
  {
    "LazyVim/LazyVim",
    opts = {
      colorscheme = "monokai-pro",
    },
  },
}

