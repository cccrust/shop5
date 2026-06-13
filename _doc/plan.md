# Shop5 — 網路商店 CLI + Web 應用（規劃草案）

Rust + SQLite 打造的 CLI + Web 網路商店平台，支援多賣家、產品管理、購物車、訂單與評價系統。

## 技術棧

- **語言:** Rust (edition 2021)
- **資料庫:** SQLite (via rusqlite, bundled)
- **CLI 框架:** clap (derive 模式)
- **日期處理:** chrono
- **序列化:** serde / serde_json
- **錯誤處理:** anyhow
- **輸出著色:** colored
- **Web 框架:** axum
- **非同步執行:** tokio
- **前端:** React 19 + TypeScript + Vite 6 + Tailwind CSS v4

## 專案結構

```
shop5/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI 入口 (allow dead_code)
│   ├── db.rs             # 資料庫初始化
│   ├── db.sql            # Schema (include_str!)
│   ├── cli/              # CLI 子指令
│   │   ├── mod.rs
│   │   ├── product.rs
│   │   ├── cart.rs
│   │   ├── order.rs
│   │   └── ...
│   ├── model/            # 資料層 (結構體 + SQL)
│   │   ├── mod.rs
│   │   ├── product.rs
│   │   ├── cart.rs
│   │   ├── order.rs
│   │   └── ...
│   └── web/              # Web API
│       ├── mod.rs
│       ├── error.rs
│       └── routes/
├── web/                  # React 前端
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── api/client.ts
│   │   ├── types/index.ts
│   │   └── pages/
│   ├── package.json
│   ├── vite.config.ts
│   └── tsconfig.json
├── run.sh                # dev/prod/api/build 模式
├── seed.sh               # 假資料腳本
├── case1.sh ~ caseN.sh   # 整合測試
└── _doc/
    ├── plan.md
    ├── v0.1.md
    └── ...
```

---

## 核心資料表

### users
| 欄位 | 型態 | 說明 |
|------|------|------|
| id | INTEGER PK | |
| username | TEXT UNIQUE | 帳號 |
| display_name | TEXT | 顯示名稱 |
| email | TEXT | 電子郵件 |
| role | TEXT DEFAULT 'buyer' | buyer / seller / admin |
| bio | TEXT | 簡介 |
| avatar | TEXT | 頭像 URL |
| created_at | TEXT | |
| updated_at | TEXT | |

### products
| 欄位 | 型態 | 說明 |
|------|------|------|
| id | INTEGER PK | |
| seller_id | INTEGER FK→users | 賣家 |
| title | TEXT | 商品名稱 |
| description | TEXT | 商品描述 |
| price | INTEGER | 價格（以分/元為單位，避免浮點數） |
| stock | INTEGER | 庫存數量 |
| category_id | INTEGER FK→categories | 分類 |
| images | TEXT | JSON 圖片 URL 陣列 |
| status | TEXT DEFAULT 'active' | active / inactive / deleted |
| sales_count | INTEGER DEFAULT 0 | 銷售量（快取） |
| rating | REAL DEFAULT 0 | 平均評價（快取） |
| review_count | INTEGER DEFAULT 0 | 評價數（快取） |
| created_at | TEXT | |
| updated_at | TEXT | |

### categories
| 欄位 | 型態 | 說明 |
|------|------|------|
| id | INTEGER PK | |
| name | TEXT UNIQUE | 分類名稱 |
| parent_id | INTEGER FK→categories | 上層分類（支援多層） |

### cart_items
| 欄位 | 型態 | 說明 |
|------|------|------|
| id | INTEGER PK | |
| user_id | INTEGER FK→users | 買家 |
| product_id | INTEGER FK→products | 商品 |
| quantity | INTEGER DEFAULT 1 | 數量 |
| UNIQUE(user_id, product_id) | | |

