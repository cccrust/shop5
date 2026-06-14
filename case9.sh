#!/usr/bin/env bash
# shop5 v0.9 認證系統端到端測試
set -x

SHOP5=${SHOP5:-cargo run --}
DB=/tmp/shop5-case9.db
PORT=18995
BASE="http://127.0.0.1:$PORT/api"

rm -f $DB
SHOP5_DB=$DB $SHOP5 init

echo ""
echo "=== 1. 建立測試使用者（含 email/password）==="
SHOP5_DB=$DB $SHOP5 user add alice 愛麗絲 --role seller --bio "3C 賣家" --email "alice@test.com" --password "pass123"
SHOP5_DB=$DB $SHOP5 user add bob 鮑勃 --role buyer --bio "愛買東西" --email "bob@test.com" --password "pass456"

echo ""
echo "=== 2. Web API 啟動 ==="
SHOP5_DB=$DB $SHOP5 web --port $PORT --dev &
PID=$!
sleep 0.5
trap "kill $PID 2>/dev/null; rm -f $DB; wait $PID 2>/dev/null || true" EXIT

echo ""
echo "=== 3. 註冊新使用者 ==="
# 註冊
REG=$(curl -s -X POST "$BASE/auth/register" \
    -H 'Content-Type: application/json' \
    -d '{"username":"charlie","display_name":"查理","email":"charlie@test.com","password":"mypass","role":"buyer"}')
echo "$REG" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert 'token' in d, '缺少 token'
assert d['user']['username']=='charlie'
print('OK 註冊成功 token=' + d['token'][:8]+'...')
"
REG_TOKEN=$(echo "$REG" | python3 -c "import sys,json; print(json.load(sys.stdin)['token'])")

echo ""
echo "=== 4. 登入 ==="
# 正確登入
LOGIN=$(curl -s -X POST "$BASE/auth/login" \
    -H 'Content-Type: application/json' \
    -d '{"username":"alice","password":"pass123"}')
echo "$LOGIN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert 'token' in d
assert d['user']['username']=='alice'
print('OK 登入成功 token=' + d['token'][:8]+'...')
"
ALICE_TOKEN=$(echo "$LOGIN" | python3 -c "import sys,json; print(json.load(sys.stdin)['token'])")

# 錯誤密碼
curl -s -X POST "$BASE/auth/login" \
    -H 'Content-Type: application/json' \
    -d '{"username":"alice","password":"wrongpass"}' \
    -o /dev/null -w '%{http_code}' | python3 -c "import sys; assert sys.stdin.read()=='401'; print('OK 錯誤密碼 401')"

# 不存在使用者
curl -s -X POST "$BASE/auth/login" \
    -H 'Content-Type: application/json' \
    -d '{"username":"nobody","password":"x"}' \
    -o /dev/null -w '%{http_code}' | python3 -c "import sys; assert sys.stdin.read()=='401'; print('OK 不存在使用者 401')"

echo ""
echo "=== 5. 取得目前使用者 (GET /auth/me) ==="
curl -s "$BASE/auth/me" -H "Authorization: Bearer $ALICE_TOKEN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert d['username']=='alice'
print('OK 取得目前使用者 alice')
"

# 未授權
curl -s -o /dev/null -w '%{http_code}' "$BASE/auth/me" | python3 -c "import sys; assert sys.stdin.read()=='401'; print('OK 未授權 401')"

echo ""
echo "=== 6. 建立商品給愛麗絲賣 ==="
SHOP5_DB=$DB $SHOP5 product add 1 "無線耳機" 599 50 --desc "高音質"
SHOP5_DB=$DB $SHOP5 product add 1 "充電線" 199 100

echo ""
echo "=== 7. 使用 token 存取購物車 /me 端點 ==="
BOB_LOGIN=$(curl -s -X POST "$BASE/auth/login" \
    -H 'Content-Type: application/json' \
    -d '{"username":"bob","password":"pass456"}')
BOB_TOKEN=$(echo "$BOB_LOGIN" | python3 -c "import sys,json; print(json.load(sys.stdin)['token'])")

# 加入購物車（舊端點，需 user_id）
curl -s -X POST "$BASE/cart" \
    -H 'Content-Type: application/json' \
    -d '{"user_id":2,"product_id":1,"quantity":2}' > /dev/null

# 用 /me 列出購物車
curl -s "$BASE/cart/me" -H "Authorization: Bearer $BOB_TOKEN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert len(d)==1
print('OK 購物車 /me 列出成功 (1 項)')
"

echo ""
echo "=== 8. 使用 token 下單 /me 端點 ==="
ORDER_ME=$(curl -s -X POST "$BASE/orders/me" \
    -H "Authorization: Bearer $BOB_TOKEN" \
    -H 'Content-Type: application/json' \
    -d '{"note":"測試訂單"}')
echo "$ORDER_ME" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert d['order']['status']=='pending'
print('OK 訂單 /me 建立成功')
"

curl -s "$BASE/orders/me" -H "Authorization: Bearer $BOB_TOKEN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert len(d)==1
print('OK 訂單 /me 列表成功')
"

echo ""
echo "=== 9. 賣家 /me 端點 ==="
curl -s "$BASE/seller/me/orders" -H "Authorization: Bearer $ALICE_TOKEN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert len(d)==1
print('OK 賣家訂單 /me')
"

curl -s "$BASE/seller/me/products" -H "Authorization: Bearer $ALICE_TOKEN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert len(d)==2
print('OK 賣家商品 /me')
"

curl -s "$BASE/seller/me/stats" -H "Authorization: Bearer $ALICE_TOKEN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert 'total_orders' in d
print('OK 賣家統計 /me')
"

echo ""
echo "=== 10. 非賣家嘗試存取賣家端點 ==="
curl -s -o /dev/null -w '%{http_code}' "$BASE/seller/me/orders" \
    -H "Authorization: Bearer $BOB_TOKEN" | python3 -c "import sys; assert sys.stdin.read()=='403'; print('OK 非賣家 403')"

echo ""
echo "=== 11. 登出 ==="
curl -s -X POST "$BASE/auth/logout" -H "Authorization: Bearer $ALICE_TOKEN" | python3 -c "
import sys,json; d=json.load(sys.stdin)
assert d['ok']
print('OK 登出成功')
"

# 登出後 token 應失效
curl -s -o /dev/null -w '%{http_code}' "$BASE/auth/me" \
    -H "Authorization: Bearer $ALICE_TOKEN" | python3 -c "import sys; assert sys.stdin.read()=='401'; print('OK 登出後 token 失效 401')"

echo ""
echo "=== 全部測試通過 ==="
