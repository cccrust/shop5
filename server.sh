#!/usr/bin/env bash
set -e

SHOP5=${SHOP5:-cargo run --}

usage() {
    echo "用法: ./server.sh <mode>"
    echo ""
    echo "模式:"
    echo "  dev        開發模式（僅 API，前端用 npm run dev）"
    echo "  prod       Production 模式（一體伺服）"
    echo "  api        僅 API 伺服器（同 dev）"
    echo "  seed       填入假資料（清除原有資料庫）"
    echo "  build      建置前端"
    echo "  test       執行所有測試（cargo test + case1-8）"
    exit 1
}

if [ $# -lt 1 ]; then
    usage
fi

MODE=$1
shift

case "$MODE" in
    dev|api)
        SHOP5_DB="${SHOP5_DB:-shop5.db}"
        sqlite3 "$SHOP5_DB" "SELECT 1 FROM products LIMIT 1" 2>/dev/null || {
            echo "⚠️  資料庫未初始化，自動執行 init..."
            SHOP5_DB="$SHOP5_DB" $SHOP5 init
        }
        echo "API 伺服器啟動於 http://localhost:8080"
        SHOP5_DB="$SHOP5_DB" $SHOP5 web --port 8080 --dev
        ;;
    prod)
        SHOP5_DB="${SHOP5_DB:-shop5.db}"
        sqlite3 "$SHOP5_DB" "SELECT 1 FROM products LIMIT 1" 2>/dev/null || {
            echo "⚠️  資料庫未初始化，自動執行 init..."
            SHOP5_DB="$SHOP5_DB" $SHOP5 init
        }
        echo "Production 模式啟動於 http://localhost:8080"
        SHOP5_DB="$SHOP5_DB" $SHOP5 web --port 8080
        ;;
    seed)
        bash seed.sh
        ;;
    build)
        (cd web && npm run build)
        ;;
    test)
        cargo test 2>&1
        echo "---"
        for i in {1..8}; do
            echo "=== case$i ==="
            rm -f shop5.db
            bash case$i.sh 2>&1 | grep -E "完成|通過|Error" | tail -1
        done
        rm -f shop5.db
        ;;
    *)
        usage
        ;;
esac
