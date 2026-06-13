#!/usr/bin/env bash
# shop5 假資料填入腳本
set -e

SHOP5=${SHOP5:-cargo run --}
DB="${SHOP5_DB:-shop5-dev.db}"

if [ -f "$DB" ]; then
    echo "⚠️  資料庫 $DB 已存在，刪除重建..."
    rm -f "$DB"
fi

echo "=== 1. 初始化資料庫 ==="
SHOP5_DB="$DB" $SHOP5 init

echo ""
echo "=== 2. 建立使用者 (10 筆) ==="
SHOP5_DB="$DB" $SHOP5 user add alice    愛麗絲   --role seller --bio "3C 賣家，喜歡最新科技產品"
SHOP5_DB="$DB" $SHOP5 user add bob      鮑勃     --role seller --bio "手作皮件設計師"
SHOP5_DB="$DB" $SHOP5 user add carol    卡蘿     --role seller --bio "天然手作保養品"
SHOP5_DB="$DB" $SHOP5 user add dave     大衛     --role buyer  --bio "喜歡買最新 3C"
SHOP5_DB="$DB" $SHOP5 user add eve      小伊     --role buyer  --bio "瑜珈愛好者"
SHOP5_DB="$DB" $SHOP5 user add frank    法蘭克   --role buyer  --bio "書蟲，一個月讀五本"
SHOP5_DB="$DB" $SHOP5 user add grace    葛蕾絲   --role seller --bio "手作飾品設計師"
SHOP5_DB="$DB" $SHOP5 user add henry    亨利     --role buyer  --bio "健身教練"
SHOP5_DB="$DB" $SHOP5 user add iris     艾瑞絲   --role buyer  --bio "美妝控"
SHOP5_DB="$DB" $SHOP5 user add jack     傑克     --role seller --bio "咖啡豆烘焙師"

echo ""
echo "=== 3. 建立分類 (8 類) ==="
# 分類功能為 v0.4，此處先略過

echo ""
echo "=== 4. 建立商品 (20 筆) ==="
# 愛麗絲的 3C 商品
SHOP5_DB="$DB" $SHOP5 product add 1 "無線藍芽耳機 Pro"         1299 50  --desc "高音質主動降噪，續航 30 小時"
SHOP5_DB="$DB" $SHOP5 product add 1 "USB-C 快充傳輸線 1M"       199 200 --desc "支援 PD 100W"
SHOP5_DB="$DB" $SHOP5 product add 1 "手機磁吸支架"              299 80  --desc "車用/桌面兩用"
SHOP5_DB="$DB" $SHOP5 product add 1 "筆電立架 鋁合金"           899 30  --desc "可折疊收納"
SHOP5_DB="$DB" $SHOP5 product add 1 "機械式鍵盤 青軸"          1990 20  --desc "RGB 背光"

# 鮑勃的手作皮件
SHOP5_DB="$DB" $SHOP5 product add 2 "手工短夾"                  890 15  --desc "義大利植鞣革，使用越久越有味道"
SHOP5_DB="$DB" $SHOP5 product add 2 "鑰匙圈"                    350 30  --desc "手工縫製"
SHOP5_DB="$DB" $SHOP5 product add 2 "手機掛繩"                  590 25  --desc "真皮編織，可調式"

# 卡蘿的保養品
SHOP5_DB="$DB" $SHOP5 product add 3 "玫瑰保濕化妝水"            420 40  --desc "天然玫瑰蒸餾"
SHOP5_DB="$DB" $SHOP5 product add 3 "薰衣草舒眠精油 10ml"       360 35  --desc "有機栽培"
SHOP5_DB="$DB" $SHOP5 product add 3 "乳木果護手霜"              250 50  --desc "深層滋潤"

