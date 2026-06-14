#!/bin/bash
# 完整購物車流程整合測試
set -x

SHOP5="${SHOP5:-cargo run --}"

rm -f shop5.db

# 初始化資料
$SHOP5 init
$SHOP5 user add 買家A buyer
$SHOP5 user add 賣家B seller
$SHOP5 product add 2 測試商品1 100 5 --desc "第一個商品"
$SHOP5 product add 2 測試商品2 200 3 --desc "第二個商品"

# 測試購物車數量調整 (CLI)
$SHOP5 cart add 1 1 --quantity 2
sqlite3 shop5.db "SELECT quantity FROM cart_items WHERE user_id=1 AND product_id=1" | grep 2

# 啟動 Web API 背景執行
$SHOP5 web --port 8080 &
WEB_PID=$!
sleep 1

# 測試更新數量 API (PUT /cart/{uid}/{pid})
curl -s -X PUT http://localhost:8080/api/cart/1/1 -H "Content-Type: application/json" -d '{"quantity":3}' | grep ok
sqlite3 shop5.db "SELECT quantity FROM cart_items WHERE user_id=1 AND product_id=1" | grep 3

# 加入第二項商品
curl -s -X POST http://localhost:8080/api/cart -H "Content-Type: application/json" -d '{"user_id":1,"product_id":2,"quantity":1}' | grep ok

# 測試預覽訂單
curl -s -X POST http://localhost:8080/api/orders/preview -H "Content-Type: application/json" -d '{"buyer_id":1}' | grep seller_id

# 測試結帳 API
curl -s -X POST http://localhost:8080/api/orders -H "Content-Type: application/json" -d '{"buyer_id":1,"note":"請快遞"}' | grep order

# 停止 Web API
kill $WEB_PID 2>/dev/null
wait $WEB_PID 2>/dev/null

# 驗證訂單已建立（含備註）
$SHOP5 order get 1 | grep 請快遞

# 清空購物車
$SHOP5 cart clear 1
$SHOP5 cart list 1 | grep 空的

echo "=== 全部測試通過 ==="
