# v0.9 — 使用者認證系統（登入/登出/註冊）

## 現狀問題

- `users` 表格沒有 `password_hash` 欄位，無法驗證身份
- 後端無任何 session/token 機制
- 前端無登入頁/註冊頁，靠 `users[0]` 偷吃步
- 賣家頁面雖有前端擋，但後端無任何授權檢查（任何人都能叫 `/api/seller/1/orders`）

## 實作範圍

### 1. 資料庫變更

**`src/db.sql`** — 修改 `users` 表格
- 新增 `password_hash TEXT NOT NULL DEFAULT ''`（`register` 時必填，CLI 維持向下相容可填空字串 or 預設值留給 seed）
- 新增 `email    TEXT NOT NULL DEFAULT ''`
- 新增 `sessions` 表格：

```sql
CREATE TABLE IF NOT EXISTS sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id     INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token       TEXT NOT NULL UNIQUE,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at  TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token);
CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
```

**抉擇：** 使用簡單 random token 存於 `sessions` 表格（比 JWT 簡單，不需額外 crate，可 server-side 撤銷）

### 2. 後端依賴新增（Cargo.toml）

- `bcrypt` — 密碼 hashing（輕量、成熟）
- `uuid` — 產生 session token（feature `v4`）
- `tower-http = { features = ["cors"] }` — 已存在

### 3. 後端新增檔案

#### `src/model/auth.rs` — 認證資料層

```
pub fn register(conn, username, display_name, email, password, role) -> Result<User>
pub fn login(conn, username, password) -> Result<Session>    // 回傳 session token
pub fn logout(conn, token) -> Result<()>
pub fn get_user_by_token(conn, token) -> Result<User>        // 驗證 token + 檢查過期
pub fn cleanup_expired(conn) -> Result<()>                   // 清除過期 session
```

#### `src/web/routes/auth.rs` — 認證 API 端點

| 方法 | 路徑 | 說明 | 認證 |
|------|------|------|------|
| POST | `/api/auth/register` | 註冊 | 不用 |
| POST | `/api/auth/login` | 登入，回傳 `{ token, user }` | 不用 |
| POST | `/api/auth/logout` | 登出（刪除 token） | 需要 |
| GET | `/api/auth/me` | 取得目前使用者資訊 | 需要 |

**Login 回傳格式：**
```json
{
  "token": "uuid-string",
  "user": { "id": 1, "username": "alice", ... }
}
```

#### `src/web/middleware.rs`（新增）— 認證中介層

- `AuthUser` extractor：從 `Authorization: Bearer <token>` header 解析使用者
- 可選中介層：對需要認證的路由加上 `require_auth`

#### 既有 API 路由調整

- `GET /api/orders`：只能看自己的訂單（`?buyer_id=` 改為從 token 推斷，但仍可保留 query param 相容）
- 查自己的購物車：`GET /api/cart` 省略 `user_id` 參數，從 token 推斷
- 賣家路由：`GET /api/seller/orders` + `GET /api/seller/products` + `GET /api/seller/stats` — 從 token 拿 seller_id，不用傳 URL

**重點：** 賣家相關路由不應該依賴 URL 參數或路徑中的 seller_id，應直接從 token 推斷目前使用者。這是真正的授權修正。

### 4. 前端新增/修改

#### `web/src/context/AuthContext.tsx`（新增）

- 管理 `token` + `user` 狀態
- 啟動時從 `localStorage` 讀取 token → 呼叫 `GET /api/auth/me` 驗證
- 提供 `login()`、`logout()`、`register()` 方法
- `isAuthenticated` / `isSeller` / `currentUser` 給各頁面使用

#### 取代 `CurrentUserContext`

- `AuthContext` 取代 `CurrentUserContext` 的角色
- 不再有「切換使用者」下拉選單 — 那是假登入的產物
- 登入後 token 存 `localStorage`，頁面重整自動恢復

#### `web/src/pages/Login.tsx`（新增）

- 帳號 + 密碼表單
- 成功後存 token + redirect 到首頁
- 連結到註冊頁

