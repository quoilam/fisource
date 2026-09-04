# fisource
定位: 一款脱敏的, agent友好的凭据管理系统

> 影响范围: 整个 fisource 项目的功能与架构设计
> 最后更新时间: 2026-09-05

# 技术栈
1. rust+cargo —— 核心逻辑(解析、agent 接口、矩阵)
2. fish 函数 —— shell 注入外壳
3. fzf —— 交互选择

# 架构决策

## 注入机制
子进程无法修改父 shell 环境, 因此 fisource 采用「fish 外壳 + Rust 核心」两层架构:

- Rust 二进制只负责**输出 fish 代码**, 不直接改环境
- 用户侧 fish 函数用 `fisource ... | source` 执行这段代码(注意不能用 `eval (fisource ...)`, fish 命令替换会按换行分词、破坏含空格的单引号字面量)
- 关键正确性点: Rust 直接输出 fish 单引号字面量 `set -gx KEY 'value'`, 仅转义 `\` 和 `'`; fish 单引号内 `$` 是字面量、不会二次展开

## 用户 vs agent 区分
靠**命令形态自然分离**, 不做 TTY 探测:

- 用户路径: 裸命令 `fisource`(fzf 选择) / `fisource xx.env`(source)
- agent 路径: `--keys` / `--list` / `--show` 三个 flag

# 功能点
1. 用户输入 `fisource`, 应用自动查询 `~/.fisource` 顶层的 `*.env` 文件(如 app.env k8s.env db.env), 基于 fzf 提供选择下拉框。下拉框项目显示当前 env 文件包含的 key 及 `#description` 描述, 用户选中后 source(默认 fish 语法)。**仅在用户 fzf 预览中允许显示值。**
2. 用户使用 `fisource xx.env`, 正确解析 bash env 语法并 source 到 fish。
3. agent 调用 `fisource --keys` 列举所有 key(不展示值), `fisource --list` 显示所有 env 文件及其描述和 key 列表(不展示值)。二者必须 CLI 友好。
4. `fisource --show` 展示 env 矩阵: 行标签是 env 文件, 行内对每个 key 独立标色——绿=值匹配、红=值不匹配、灰/黄=缺失。用于实时校验当前环境变量是否符合对应文件的预期值。
5. 用户接口 0/少参数、命令式、命令短; agent 接口功能完善、灵活。

# env 文件字段
- `#description`: 描述真实环境变量文件的用途, 例如「用于k8s集群的配置」
- 若干 kv 对, 基于最通用的 bash env 语法

# 解析子集
输入侧**极简解析**, 不做变量引用展开、不做命令替换、不支持多行值(消除 `$()` 注入面):

- 注释 `#`
- `KEY=VALUE`
- 可选 `export` 前缀
- 双引号 / 单引号包裹值

# 脱敏边界
- **值仅允许在用户 fzf 预览中显示**
- `--keys` / `--list` / `--show` 三个接口一律脱敏

# 语义
- **source 语义: 叠加覆盖**。新文件同名 key 覆盖旧值, 旧文件独有 key 残留(凭据会混杂, 由 `--show` 矩阵暴露)。
- **状态追踪: 无**。不在 fzf 下拉框显示「已加载」状态, 状态判定全部交给 `--show` 实时比对。

# Agent 接口格式
输出**纯文本逐行、agent 优先**, 不使用 JSON(flat key 列表用 JSON 过重)。

- `--keys`: 列举所有 key, 每行一个
- `--list`: 带前缀的块结构, 同名 key 不跨文件去重:

```
# file: app.env
# desc: 用于k8s集群的配置
key: DB_HOST
key: DB_USER
# file: db.env
# desc: 数据库连接
key: DB_HOST
```

# CI 与分发
- **平台矩阵**: macOS arm64 + macOS x86_64 + Linux x86_64 + Linux arm64(4 目标)
- **Release 工作流**: 用 cargo-dist, tag 触发后自动完成交叉编译、打包、生成 GitHub Release
- **brew 分发**: 暂缓, 先保证 GitHub Release 可用; cargo-dist 后续可直接生成 formula 预留了路口
- **常规 CI**: PR/push 跑 `cargo fmt --check` + `cargo clippy` + `cargo test` 三项门禁
