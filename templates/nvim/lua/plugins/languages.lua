return {
	{
		"whoissethdaniel/mason-tool-installer.nvim",
		dependencies = { "mason-org/mason.nvim" },
		opts = {
			ensure_installed = {
				"shfmt",
				"shellcheck",
				"prettier",
				"djlint",
				"ruff",
				"yaml-language-server",
				"bash-language-server",
			},
		},
	},
	{
		"neovim/nvim-lspconfig",
		opts = {
			servers = {
				basedpyright = {},
				rust_analyzer = {
					settings = {
						["rust-analyzer"] = {
							checkOnSave = {
								command = "clippy",
							},
						},
					},
				},
				html = {},
				cssls = {},
				jinja_lsp = {},
				bashls = {},
				marksman = {},
				lemminx = {},
				terraformls = {},
				helm_ls = {},
				ansiblels = {},
				yamlls = {
					settings = {
						yaml = {
							schemaStore = {
								enable = true,
								url = "https://www.schemastore.org/api/json/catalog.json",
							},
							schemas = {
								["kubernetes"] = {
									"*-k8s.yaml",
									"*.kubernetes.yaml",
									"*.kube.yaml",
									"*deployment.yaml",
									"*service.yaml",
									"*ingress.yaml",
									"*configmap.yaml",
									"*secret.yaml",
									"*pod.yaml",
									"*.kustomization.yaml",
									"*.cluster.yaml",
									"*.flux.yaml",
									"*.argocd.yaml",
								},
								["https://helm.sh/schemas/helm-template-0.2.0.json"] = "*/templates/*.yaml",
							},
						},
					},
				},
				dockerls = {},
				varnishls = {
					cmd = { vim.fn.expand("~/bin/varnishls") },
					filetypes = { "vcl" },
					on_attach = function(client, _)
						client.server_capabilities.semanticTokensProvider = nil
					end,
				},
			},
		},
	},
	{
		"L3MON4D3/LuaSnip",
		dependencies = {
			"rafamadriz/friendly-snippets",
			"saecki/crates.nvim",
			"chrisgrieser/nvim-various-textobjs",
		},
	},
	{
		"nvim-neotest/neotest",
		dependencies = {
			"nvim-lua/plenary.nvim",
			"nvim-treesitter/nvim-treesitter",
			"antoinemadec/FixCursorHold.nvim",
			"nvim-neotest/neotest-python",
		},
		config = function()
			require("neotest").setup({
				adapters = {
					require("neotest-python")({
						dap = { justMyCode = false },
					}),
				},
			})
		end,
	},
	{
		"nvim-treesitter/nvim-treesitter",
		opts = {
			ensure_installed = {
				"sql",
				"html",
				"css",
				"comment",
				"python",
				"bash",
				"markdown",
				"markdown_inline",
				"regex",
				"vim",
				"yaml",
				"dockerfile",
				"nginx",
				"terraform",
				"helm",
			},
		},
	},
	{
		"mfussenegger/nvim-dap",
		dependencies = {
			"rcarriga/nvim-dap-ui",
			"theHamsta/nvim-dap-virtual-text",
			"nvim-neotest/nvim-nio",
		},
		config = function()
			local dap = require("dap")
			local dapui = require("dapui")

			dap.adapters.python = {
				type = "executable",
				command = "python",
				args = { "-m", "debugpy.adapter" },
			}

			dap.configurations.python = {
				{
					type = "python",
					request = "launch",
					name = "Launch file",
					program = "${file}",
					pythonPath = function()
						return vim.fn.exepath("python")
					end,
				},
			}

			dap.adapters.codelldb = {
				type = "server",
				port = "${port}",
				executable = {
					command = "codelldb",
					args = { "--port", "${port}" },
				},
			}

			dap.configurations.rust = {
				{
					name = "Launch file",
					type = "codelldb",
					request = "launch",
					program = function()
						return vim.fn.input("Path to executable: ", vim.fn.getcwd() .. "/", "file")
					end,
					cwd = "${workspaceFolder}",
					stopOnEntry = false,
				},
			}

			dapui.setup()

			dap.listeners.after.event_initialized["dapui_config"] = function()
				dapui.open()
			end
			dap.listeners.before.event_terminated["dapui_config"] = function()
				dapui.close()
			end
			dap.listeners.before.event_exited["dapui_config"] = function()
				dapui.close()
			end
		end,
	},
	{
		"saecki/crates.nvim",
		event = { "BufRead Cargo.toml" },
		opts = {},
	},
	{
		"mfussenegger/nvim-lint",
		opts = {
			linters_by_ft = {
				python = { "ruff" },
				sh = { "shellcheck" },
				c = { "clang-tidy" },
			},
		},
	},
	{
		"stevearc/conform.nvim",
		opts = {
			formatters_by_ft = {
				python = { "ruff_format" },
				rust = { "rustfmt" },
				sh = { "shfmt" },
			},
			format_on_save = function(bufnr)
				local formatters = require("conform").list_formatters(bufnr)
				if #formatters > 0 then
					return {
						timeout_ms = 500,
						lsp_fallback = true,
					}
				end
			end,
		},
	},
}
