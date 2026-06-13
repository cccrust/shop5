#!/usr/bin/env bash
# shop5 v0.1 CLI 整合測試
set -x

SHOP5=${SHOP5:-cargo run --}
DB=/tmp/shop5-case1.db

rm -f $DB

echo "=== 1. 初始化資料庫 ==="
SHOP5_DB=$DB $SHOP5 init

echo "=== 2. 建立使用者 ==="
SHOP5_DB=$DB $SHOP5 user add alice 愛麗絲 --role seller --bio "3C 賣家"
SHOP5_DB=$DB $SHOP5 user add bob 鮑勃 --role buyer --bio "愛買東西"

echo "=== 3. 列出使用者 ==="
SHOP5_DB=$DB $SHOP5 user list
SHOP5_DB=$DB $SHOP5 user list --search 愛麗

echo "=== 4. 檢視使用者 ==="
SHOP5_DB=$DB $SHOP5 user get 1
SHOP5_DB=$DB $SHOP5 user get 2

echo "=== 5. 更新使用者 ==="
SHOP5_DB=$DB $SHOP5 user update 1 --name "愛麗絲醬" --bio "3C 賣家兼咖啡師"
SHOP5_DB=$DB $SHOP5 user get 1

echo "=== 6. 建立商品 ==="
SHOP5_DB=$DB $SHOP5 product add 1 無線藍芽耳機 599 50 --desc "高音質降噪"
SHOP5_DB=$DB $SHOP5 product add 1 'USB-C 充電線' 199 100
SHOP5_DB=$DB $SHOP5 product add 1 手機保護殼 349 30 --desc "軍規防摔"

echo "=== 7. 列出商品 ==="
SHOP5_DB=$DB $SHOP5 product list
SHOP5_DB=$DB $SHOP5 product list --seller-id 1

echo "=== 8. 檢視商品 ==="
SHOP5_DB=$DB $SHOP5 product get 1
SHOP5_DB=$DB $SHOP5 product get 3

echo "=== 9. 更新商品 ==="
SHOP5_DB=$DB $SHOP5 product update 1 --price 499 --stock 100
SHOP5_DB=$DB $SHOP5 product get 1

echo "=== 10. 購物車操作 ==="
SHOP5_DB=$DB $SHOP5 cart add 2 1 --quantity 2
SHOP5_DB=$DB $SHOP5 cart add 2 2 --quantity 3
SHOP5_DB=$DB $SHOP5 cart list 2
SHOP5_DB=$DB $SHOP5 cart remove 2 2
SHOP5_DB=$DB $SHOP5 cart list 2
SHOP5_DB=$DB $SHOP5 cart add 2 2 --quantity 1
SHOP5_DB=$DB $SHOP5 cart list 2

echo "=== 11. 建立訂單 ==="
SHOP5_DB=$DB $SHOP5 order create 2 --note "請包裝好"
SHOP5_DB=$DB $SHOP5 order get 1

echo "=== 12. 直購 ==="
SHOP5_DB=$DB $SHOP5 order buy 2 1 2 2 --note "急用"
SHOP5_DB=$DB $SHOP5 order get 2

echo "=== 13. 列出訂單 ==="
SHOP5_DB=$DB $SHOP5 order list --buyer-id 2
SHOP5_DB=$DB $SHOP5 order list --seller-id 1

echo "=== 14. 更新訂單狀態 ==="
SHOP5_DB=$DB $SHOP5 order update 1 --status paid
SHOP5_DB=$DB $SHOP5 order update 1 --status shipped
SHOP5_DB=$DB $SHOP5 order update 1 --status delivered
SHOP5_DB=$DB $SHOP5 order get 1

echo "=== 15. 錯誤處理測試 ==="
SHOP5_DB=$DB $SHOP5 product add 999 無此賣家 100 10 2>&1 || true
SHOP5_DB=$DB $SHOP5 cart add 2 999 --quantity 1 2>&1 || true
SHOP5_DB=$DB $SHOP5 order update 999 --status paid 2>&1 || true

echo "=== 完成 ==="
rm -f $DB
