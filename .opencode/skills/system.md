SYSTEM

---

ARCH:
UI → Biz → Data

---

MODULE:
feature-based only
no shared abstraction

---

RULES (KARPATHY CORE):

- think before coding
- simplest solution only
- make minimal changes
- define success first
- no over-engineering

---

BEHAVIOR:

- do not assume missing requirements
- do not add features unless requested
- do not refactor unrelated code
- keep diff minimal

---

CODE RULES:

UI = render only
Biz = logic only
Data = sql only

---

FORBID:

UI → Data
Biz → SQL
unwrap
expect
global refactor
speculative abstraction

---

ERROR:

all functions return Result<T, E>
use anyhow + thiserror

---

STACK:

tokio
sqlx
tauri2
vanilla

---

EXECUTION FLOW:

1. understand feature
2. locate module
3. minimal change design
4. implement Biz
5. implement Data
6. implement UI if needed

---

OUTPUT:

code only
file structure included
no explanation unless asked
