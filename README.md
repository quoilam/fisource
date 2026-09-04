# fisource

脱敏的、agent 友好的凭据管理系统。

## 安装

```bash
cargo install --git https://github.com/quoilam/fisource
```

或者直接下载预编译二进制（见文末 [下载](#下载)）。

然后把 fish 函数加到 `~/.config/fish/functions/fisource.fish`：

```fish
function fisource
    if test (count $argv) -eq 0
        set -l selected (command fisource --fzf-list | fzf \
            --delimiter '\t' \
            --with-nth 1,2,3 \
            --preview 'command fisource --fzf-preview {1}' \
            --preview-window right:60%)
        if test -n "$selected"
            set -l parts (string split '\t' $selected)
            command fisource $parts[1] | source
        end
    else
        command fisource $argv[1] | source
    end
end
```

## 用法

把 env 文件放到 `~/.fisource/`，用 `#description:` 注释写用途：

```bash
# ~/.fisource/db.env
#description: 数据库连接
DB_HOST=localhost
DB_PASSWORD="p@ss w0rd"
```

- `fisource` —— fzf 交互选择 env 并 source
- `fisource db.env` —— 直接 source 指定文件
- `fisource --keys` —— 列举所有 key（脱敏）
- `fisource --list` —— 列出所有 env 文件及描述和 key（脱敏）
- `fisource --show` —— 校验矩阵，绿=匹配、红=值不匹配、黄=缺失

## 下载

预编译二进制见 [Releases](https://github.com/quoilam/fisource/releases)。
