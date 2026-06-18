-- =============================================================================
-- MatFlow Project Neovim Configuration
-- Optimized for: Rust, Leptos, Tauri (HTML/CSS/Rust)
-- =============================================================================

-- -----------------------------------------------------------------------------
-- 1. 基础编辑器设置 (General Settings)
-- -----------------------------------------------------------------------------
vim.opt.number = true              -- 显示行号
vim.opt.relativenumber = true     -- 显示相对行号 (方便快速跳行)
vim.opt.mouse = 'a'                -- 启用鼠标
vim.opt.ignorecase = true          -- 搜索忽略大小写
vim.opt.smartcase = true           -- 智能大小写
vim.opt.cursorline = true          -- 高亮当前行
vim.opt.termguicolors = true       -- 启用真彩色
vim.opt.scrolloff = 8              -- 保持光标上下距离边缘 8 行

-- 缩进通用配置
vim.opt.expandtab = true            -- 将 Tab 转换为空格
vim.opt.shiftwidth = 4              -- 默认缩进 4 空格
vim.opt.tabstop = 4                -- Tab 宽度 4
vim.opt.softtabstop = 4

-- -----------------------------------------------------------------------------
-- 2. 针对 HTML / Rust 的深度缩进优化 (Deep Indentation Fix)
-- -----------------------------------------------------------------------------
-- 解决 HTML/JSX 缩进太深导致编辑不便的问题
vim.api.nvim_create_autocmd("FileType", {
    pattern = { "html", "rust", "css", "typescript", "javascript" },
    callback = function()
        -- 视觉优化
        vim.opt_local.wrap = true            -- 开启折行
        vim.opt_local.breakindent = true     -- 折行后保持缩进 (关键！)
        vim.opt_local.linebreak = true       -- 在单词边界折行，避免切断单词
        
        -- 折叠配置
        vim.opt_local.foldmethod = "indent"  -- 根据缩进自动折叠
        vim.opt_local.foldlevel = 99         -- 默认展开所有
        
        -- 针对 HTML 适当调整缩进 (如果你决定在这些语言用 2 空格，取消下面注释)
        -- vim.opt_local.shiftwidth = 2
        -- vim.opt_local.tabstop = 2
        -- vim.opt_local.softtabstop = 2
    end,
})

-- -----------------------------------------------------------------------------
-- 3. 快捷键映射 (Keymaps)
-- -----------------------------------------------------------------------------
vim.g.mapleader = " " -- 将空格设为 Leader 键

-- 保存与退出
vim.keymap.set("n", "<leader>w", ":w<CR>", { desc = "Save file" })
vim.keymap.set("n", "<leader>q", ":q<CR>", { desc = "Quit" })

-- 快速折叠操作 (针对深层 HTML)
vim.keymap.set("n", "zC", "zc", { desc = "Close fold" })
vim.keymap.set("n", "zO", "zo", { desc = "Open fold" })
vim.keymap.set("n", "zT", "za", { desc = "Toggle fold" })

-- 快速移动到行首/行尾 (忽略缩进空格)
vim.keymap.set("n", "J", "gj", { desc = "Move down visually" })
vim.keymap.set("n", "K", "gk", { desc = "Move up visually" })

-- -----------------------------------------------------------------------------
-- 4. 插件管理 (建议安装 lazy.nvim)
-- -----------------------------------------------------------------------------
-- 这里预留了插件配置空间，建议安装以下插件来进一步解决缩进问题:
-- 1. nvim-treesitter (语法高亮和智能缩进)
-- 2. rust-analyzer (Rust LSP)
-- 3. vim-minimap (右侧代码小地图)