### orders
| 欄位 | 型態 | 說明 |
|------|------|------|
| id | INTEGER PK | |
| buyer_id | INTEGER FK→users | 買家 |
| seller_id | INTEGER FK→users | 賣家 |
| status | TEXT DEFAULT 'pending' | pending / paid / shipped / delivered / cancelled |
| total | INTEGER | 總金額（分） |
| note | TEXT | 備註 |
| created_at | TEXT | |
| updated_at | TEXT | |

### order_items
| 欄位 | 型態 | 說明 |
|------|------|------|
| id | INTEGER PK | |
| order_id | INTEGER FK→orders | |
| product_id | INTEGER FK→products | |
| product_title | TEXT | 下單時的商品名稱（快取） |
| product_price | INTEGER | 下單時的單價（快取） |
| quantity | INTEGER | 數量 |
| subtotal | INTEGER | 小計 |

### reviews
| 欄位 | 型態 | 說明 |
|------|------|------|
| id | INTEGER PK | |
| order_id | INTEGER FK→orders | |
| user_id | INTEGER FK→users | 評價者 |
| product_id | INTEGER FK→products | 被評價商品 |
| rating | INTEGER (1-5) | 評分 |
| content | TEXT | 評價內容 |
| created_at | TEXT | |
| UNIQUE(order_id, product_id, user_id) | 一筆訂單一個商品只能評價一次 | |

---

## 版本規劃

| 版本 | 主題 | CLI | Web API | 前端 |
|------|------|-----|---------|------|
| v0.1 | CLI 基礎 | 使用者、商品、購物車、訂單 CRUD | — | — |
| v0.2 | Web API | 維持 | 完整 REST API | — |
| v0.3 | React 前端 | 維持 | 維持 | 商品瀏覽、購物車、訂單 |
| v0.4 | 分類與搜尋 | 分類管理、商品搜尋 | 分類 + 搜尋端點 | 分類瀏覽、搜尋欄 |
| v0.5 | 賣家功能 | 賣家訂單管理、商品上下架 | 賣家 API | 賣家後台（訂單、商品管理） |
| v0.6 | 評價系統 | 評價 CRUD | 評價 API | 商品評價顯示 + 撰寫評價 |
| v0.7 | 完整購物車流程 | 購物車增強 | 結帳 API | 購物車頁面 + 結帳流程 |
| v0.8 | 賣家儀表板 | 銷售統計 | 統計 API | 圖表儀表板 |

---

## v0.1 — CLI 基礎

### 功能範圍

使用者管理、商品管理、購物車管理、訂單管理（純 CLI，無 Web）。

### 資料表 (5 張)

- users（含 role 欄位支援 buyer/seller）
- products（基本商品資訊，不含分類與圖片）
- cart_items（購物車）
- orders（訂單主檔）
- order_items（訂單明細）

### CLI 指令

```
shop5 init                           # 初始化資料庫
shop5 user add <username> <name> [--role] [--bio]
shop5 user list
shop5 user get <id>
shop5 user update <id> [--name] [--bio] [--role]
shop5 user delete <id>

shop5 product add <seller_id> <title> <price> <stock> [--desc]
shop5 product list [--seller-id] [--status]
shop5 product get <id>
shop5 product update <id> [--title] [--price] [--stock] [--status]
shop5 product delete <id>

shop5 cart add <user_id> <product_id> [--quantity]
shop5 cart remove <user_id> <product_id>
shop5 cart list <user_id>
shop5 cart clear <user_id>

shop5 order create <buyer_id>           # 將購物車內容轉成訂單
shop5 order list <user_id>              # 列出使用者的訂單
shop5 order get <id>                    # 訂單詳情（含明細）
shop5 order update <id> --status <s>    # 更新訂單狀態
```

### 單元測試 (model/*.rs)

- product: 新增、查詢、列表、更新庫存
- cart: 加入/移除/列表/清除
- order: 建立訂單、查詢明細、狀態更新

### 測試腳本