# 葛蕾絲的手作飾品
SHOP5_DB="$DB" $SHOP5 product add 7 "天然石手鍊"                680 20  --desc "每個都是獨一無二"
SHOP5_DB="$DB" $SHOP5 product add 7 "純銀耳環 簡約款"           990 15  --desc "925 純銀"
SHOP5_DB="$DB" $SHOP5 product add 7 "編織幸運繩手環"            290 40  --desc "多色可選"

# 傑克的咖啡
SHOP5_DB="$DB" $SHOP5 product add 10 "衣索比亞 耶加雪菲 200g"  450 30  --desc "淺焙，花香檸檬調"
SHOP5_DB="$DB" $SHOP5 product add 10 "瓜地馬拉 安提瓜 200g"    420 30  --desc "中焙，巧克力堅果調"
SHOP5_DB="$DB" $SHOP5 product add 10 "巴西 喜拉朵 200g"        380 30  --desc "中深焙，堅果奶油調"
SHOP5_DB="$DB" $SHOP5 product add 10 "濾掛式咖啡綜合包 10入"   350 60  --desc "三種風味各 3-4 包"
SHOP5_DB="$DB" $SHOP5 product add 10 "手沖壺 不鏽鋼 600ml"     890 20  --desc "細口壺，鵝頸設計"

echo ""
echo "=== 5. 購物車 & 訂單 ==="
# 鮑勃買東西
SHOP5_DB="$DB" $SHOP5 cart add 4 1 --quantity 1
SHOP5_DB="$DB" $SHOP5 cart add 4 2 --quantity 3
SHOP5_DB="$DB" $SHOP5 cart add 4 5 --quantity 1
SHOP5_DB="$DB" $SHOP5 order create 4 --note "請幫我包好，謝謝"

# 小伊買保養品
SHOP5_DB="$DB" $SHOP5 cart add 5 9 --quantity 2
SHOP5_DB="$DB" $SHOP5 cart add 5 10 --quantity 1
SHOP5_DB="$DB" $SHOP5 cart add 5 11 --quantity 3
SHOP5_DB="$DB" $SHOP5 order create 5 --note "送禮用，請用禮物包裝"

# 法蘭克買咖啡
SHOP5_DB="$DB" $SHOP5 cart add 6 15 --quantity 2
SHOP5_DB="$DB" $SHOP5 cart add 6 18 --quantity 1
SHOP5_DB="$DB" $SHOP5 order create 6

# 亨利買皮件
SHOP5_DB="$DB" $SHOP5 cart add 8 6 --quantity 1
SHOP5_DB="$DB" $SHOP5 cart add 8 7 --quantity 2
SHOP5_DB="$DB" $SHOP5 order create 8

# 艾瑞絲買飾品 + 咖啡
SHOP5_DB="$DB" $SHOP5 cart add 9 12 --quantity 1
SHOP5_DB="$DB" $SHOP5 cart add 9 14 --quantity 2
SHOP5_DB="$DB" $SHOP5 order create 9

echo ""
echo "=== 6. 更新部分訂單狀態 ==="
SHOP5_DB="$DB" $SHOP5 order update 1 --status paid
SHOP5_DB="$DB" $SHOP5 order update 1 --status shipped
SHOP5_DB="$DB" $SHOP5 order update 1 --status delivered
SHOP5_DB="$DB" $SHOP5 order update 2 --status paid
SHOP5_DB="$DB" $SHOP5 order update 2 --status shipped
SHOP5_DB="$DB" $SHOP5 order update 2 --status delivered

echo ""
echo "=== 假資料填入完成 ==="
echo ""
echo "  使用者: 10 筆（4 賣家 + 6 買家）"
echo "  商品:   20 筆"
echo "  訂單:   5 筆（2 已送達、3 待處理）"
echo ""
echo "啟動互動："
echo "  SHOP5_DB=$DB cargo run -- product list"
echo "  SHOP5_DB=$DB cargo run -- cart list 4"
echo "  SHOP5_DB=$DB cargo run -- order list --buyer-id 4"
echo "  SHOP5_DB=$DB cargo run -- order get 1"
