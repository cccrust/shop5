#!/usr/bin/env bash
# shop5 v0.4 分類與搜尋測試
set -x

SHOP5=${SHOP5:-cargo run --}
DB=/tmp/shop5-case4.db
PORT=18997
BASE="http://127.0.0.1:$PORT/api"

rm -f $DB
SHOP5_DB=$DB $SHOP5 init

# 建立測試資料
SHOP5_DB=$DB $SHOP5 user add alice 愛麗絲 --role seller --bio "3C 賣家"
SHOP5_DB=$DB $SHOP5 user add bob 鮑勃 --role seller --bio "咖啡達人"

echo ""
echo "=== 1. 分類 CLI ==="
SHOP5_DB=$DB $SHOP5 category add "3C"
SHOP5_DB=$DB $SHOP5 category add "食品"
SHOP5_DB=$DB $SHOP5 category add "手機" --parent-id 1

# 列表
SHOP5_DB=$DB $SHOP5 category list | python3 -c "import sys; lines=sys.stdin.read(); assert '3C' in lines; assert '食品' in lines; print('OK 分類列表')"

# 檢視
SHOP5_DB=$DB $SHOP5 category get 1 | python3 -c "import sys; assert '3C' in sys.stdin.read(); print('OK 檢視分類 #1')"

# 刪除
SHOP5_DB=$DB $SHOP5 category delete 3
SHOP5_DB=$DB $SHOP5 category list | python3 -c "import sys; assert '手機' not in sys.stdin.read(); print('OK 刪除分類')"

echo ""
echo "=== 2. 分類 Web API ==="
SHOP5_DB=$DB $SHOP5 web --port $PORT --dev &
PID=$!
sleep 0.5
trap "kill $PID 2>/dev/null; rm -f $DB; wait $PID 2>/dev/null || true" EXIT

# 建立分類
curl -s -X POST "$BASE/categories" -H 'Content-Type: application/json' -d '{"name":"美妝"}' | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['name']=='美妝'; print('OK API 新增分類 id='+str(d['id']))
"
MOM_ID=$(curl -s "$BASE/categories" | python3 -c "import sys,json; d=json.load(sys.stdin); print([c['id'] for c in d if c['name']=='美妝'][0])")
curl -s -X POST "$BASE/categories" -H 'Content-Type: application/json' -d "{\"name\":\"保養品\",\"parent_id\":$MOM_ID}" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert d['parent_id']==$MOM_ID; print('OK API 新增子分類')
"

# 分類列表
curl -s "$BASE/categories" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==4; print('OK API 分類列表 (4 項)')
"

# 刪除分類（美妝）
curl -s -X DELETE "$BASE/categories/$MOM_ID" -o /dev/null -w '%{http_code}' | python3 -c "import sys; assert sys.stdin.read()=='200'; print('OK API 刪除分類')"
curl -s "$BASE/categories" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==3; print('OK API 刪除後剩 3 項')
"

# 404 刪除不存在分類
curl -s -o /dev/null -w '%{http_code}' -X DELETE "$BASE/categories/999" | python3 -c "import sys; assert sys.stdin.read()=='404'; print('OK API 刪除不存在分類 404')"

echo ""
echo "=== 3. 商品搜尋 ==="
# 建立分類
SHOP5_DB=$DB $SHOP5 category add "3C" 2>/dev/null || true
SHOP5_DB=$DB $SHOP5 category add "食品" 2>/dev/null || true

# 建立商品（含分類）
SHOP5_DB=$DB $SHOP5 product add 1 "無線耳機" 599 50 --desc "高音質藍牙" --category-id 1
SHOP5_DB=$DB $SHOP5 product add 1 "充電線" 199 100 --desc "USB-C" --category-id 1
SHOP5_DB=$DB $SHOP5 product add 2 "咖啡豆" 450 30 --desc "衣索比亞" --category-id 2

# 搜尋 API
curl -s "$BASE/products/search?q=耳機" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==1; assert d[0]['title']=='無線耳機'; print('OK 搜尋關鍵字')
"

curl -s "$BASE/products/search?category_id=1" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==2; print('OK 搜尋分類')
"

curl -s "$BASE/products/search?min_price=300&max_price=500" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==1; assert d[0]['title']=='咖啡豆'; print('OK 搜尋價格範圍')
"

curl -s "$BASE/products/search?seller_id=1" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==2; print('OK 搜尋賣家')
"

curl -s "$BASE/products/search?q=不存在" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==0; print('OK 搜尋無結果')
"

echo ""
echo "=== 4. 商品列表分類篩選 ==="
curl -s "$BASE/products?category_id=2" | python3 -c "
import sys,json; d=json.load(sys.stdin); assert len(d)==1; assert d[0]['category_id']==2; print('OK 列表分類篩選')
"

echo ""
echo "=== 5. CLI 搜尋 ==="
SHOP5_DB=$DB $SHOP5 product search --keyword "耳機" | python3 -c "import sys; assert '無線耳機' in sys.stdin.read(); print('OK CLI 搜尋')"

SHOP5_DB=$DB $SHOP5 product search --min-price 400 --max-price 600 | python3 -c "import sys; assert '無線耳機' in sys.stdin.read(); print('OK CLI 價格搜尋')"

SHOP5_DB=$DB $SHOP5 product search --keyword "不存在" | python3 -c "import sys; assert '查無' in sys.stdin.read(); print('OK CLI 無結果')"

echo ""
echo "=== 全部測試通過 ==="
