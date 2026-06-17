# matflow

项目材料管理桌面单机软件，支持MacOS，Win。

## 核心功能
- 采购管理
- 入库流水
- 出库流水
- 库存记录
- 物料信息

详细依据`docs/业务说明.md`文件说明

## 技术方案
- 桌面软件，没有服务器
- [Rust](https://rust-lang.org) 
- [Tauri](https://v2.tauri.app/star)
- 本地数据库SQLite3
- 附件保存本地

```
cargo tauri dev
```