- `case1.sh` — CLI 整合測試（使用者 → 產品 → 購物車 → 訂單）

---

## v0.2 — Web API

### 新增技術棧

- axum / tokio / tower-http (CORS)

### RESTful API

| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/users` | 使用者列表 |
| POST | `/api/users` | 建立使用者 |
| GET | `/api/users/{id}` | 使用者詳情 |
| PUT | `/api/users/{id}` | 更新使用者 |
| DELETE | `/api/users/{id}` | 刪除使用者 |
| GET | `/api/products` | 商品列表 |
| POST | `/api/products` | 新增商品 |
| GET | `/api/products/{id}` | 商品詳情 |
| PUT | `/api/products/{id}` | 更新商品 |
| DELETE | `/api/products/{id}` | 刪除商品 |
| GET | `/api/cart/{user_id}` | 購物車列表 |
| POST | `/api/cart` | 加入購物車 |
| DELETE | `/api/cart` | 移除購物車項目 |
| DELETE | `/api/cart/{user_id}` | 清空購物車 |
| GET | `/api/orders` | 訂單列表 (支援 ?buyer_id= / ?seller_id=) |
| POST | `/api/orders` | 建立訂單（從購物車） |
| GET | `/api/orders/{id}` | 訂單詳情 |
| PUT | `/api/orders/{id}` | 更新訂單狀態 |

### 執行模式

```bash
shop5 web --port 8080              # 啟動 API
shop5 web --port 8080 --dev        # 開發模式
```

### 測試腳本

- `case2.sh` — Web API 整合測試

---

## v0.3 — React 前端

### 新增技術棧

- React 19 + TypeScript + Vite 6 + Tailwind CSS v4
- react-router-dom v7

### 前端頁面

| 路由 | 頁面 | 說明 |
|------|------|------|
| `/` | ProductList | 商品列表（所有 active 商品） |
| `/products/:id` | ProductDetail | 商品詳情 + 加入購物車 |
| `/cart` | Cart | 購物車內容（數量調整、結帳按鈕） |
| `/orders` | OrderList | 使用者訂單列表 |
| `/orders/:id` | OrderDetail | 訂單詳情（明細、狀態） |
| `/users` | UserList | 使用者列表 |
| `/users/:id` | UserDetail | 使用者資訊 |
| `/search` | Search | 商品搜尋（預留） |

### Layout

- 底部固定導航列：首頁 / 搜尋 / 購物車 / 訂單 / 使用者
- 購物車圖示顯示數量角標

### 一體化 Production

```bash
./run.sh prod     # API + SPA 同一埠
./run.sh dev      # API + Vite dev server
```

---

## v0.4 — 分類與搜尋

### 資料表新增

- categories（多層分類，parent_id 支援樹狀結構）

### CLI 新增

```
shop5 category add <name> [--parent-id]
shop5 category list
shop5 category get <id>
shop5 category delete <id>
shop5 product search [--keyword] [--category] [--min-price] [--max-price] [--seller] [--sort]
```

### Web API 新增

| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/categories` | 分類列表 |
| POST | `/api/categories` | 新增分類 |
| DELETE | `/api/categories/{id}` | 刪除分類 |
| GET | `/api/products/search?q=&category=&min_price=&max_price=` | 商品搜尋 |

### 前端新增/修改

- Search 頁面實作搜尋欄 + 篩選器
- 分類樹狀瀏覽
- 商品卡顯示分類標籤

---

## v0.5 — 賣家功能

### 功能範圍

- 賣家可以管理自己的商品（上下架）
- 賣家可以查看自己收到的訂單
- 賣家可以更新訂單狀態（出貨、完成）

### Web API 新增

| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/seller/{id}/orders` | 賣家的訂單列表 |
| GET | `/api/seller/{id}/products` | 賣家的商品列表 |

### 前端新增

| 路由 | 頁面 | 說明 |
|------|------|------|
| `/seller/orders` | SellerOrders | 賣家訂單管理 |
| `/seller/products` | SellerProducts | 賣家商品管理（上下架） |
| `/seller/products/new` | ProductEdit | 新增商品表單 |

---

## v0.6 — 評價系統

### 資料表新增

- reviews（訂單完成後可評價）

### CLI 新增

```
shop5 review add <order_id> <user_id> <product_id> <rating> [--content]
shop5 review list <product_id>
shop5 review get <id>
shop5 review delete <id>
```

### Web API 新增

| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/reviews` | 新增評價 |
| GET | `/api/reviews/product/{product_id}` | 商品評價列表 |
| DELETE | `/api/reviews/{id}` | 刪除評價 |
| GET | `/api/reviews/user/{user_id}` | 使用者所有評價 |

### 前端新增

- ProductDetail 顯示平均評分 + 星等 + 評價列表
- 訂單完成後可撰寫評價（評分 1-5 + 文字）
- 使用者頁面顯示收到的評價

---

## v0.7 — 完整購物車流程

### 功能範圍

- 購物車內可調整數量、刪除項目
- 明顯的結帳按鈕，進入結帳頁面
- 結帳頁面顯示訂單摘要、輸入備註
- 送出後顯示訂單確認頁面

### 前端新增/修改

| 路由 | 頁面 | 說明 |
|------|------|------|
| `/checkout` | Checkout | 訂單摘要 + 備註 + 送出 |
| `/order/confirm/:id` | OrderConfirm | 訂單確認頁 |

### Web API 新增

| 方法 | 路徑 | 說明 |
|------|------|------|
| POST | `/api/orders/preview` | 預覽訂單（不扣庫存） |
| PUT | `/api/cart/{user_id}/{product_id}` | 更新購物車數量 |

---

## v0.8 — 賣家儀表板（規劃）

### 功能範圍

- 賣家專屬儀表板，圖表顯示銷售趨勢
- 商品銷售排行
- 每日訂單量統計

### Web API 新增

| 方法 | 路徑 | 說明 |
|------|------|------|
| GET | `/api/seller/{id}/stats/sales` | 銷售統計（日/週/月） |
| GET | `/api/seller/{id}/stats/top-products` | 熱銷商品排行 |

### 前端新增

- Dashboard 頁面，使用純 CSS 圖表（無第三方圖表庫）

---

## seed.sh 假資料規劃

| 項目 | 數量 | 說明 |
|------|------|------|
| 使用者 | 10 | 包含買家與賣家 |
| 分類 | 8 | 3C、服飾、食品、書籍、居家、美妝、運動、其他 |
| 商品 | 30 | 分散於各分類與賣家 |
| 購物車 | 5 | 部分使用者的購物車內容 |
| 訂單 | 10 | 包含各種狀態 |
| 評價 | 8 | 已完成訂單的評價 |

---

## 測試腳本規劃

| 腳本 | 測試範圍 | 預估斷言數 |
|------|---------|-----------|
| case1.sh | CLI 基礎流程（使用者 → 商品 → 購物車 → 訂單） | ~30 |
| case2.sh | Web API 完整測試 | ~40 |
| case3.sh | 前端 + Production E2E（含 SPA 路由） | ~25 |
| case4.sh | 搜尋 + 分類 | ~20 |
| case5.sh | 賣家功能 | ~20 |
| case6.sh | 評價系統 | ~20 |

---

## 寫碼慣例（同 sms4）

- 所有 CLI 輸出、註解為繁體中文
- 錯誤處理使用 `anyhow::Result`
- `#![allow(dead_code, unused)]` 存在於 `src/main.rs` crate root
- `colored` crate 原生支援 `NO_COLOR` 環境變數
- CLI 腳本使用 `SHOP5=${SHOP5:-cargo run --}` env var 覆蓋
- 前端手機優先、黑色主題、最大寬度 600px