#### `web/src/pages/Register.tsx`（新增）

- 帳號 + 顯示名稱 + email + 密碼 + 確認密碼 + 角色選擇（買家/賣家）
- 成功後自動登入（直接拿 token）或導到登入頁

#### 路由表變更（`App.tsx`）

- 新增 `/login` → `<Login />`
- 新增 `/register` → `<Register />`
- 新增 `ProtectedRoute` wrapper：未登入 → redirect `/login`
- `<Layout />` 不再顯示使用者下拉選單，改為顯示目前使用者名稱 + 登出按鈕

#### 受保護路由（需登入才能存取）

- `/cart` → 需登入
- `/checkout` → 需登入
- `/orders` → 需登入
- `/orders/:id` → 需登入
- `/seller/*` → 需登入 + 賣家角色
- `/users` → 需登入
- `/products/new` → 需登入 + 賣家角色

#### 公開路由（不需登入）

- `/` → 商品列表
- `/products/:id` → 商品詳情
- `/search` → 搜尋
- `/login` → 登入
- `/register` → 註冊

### 5. 前端 API client 變更

`web/src/api/client.ts`：
- 所有 request 自動帶上 `Authorization: Bearer <token>` header
- 401 回應時自動清除 token 並 redirect 到 `/login`
- 新增 `api.auth.login()` / `api.auth.register()` / `api.auth.logout()` / `api.auth.me()` 方法

### 6. CLI 相容性

CLI `shop5 user add` 目前沒有密碼欄位。維持向後相容：
- `--password` / `--email` 選項（選填）
- 填入了就存 hash，沒填就留空（CLI 使用者的帳號無法透過 Web 登入）
- `seed.sh` 的測試使用者都加上密碼（如 `password123`）和 email

### 7. `seed.sh` 變更

- 每個使用者新增 `--email` 和 `--password password123`
- `register` API 端點的回應與 `seed` 腳本配合

### 8. 測試更新

- 修正既有 case 測試：需要登入才能用的 API 端點要先 login 拿 token
- 新增 `case9.sh`：auth 流程 E2E 測試（註冊 → 登入 → 存取保護路由 → 登出 → 無法存取）

### 9. 其他

- `websocket` 或 `stale process` 相關：不需要
- `docs` 更新：計畫、todo

## 實作步驟

| 步驟 | 檔案 | 說明 |
|------|------|------|
| 1 | `Cargo.toml` | 新增 `bcrypt`、`uuid` 依賴 |
| 2 | `src/db.sql` | 修改 `users` 新增 `password_hash`、`email`；新增 `sessions` 表格 |
| 3 | `src/model/auth.rs` | 實作 register/login/logout/get_user_by_token |
| 4 | `src/model/user.rs` | `add()` 支援 `password_hash` 和 `email` 參數 |
| 5 | `src/model/mod.rs` | 匯出 auth module |
| 6 | `src/web/middleware.rs` | 實作 `AuthUser` extractor（從 Bearer token 解析 user） |
| 7 | `src/web/routes/auth.rs` | 實作 register/login/logout/me 端點 |
| 8 | `src/web/routes/mod.rs` | 掛載 auth routes；保護既有路由 |
| 9 | `src/web/routes/order.rs` | 調整為從 token 推斷 buyer_id |
| 10 | `src/web/routes/seller.rs` | 調整為從 token 推斷 seller_id；移除 URL 路徑的 seller_id |
| 11 | `src/web/routes/cart.rs` | 調整為從 token 推斷 user_id |
| 12 | `frontend: AuthContext` | 取代 CurrentUserContext |
| 13 | `frontend: Login + Register 頁` | 新增 |
| 14 | `frontend: ProtectedRoute` | 路由守衛 |
| 15 | `frontend: Layout 更新` | 顯示使用者 + 登出 |
| 16 | `frontend: API client` | Bearer token header + 401 處理 |
| 17 | `seed.sh` | 所有使用者加上密碼/email |
| 18 | `case*.sh` | 修正既有測試 |
| 19 | `case9.sh` | 新增 auth E2E 測試 |
