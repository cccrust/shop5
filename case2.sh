#!/usr/bin/env bash
# shop5 v0.2 Web API 整合測試
set -x

SHOP5=${SHOP5:-cargo run --}
DB=/tmp/shop5-case2.db
PORT=18999
BASE="http://127.0.0.1:$PORT/api"

rm -f $DB

# 初始化資料庫
SHOP5_DB=$DB $SHOP5 init

# 在背景啟動 API 伺服器
SHOP5_DB=$DB $SHOP5 web --port $PORT --dev &
PID=$!
sleep 0.5

cleanup() {
    kill $PID 2>/dev/null || true
    rm -f $DB
    wait $PID 2>/dev/null || true
}
trap cleanup EXIT

echo "=== 1. 使用者 API ==="

# 建立使用者
USER1=$(curl -s -X POST "$BASE/users" \
    -H 'Content-Type: application/json' \
    -d '{"username":"alice","display_name":"愛麗絲","role":"seller","bio":"3C 賣家"}')
echo "$USER1" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['id']==1; assert d['username']=='alice'; print('OK 建立使用者 #1')"

USER2=$(curl -s -X POST "$BASE/users" \
    -H 'Content-Type: application/json' \
    -d '{"username":"bob","display_name":"鮑勃","role":"buyer","bio":"愛買東西"}')
echo "$USER2" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['id']==2; print('OK 建立使用者 #2')"

# 列出使用者
curl -s "$BASE/users" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==2; print('OK 列出使用者')"

# 搜尋使用者
curl -s "$BASE/users?search=愛麗" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==1; print('OK 搜尋使用者')"

# 取得單一使用者
curl -s "$BASE/users/1" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['username']=='alice'; print('OK 取得使用者 #1')"

# 更新使用者
curl -s -X PUT "$BASE/users/1" \
    -H 'Content-Type: application/json' \
    -d '{"display_name":"愛麗絲醬"}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['display_name']=='愛麗絲醬'; print('OK 更新使用者')"

echo ""
echo "=== 2. 商品 API ==="

# 建立商品
P1=$(curl -s -X POST "$BASE/products" \
    -H 'Content-Type: application/json' \
    -d '{"seller_id":1,"title":"無線藍芽耳機","price":599,"stock":50,"description":"高音質降噪"}')
echo "$P1" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['id']==1; print('OK 建立商品 #1')"

P2=$(curl -s -X POST "$BASE/products" \
    -H 'Content-Type: application/json' \
    -d '{"seller_id":1,"title":"USB-C 充電線","price":199,"stock":100}')
echo "$P2" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['id']==2; print('OK 建立商品 #2')"

# 列出商品
curl -s "$BASE/products" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==2; print('OK 列出商品')"

# 單一商品
curl -s "$BASE/products/1" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['price']==599; print('OK 取得商品 #1')"

# 更新商品
curl -s -X PUT "$BASE/products/1" \
    -H 'Content-Type: application/json' \
    -d '{"price":499,"stock":100}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['price']==499; print('OK 更新商品')"

echo ""
echo "=== 3. 購物車 API ==="

# 加入購物車
curl -s -X POST "$BASE/cart" \
    -H 'Content-Type: application/json' \
    -d '{"user_id":2,"product_id":1,"quantity":2}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['ok']; print('OK 加入購物車')"

curl -s -X POST "$BASE/cart" \
    -H 'Content-Type: application/json' \
    -d '{"user_id":2,"product_id":2,"quantity":3}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['ok']; print('OK 加入購物車 #2')"

# 列出購物車
CART=$(curl -s "$BASE/cart/2")
echo "$CART" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==2; assert d[0]['quantity']==2; print('OK 列出購物車 (2項)')"

# 移除購物車
curl -s -X DELETE "$BASE/cart" \
    -H 'Content-Type: application/json' \
    -d '{"user_id":2,"product_id":2}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['ok']; print('OK 移除購物車')"

CART2=$(curl -s "$BASE/cart/2")
echo "$CART2" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==1; print('OK 只剩 1 項')"

echo ""
echo "=== 4. 訂單 API ==="

# 建立訂單
ORDER1=$(curl -s -X POST "$BASE/orders" \
    -H 'Content-Type: application/json' \
    -d '{"buyer_id":2,"note":"請包裝好"}')
echo "$ORDER1" | python3 -c "
import sys,json
d=json.load(sys.stdin)
assert d['order']['status']=='pending'
assert d['order']['total']==998
assert len(d['items'])==1
print('OK 建立訂單 #1 (NT\$998)')
"

# 列出訂單
curl -s "$BASE/orders?buyer_id=2" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==1; print('OK 列出買家訂單')"

# 取得訂單詳情
curl -s "$BASE/orders/1" | python3 -c "
import sys,json
d=json.load(sys.stdin)
assert d['order']['status']=='pending'
assert len(d['items'])==1
assert d['items'][0]['product_title']=='無線藍芽耳機'
print('OK 取得訂單 #1 詳情')
"

# 更新訂單狀態
curl -s -X PUT "$BASE/orders/1" \
    -H 'Content-Type: application/json' \
    -d '{"status":"paid"}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['status']=='paid'; print('OK 更新訂單為 paid')"

curl -s -X PUT "$BASE/orders/1" \
    -H 'Content-Type: application/json' \
    -d '{"status":"delivered"}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['status']=='delivered'; print('OK 更新訂單為 delivered')"

echo ""
echo "=== 5. 錯誤處理 ==="

# 404 不存在使用者
curl -s -o /dev/null -w '%{http_code}' "$BASE/users/999" | python3 -c "import sys; assert sys.stdin.read()=='404'; print('OK 404 不存在使用者')"

# 404 不存在商品
curl -s -o /dev/null -w '%{http_code}' "$BASE/products/999" | python3 -c "import sys; assert sys.stdin.read()=='404'; print('OK 404 不存在商品')"

# 400 參數錯誤（商品價格負數）
curl -s -X POST "$BASE/products" \
    -H 'Content-Type: application/json' \
    -d '{"seller_id":1,"title":"壞商品","price":-1,"stock":10}' \
    -o /dev/null -w '%{http_code}' | python3 -c "import sys; assert sys.stdin.read()=='400'; print('OK 400 價格錯誤')"

echo ""
echo "=== 全部測試通過 ==="
