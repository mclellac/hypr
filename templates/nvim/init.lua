-- bootstrap lazy.nvim, LazyVim and your plugins
require("config.lazy")

-- Global Theme Overrides
local function apply_theme_overrides()
  local ok, _ = pcall(function()
    local hl = vim.api.nvim_set_hl

    -- Load all theme colors
    local bg = "{{ theme.colors.background }}"
    local fg = "{{ theme.colors.foreground }}"

    -- Helper to blend colors for guaranteed high-contrast comments and dim text
    local function blend(hex1, hex2, w)
      local function rgb(hex)
        local h = hex:gsub("#", "")
        if #h == 3 then
          h = h:sub(1,1):rep(2) .. h:sub(2,2):rep(2) .. h:sub(3,3):rep(2)
        end
        return tonumber(h:sub(1, 2), 16) or 0, tonumber(h:sub(3, 4), 16) or 0, tonumber(h:sub(5, 6), 16) or 0
      end
      local r1, g1, b1 = rgb(hex1)
      local r2, g2, b2 = rgb(hex2)
      local r = math.floor(r1 * w + r2 * (1 - w))
      local g = math.floor(g1 * w + g2 * (1 - w))
      local b = math.floor(b1 * w + b2 * (1 - w))
      return string.format("#%02x%02x%02x", r, g, b)
    end

    local dim_fg = blend(bg, fg, 0.45) -- 45% bg, 55% fg -> highly readable!
    local comment_fg = blend(bg, fg, 0.55) -- 55% bg, 45% fg -> readable comment color
    local whitespace_fg = blend(bg, fg, 0.85)

    local cursor = "{{ theme.colors.cursor }}"
    local cursor_text = "{{ theme.colors.cursor_text }}"
    local primary = "{{ theme.colors.primary }}"
    local secondary = "{{ theme.colors.secondary }}"
    local accent = "{{ theme.colors.accent }}"
    local alert = "{{ theme.colors.alert }}"
    local warning = "{{ theme.colors.warning }}"
    local success = "{{ theme.colors.success }}"

    local color0 = "{{ theme.colors.color0 }}"
    local color1 = "{{ theme.colors.color1 }}"
    local color2 = "{{ theme.colors.color2 }}"
    local color3 = "{{ theme.colors.color3 }}"
    local color4 = "{{ theme.colors.color4 }}"
    local color5 = "{{ theme.colors.color5 }}"
    local color6 = "{{ theme.colors.color6 }}"
    local color7 = "{{ theme.colors.color7 }}"
    local color8 = "{{ theme.colors.color8 }}"
    local color9 = "{{ theme.colors.color9 }}"
    local color10 = "{{ theme.colors.color10 }}"
    local color11 = "{{ theme.colors.color11 }}"
    local color12 = "{{ theme.colors.color12 }}"
    local color13 = "{{ theme.colors.color13 }}"
    local color14 = "{{ theme.colors.color14 }}"
    local color15 = "{{ theme.colors.color15 }}"

    -- Core UI
    hl(0, "Normal", { bg = bg, fg = fg })
    hl(0, "NormalFloat", { bg = bg, fg = fg })
    hl(0, "FloatBorder", { fg = color0 })
    hl(0, "NormalNC", { bg = bg, fg = fg })
    hl(0, "Directory", { fg = color4, bold = true })
    hl(0, "Cursor", { bg = cursor, fg = cursor_text })
    hl(0, "Visual", { bg = primary, fg = bg })
    hl(0, "CursorLine", { bg = bg })
    hl(0, "CursorColumn", { bg = bg })
    hl(0, "ColorColumn", { bg = bg })
    hl(0, "LineNr", { fg = comment_fg })
    hl(0, "CursorLineNr", { fg = primary, bold = true })
    hl(0, "VertSplit", { fg = color0 })
    hl(0, "WinSeparator", { fg = color0 })
    hl(0, "StatusLine", { bg = color0, fg = fg })
    hl(0, "StatusLineNC", { bg = bg, fg = comment_fg })
    hl(0, "Folded", { bg = color0, fg = comment_fg })
    hl(0, "FoldColumn", { bg = bg, fg = comment_fg })
    hl(0, "SignColumn", { bg = bg })
    hl(0, "Search", { bg = color3, fg = bg })
    hl(0, "IncSearch", { bg = color1, fg = bg })
    hl(0, "MatchParen", { bg = comment_fg, fg = fg, bold = true })
    hl(0, "NonText", { fg = comment_fg })
    hl(0, "Whitespace", { fg = whitespace_fg })
    hl(0, "SpecialKey", { fg = comment_fg })
    hl(0, "Conceal", { fg = comment_fg })
    hl(0, "TabLine", { fg = comment_fg, bg = "none" })
    hl(0, "TabLineFill", { fg = "none", bg = "none" })

    -- Core Syntax
    hl(0, "Comment", { fg = comment_fg, italic = true })
    hl(0, "Constant", { fg = color6 })
    hl(0, "String", { fg = color2 })
    hl(0, "Character", { fg = color2 })
    hl(0, "Number", { fg = color3 })
    hl(0, "Boolean", { fg = color3 })
    hl(0, "Float", { fg = color3 })
    hl(0, "Identifier", { fg = color4 })
    hl(0, "Function", { fg = color5, bold = true })
    hl(0, "Statement", { fg = color1 })
    hl(0, "Conditional", { fg = color1 })
    hl(0, "Repeat", { fg = color1 })
    hl(0, "Label", { fg = color1 })
    hl(0, "Operator", { fg = fg })
    hl(0, "Keyword", { fg = color5, bold = true })
    hl(0, "Exception", { fg = color1 })
    hl(0, "PreProc", { fg = color4 })
    hl(0, "Include", { fg = color4 })
    hl(0, "Define", { fg = color4 })
    hl(0, "Macro", { fg = color4 })
    hl(0, "PreCondit", { fg = color4 })
    hl(0, "Type", { fg = color3 })
    hl(0, "StorageClass", { fg = color3 })
    hl(0, "Structure", { fg = color3 })
    hl(0, "Typedef", { fg = color3 })
    hl(0, "Special", { fg = color6 })
    hl(0, "SpecialChar", { fg = color6 })
    hl(0, "Tag", { fg = color6 })
    hl(0, "Delimiter", { fg = fg })
    hl(0, "SpecialComment", { fg = color8 })
    hl(0, "Debug", { fg = alert })
    hl(0, "Underlined", { underline = true })
    hl(0, "Error", { fg = alert, bold = true })
    hl(0, "Todo", { fg = bg, bg = color3, bold = true })

    -- Treesitter (Explicit links to ensure theme enforces colors)
    hl(0, "@variable", { fg = fg })
    hl(0, "@variable.builtin", { fg = color4 })
    hl(0, "@variable.parameter", { fg = fg })
    hl(0, "@variable.member", { fg = color4 })
    hl(0, "@property", { fg = color4 })
    hl(0, "@attribute", { fg = color4 })
    hl(0, "@constructor", { fg = color6 })
    hl(0, "@constant", { fg = color6 })
    hl(0, "@constant.builtin", { fg = color6 })
    hl(0, "@constant.macro", { fg = color6 })
    hl(0, "@module", { fg = fg })
    hl(0, "@module.builtin", { fg = fg })
    hl(0, "@label", { fg = color1 })
    hl(0, "@string", { fg = color2 })
    hl(0, "@string.regexp", { fg = color2 })
    hl(0, "@string.escape", { fg = color2 })
    hl(0, "@string.special", { fg = color2 })
    hl(0, "@character", { fg = color2 })
    hl(0, "@character.special", { fg = color2 })
    hl(0, "@number", { fg = color3 })
    hl(0, "@boolean", { fg = color3 })
    hl(0, "@float", { fg = color3 })
    hl(0, "@type", { fg = color3 })
    hl(0, "@type.builtin", { fg = color3 })
    hl(0, "@type.definition", { fg = color3 })
    hl(0, "@type.qualifier", { fg = color3 })
    hl(0, "@function", { fg = color5, bold = true })
    hl(0, "@function.builtin", { fg = color5, bold = true })
    hl(0, "@function.macro", { fg = color5, bold = true })
    hl(0, "@function.method", { fg = color5, bold = true })
    hl(0, "@method", { fg = color5, bold = true })
    hl(0, "@parameter", { fg = fg })
    hl(0, "@keyword", { fg = color5, bold = true })
    hl(0, "@keyword.function", { fg = color5, bold = true })
    hl(0, "@keyword.operator", { fg = color5, bold = true })
    hl(0, "@keyword.return", { fg = color5, bold = true })
    hl(0, "@operator", { fg = fg })
    hl(0, "@punctuation.delimiter", { fg = fg })
    hl(0, "@punctuation.bracket", { fg = fg })
    hl(0, "@punctuation.special", { fg = fg })
    hl(0, "@comment", { fg = comment_fg, italic = true })
    hl(0, "@tag", { fg = color6 })
    hl(0, "@tag.attribute", { fg = color4 })
    hl(0, "@tag.delimiter", { fg = fg })
    hl(0, "@text", { fg = fg })

    -- Diagnostics
    hl(0, "DiagnosticError", { fg = alert })
    hl(0, "DiagnosticWarn", { fg = warning })
    hl(0, "DiagnosticInfo", { fg = color4 })
    hl(0, "DiagnosticHint", { fg = success })
    hl(0, "DiagnosticUnderlineError", { undercurl = true, sp = alert })
    hl(0, "DiagnosticUnderlineWarn", { undercurl = true, sp = warning })
    hl(0, "DiagnosticUnderlineInfo", { undercurl = true, sp = color4 })
    hl(0, "DiagnosticUnderlineHint", { undercurl = true, sp = success })

    -- Lsp
    hl(0, "LspReferenceText", { bg = color8 })
    hl(0, "LspReferenceRead", { bg = color8 })
    hl(0, "LspReferenceWrite", { bg = color8 })

    -- Indent Blankline
    hl(0, "IndentBlanklineChar", { fg = color0 })
    hl(0, "IndentBlanklineContextChar", { fg = primary })

    -- Gitsigns
    hl(0, "GitSignsAdd", { fg = success })
    hl(0, "GitSignsChange", { fg = warning })
    hl(0, "GitSignsDelete", { fg = alert })

    -- Telescope
    hl(0, "TelescopeBorder", { fg = color0 })
    hl(0, "TelescopePromptBorder", { fg = primary })
    hl(0, "TelescopeResultsBorder", { fg = color0 })
    hl(0, "TelescopePreviewBorder", { fg = color0 })
    hl(0, "TelescopeSelection", { bg = primary, fg = bg })

    -- Bufferline
    hl(0, "BufferLineIndicatorSelected", { fg = primary })
    hl(0, "BufferLineFill", { bg = color0 })

    -- Flash
    hl(0, "FlashMatch", { bg = color3, fg = bg })
    hl(0, "FlashCurrent", { bg = color1, fg = bg })
    hl(0, "FlashLabel", { bg = color5, fg = bg, bold = true })

    -- Snacks Picker / Explorer
    hl(0, "SnacksPicker", { bg = bg, fg = fg })
    hl(0, "SnacksPickerBorder", { fg = color0 })
    hl(0, "SnacksPickerTitle", { fg = primary, bold = true })
    hl(0, "SnacksPickerSubtitle", { fg = secondary })
    hl(0, "SnacksPickerTotals", { fg = color8 })
    hl(0, "SnacksPickerPrompt", { fg = primary, bold = true })
    hl(0, "SnacksPickerInput", { fg = fg })
    hl(0, "SnacksPickerInputBorder", { fg = primary })
    hl(0, "SnacksPickerListBorder", { fg = color0 })
    hl(0, "SnacksPickerPreviewBorder", { fg = color0 })
    hl(0, "SnacksPickerListCursorLine", { bg = color0 })
    hl(0, "SnacksPickerSelected", { bg = color0, fg = primary })
    hl(0, "SnacksPickerMatch", { fg = primary, bold = true })
    hl(0, "SnacksPickerDir", { fg = color4 })
    hl(0, "SnacksPickerFile", { fg = fg })
    hl(0, "SnacksPickerIcon", { fg = primary })
    hl(0, "SnacksPickerLabel", { fg = secondary })
    hl(0, "SnacksPickerRow", { fg = color8 })
    hl(0, "SnacksPickerCol", { fg = color8 })
    hl(0, "SnacksPickerKeys", { fg = color1 })
    hl(0, "SnacksPickerGitStatusUntracked", { fg = color6 })
    hl(0, "SnacksPickerGitStatusModified", { fg = warning })
    hl(0, "SnacksPickerGitStatusAdded", { fg = success })
    hl(0, "SnacksPickerGitStatusDeleted", { fg = alert })
    hl(0, "SnacksPickerGitStatusRenamed", { fg = color5 })
    hl(0, "SnacksPickerGitStatusUnmerged", { fg = alert, bold = true })
    hl(0, "SnacksPickerGitStatusIgnored", { fg = dim_fg })

    -- Neo-tree / File Explorer Overrides
    hl(0, "NeoTreeDotFiles", { fg = dim_fg })
    hl(0, "NeoTreeDimText", { fg = dim_fg })
    hl(0, "SnacksExplorerFileHidden", { fg = dim_fg })
    hl(0, "SnacksExplorerFileDim", { fg = dim_fg })

    -- Markdown Contrast Overrides
    hl(0, "markdownBlockquote", { fg = color6 })
    hl(0, "markdownCode", { fg = color2 })
    hl(0, "markdownUrl", { fg = color4, underline = true })
    hl(0, "markdownLinkText", { fg = color6 })

    -- Treesitter Markdown overrides
    hl(0, "@markup.raw.markdown_inline", { fg = color2 })
    hl(0, "@markup.link.label.markdown_inline", { fg = color6 })
    hl(0, "@markup.link.url.markdown_inline", { fg = color4, underline = true })
    hl(0, "@markup.list.markdown", { fg = color6 })
  end)

  if not ok then
    -- Silent failure to avoid breaking startup if variables are missing
  end
end

-- Run overrides on startup and whenever colorscheme changes
apply_theme_overrides()

vim.api.nvim_create_autocmd("ColorScheme", {
  callback = function()
    apply_theme_overrides()
  end,
})
