#!/bin/bash
# 評價系統整合測試
set -x

SHOP5="${SHOP5:-cargo run --}"

rm -f shop5.db

# 初始化並建立測試資料
$SHOP5 init
$SHOP5 user add 買家A buyer --bio "愛買東西"
$SHOP5 user add 賣家B seller --bio "專賣好物"
$SHOP5 product add 2 測試商品1 100 5 --desc "第一個商品"
$SHOP5 product add 2 測試商品2 200 3 --desc "第二個商品"

# 建立訂單（從購物車下單）
$SHOP5 cart add 1 1
$SHOP5 cart add 1 2 --quantity 2
$SHOP5 order create 1

# 將訂單標記為已送達
sqlite3 shop5.db "UPDATE orders SET status='delivered' WHERE id=1"
$SHOP5 order get 1 | grep 已送達

# 新增評價
$SHOP5 review add 1 1 1 5 --content "非常好用！"
$SHOP5 review add 1 1 2 3

# 檢視商品評價列表
$SHOP5 review list 1 | grep 非常好用！

# 檢查產品評分
sqlite3 shop5.db "SELECT rating, review_count FROM products WHERE id=1" | grep "5.0|1"
sqlite3 shop5.db "SELECT rating, review_count FROM products WHERE id=2" | grep "3.0|1"

# 檢視單筆評價
$SHOP5 review get 1 | grep ★★★★★

# 刪除評價
$SHOP5 review delete 1
sqlite3 shop5.db "SELECT review_count FROM products WHERE id=1" | grep 0

# 錯誤案例：評分超出範圍
$SHOP5 review add 1 1 1 6 && exit 1

# 建立第二筆訂單（未送達），不應能評價
$SHOP5 cart add 1 1
$SHOP5 order create 1
$SHOP5 review add 2 1 1 4 && exit 1

echo "=== 全部測試通過 ==="
