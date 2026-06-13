#!/usr/bin/env bash
# shop5 v0.3 前端 + Production 端到端測試
set -x

SHOP5=${SHOP5:-cargo run --}
DB=/tmp/shop5-case3.db
PORT=18998
BASE="http://127.0.0.1:$PORT"

# 確保前端已建置
if [ ! -d "web/dist" ]; then
    echo "請先執行 npm run build"
    exit 1
fi

rm -f $DB

# 初始化資料庫 + 建立測試資料
SHOP5_DB=$DB $SHOP5 init
SHOP5_DB=$DB $SHOP5 user add alice 愛麗絲 --role seller --bio "3C 賣家"
SHOP5_DB=$DB $SHOP5 user add bob 鮑勃 --role buyer --bio "愛買東西"
SHOP5_DB=$DB $SHOP5 product add 1 "無線藍芽耳機" 599 50 --desc "高音質降噪"
SHOP5_DB=$DB $SHOP5 product add 1 "USB-C 充電線" 199 100

# Production 模式啟動
SHOP5_DB=$DB $SHOP5 web --port $PORT &
PID=$!
sleep 0.5

cleanup() {
    kill $PID 2>/dev/null || true
    rm -f $DB
    wait $PID 2>/dev/null || true
}
trap cleanup EXIT

echo "=== 1. SPA 路由測試 ==="

# 首頁
curl -s "$BASE/" | python3 -c "
import sys
html = sys.stdin.read()
assert 'Shop5' in html or '<div id=\"root\">' in html
print('OK 首頁 index.html')
"

# SPA fallback（/users 路由）
curl -s "$BASE/users" | python3 -c "
import sys; html=sys.stdin.read()
assert '<div id=\"root\">' in html
print('OK SPA fallback /users')
"

# SPA fallback（/cart）
curl -s "$BASE/cart" | python3 -c "
import sys; html=sys.stdin.read()
assert '<div id=\"root\">' in html
print('OK SPA fallback /cart')
"

# SPA fallback（/orders）
curl -s "$BASE/orders" | python3 -c "
import sys; html=sys.stdin.read()
assert '<div id=\"root\">' in html
print('OK SPA fallback /orders')
"

# SPA fallback（深層路徑）
curl -s "$BASE/products/1" | python3 -c "
import sys; html=sys.stdin.read()
assert '<div id=\"root\">' in html
print('OK SPA fallback /products/1')
"

echo ""
echo "=== 2. 靜態資源測試 ==="
JS_FILE=$(ls web/dist/assets/index-*.js 2>/dev/null | head -1 | xargs basename)
curl -s -o /dev/null -w '%{http_code}' "$BASE/assets/$JS_FILE" | python3 -c "import sys; assert sys.stdin.read()=='200'; print('OK 靜態資源可訪問')"

# 404 測試
curl -s -o /dev/null -w '%{http_code}' "$BASE/assets/nonexistent.js" | python3 -c "import sys; assert sys.stdin.read()=='404'; print('OK 不存在的資源回傳 404')"

echo ""
echo "=== 3. API 端點測試（Production 模式）==="
curl -s "$BASE/api/users" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==2; print('OK API users')"

curl -s "$BASE/api/products" | python3 -c "import sys,json; d=json.load(sys.stdin); assert len(d)==2; print('OK API products')"

# 建立訂單
curl -s -X POST "$BASE/api/cart" -H 'Content-Type: application/json' -d '{"user_id":2,"product_id":1,"quantity":1}' | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['ok']; print('OK 加入購物車')"

curl -s -X POST "$BASE/api/orders" -H 'Content-Type: application/json' -d '{"buyer_id":2,"note":"測試"}' | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert d['order']['status']=='pending'
print('OK 建立訂單')
"

echo ""
echo "=== 全部測試通過 ==="
