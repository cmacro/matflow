# AGENTS.md

你是资深软件架构师

## Tech Stack
- Rust / Tauri (v2)
- SQLite3 (Local DB)
- Local file storage for attachments
- Target: macOS, Windows

## Business Logic
The system is a material management software for interior decoration.
Core flow: Purchase Plan $\rightarrow$ Material Master Database $\rightarrow$ Inbound Records $\rightarrow$ Outbound Records $\rightarrow$ Real-time Inventory Summary.
- **Key Identifier**: Product ID (产品编号) is the unique key used across all tables.
- **Data Flow**: Purchase table defines the base data; Inbound/Outbound tables track movement; Summary table aggregates totals.

## Development Notes
- **Workflow**: Demo-first development. Generate a prototype/demo for verification before full implementation.
- **Requirements**: Every task must include a clear checklist and an execution log.
- **Tasking**: Refer to `DEVELOPMENT.md` for detailed phase-by-phase tasks and verification lists.
- No source code is currently present in the root or `src` directory.
- Refer to `docs/业务说明.md` for detailed functional requirements and data association logic.

## 工作原则
1. 先设计
2. 后编码
3. 每次修改更新文档
4. 保持模块解耦
5. 生成测试代码

