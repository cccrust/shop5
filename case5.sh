#!/usr/bin/env bash
# shop5 v0.5 賣家功能測試
set -x

SHOP5=${SHOP5:-cargo run --}
DB=/tmp/shop5-case5.db
PORT=18996
BASE="http://127.0.0.1:$PORT/api"

rm -f $DB
SHOP5_DB=$DB $SHOP5 init

echo ""
echo "=== 1. 建立測試資料 ==="
SHOP5_DB=$DB $SHOP5 user add alice 愛麗絲 --role seller --bio "3C 賣家"
SHOP5_DB=$DB $SHOP5 user add bob 鮑勃 --role buyer --bio "愛買東西"
SHOP5_DB=$DB $SHOP5 category add "3C"
SHOP5_DB=$DB $SHOP5 product add 1 "無線耳機" 599 50 --desc "高音質" --category-id 1
SHOP5_DB=$DB $SHOP5 product add 1 "充電線" 199 100 --desc "USB-C" --category-id 1
SHOP5_DB=$DB $SHOP5 product add 1 "手機殼" 349 30 --desc "軍規" --category-id 1

echo ""
echo "=== 2. 賣家 API ==="
SHOP5_DB=$DB $SHOP5 web --port $PORT --dev &
PID=$!
sleep 0.5
trap "kill $PID 2>/dev/null; rm -f $DB; wait $PID 2>/dev/null || true" EXIT

# 賣家訂單 API
curl -s "$BASE/seller/1/orders" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==0; print('OK 賣家暫無訂單')
"

# 賣家商品 API
curl -s "$BASE/seller/1/products" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==3; print('OK 賣家有 3 項商品')
"

# 買家下單
curl -s -X POST "$BASE/cart" -H 'Content-Type: application/json' -d '{"user_id":2,"product_id":1,"quantity":2}' > /dev/null
curl -s -X POST "$BASE/cart" -H 'Content-Type: application/json' -d '{"user_id":2,"product_id":2,"quantity":1}' > /dev/null
curl -s -X POST "$BASE/orders" -H 'Content-Type: application/json' -d '{"buyer_id":2,"note":"測試"}' > /dev/null

# 再檢查賣家訂單
curl -s "$BASE/seller/1/orders" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==1; print('OK 賣家收到 1 筆訂單')
"

echo ""
echo "=== 3. 商品上下架 ==="
# 下架
curl -s -X PUT "$BASE/products/1" -H 'Content-Type: application/json' -d '{"status":"inactive"}' | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['status']=='inactive'; print('OK 商品 #1 已下架')
"

# 確認下架後不在一般列表
curl -s "$BASE/products?status=active" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==2; print('OK 商品列表只剩 2 項 active')
"

# 上架
curl -s -X PUT "$BASE/products/1" -H 'Content-Type: application/json' -d '{"status":"active"}' | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['status']=='active'; print('OK 商品 #1 已重新上架')
"

echo ""
echo "=== 4. 賣家更新訂單狀態 ==="
curl -s -X PUT "$BASE/orders/1" -H 'Content-Type: application/json' -d '{"status":"paid"}' | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['status']=='paid'; print('OK 訂單已更新為 paid')
"
curl -s -X PUT "$BASE/orders/1" -H 'Content-Type: application/json' -d '{"status":"shipped"}' | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['status']=='shipped'; print('OK 訂單已更新為 shipped')
"
curl -s -X PUT "$BASE/orders/1" -H 'Content-Type: application/json' -d '{"status":"delivered"}' | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['status']=='delivered'; print('OK 訂單已更新為 delivered')
"

echo ""
echo "=== 5. 賣家建立商品（API）==="
curl -s -X POST "$BASE/products" -H 'Content-Type: application/json' -d '{"seller_id":1,"title":"新商品","price":999,"stock":10,"description":"測試","category_id":1}' | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['title']=='新商品'; print('OK API 建立新商品')
"

curl -s "$BASE/seller/1/products" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==4; print('OK 賣家現在有 4 項商品')
"

echo ""
echo "=== 全部測試通過 ==="
