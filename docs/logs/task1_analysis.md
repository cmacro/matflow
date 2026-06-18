# 任务 1: 业务逻辑分析总结
日期: 2026-06-18

## 1. 核心业务流
采购计划 (Purchase Plan) $\rightarrow$ 物料主库 (Material Master) $\rightarrow$ 入库流水 (Inbound) $\rightarrow$ 出库流水 (Outbound) $\rightarrow$ 库存汇总 (Inventory Summary)

## 2. 关键技术需求
- **唯一标识符**: `product_id` 是全系统的核心关联键。
- **数据链路**:
    - 采购计划 $\rightarrow$ 物料主库 (元数据同步)
    - 物料主库 $\rightarrow$ 入/出库表 (信息自动补全)
    - 入/出库表 $\rightarrow$ 库存汇总 (聚合统计: $\sum \text{入} - \sum \text{出}$)
- **强制约束**: 必须先定义物料编号，方可记录流水。

## 3. 数据库设计预案
- `purchases`: 采购计划基础表
- `material_master`: 标准物料字典
- `inbound_logs`: 入库流水表
- `outbound_logs`: 出库流水表
- `inventory_summary`: 通过 SQL 视图 (View) 实时生成

## 4. 功能模块
- 采购管理 (CRUD)
- 主库同步机制
- 入出库快捷录入 (含自动补全)
- 实时库存报表视图
