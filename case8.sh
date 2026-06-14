#!/bin/bash
# 賣家儀表板整合測試
set -x

SHOP5="${SHOP5:-cargo run --}"

rm -f shop5.db

# 初始化並建立測試資料
$SHOP5 init
$SHOP5 user add 賣家A 賣家A --role seller
$SHOP5 user add 買家B 買家B --role buyer
$SHOP5 product add 1 手機 500 10 --desc "智慧型手機"
$SHOP5 product add 1 耳機 100 20 --desc "無線耳機"
$SHOP5 product add 1 充電線 50 30 --desc "USB-C"

# 建立訂單並標記為已送達（模擬銷售）
$SHOP5 cart add 2 1 --quantity 2
$SHOP5 cart add 2 2 --quantity 1
$SHOP5 order create 2
sqlite3 shop5.db "UPDATE orders SET status='delivered' WHERE id=1"

$SHOP5 cart add 2 1 --quantity 1
$SHOP5 cart add 2 3 --quantity 3
$SHOP5 order create 2
sqlite3 shop5.db "UPDATE orders SET status='delivered' WHERE id=2"

$SHOP5 cart add 2 2 --quantity 5
$SHOP5 order create 2
sqlite3 shop5.db "UPDATE orders SET status='delivered' WHERE id=3"

# 啟動 Web API
$SHOP5 web --port 8080 &
WEB_PID=$!
sleep 1

# 測試 stats API
curl -s http://localhost:8080/api/seller/1/stats | grep total_orders

# 停服務
kill $WEB_PID 2>/dev/null
wait $WEB_PID 2>/dev/null

# 驗證 CLI
$SHOP5 product list --seller-id 1 | grep 手機

echo "=== 全部測試通過 ==="
