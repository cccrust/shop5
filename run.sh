#!/usr/bin/env bash
set -e

SHOP5=${SHOP5:-cargo run --}

usage() {
    echo "用法: ./run.sh <mode>"
    echo ""
    echo "模式:"
    echo "  dev        開發模式（API + Vite dev server）"
    echo "  prod       Production 模式（一體伺服）"
    echo "  api        僅 API 伺服器（開發模式）"
    echo "  build      建置前端"
    exit 1
}

if [ $# -lt 1 ]; then
    usage
fi

MODE=$1
shift

case "$MODE" in
    dev)
        echo "開發模式：請在另一個終端機執行 'cd web && npm run dev'"
        echo "API 伺服器啟動於 http://localhost:8080"
        SHOP5_DB="${SHOP5_DB:-shop5.db}" $SHOP5 web --port 8080 --dev
        ;;
    prod)
        echo "Production 模式啟動於 http://localhost:8080"
        SHOP5_DB="${SHOP5_DB:-shop5.db}" $SHOP5 web --port 8080
        ;;
    api)
        echo "API 伺服器啟動於 http://localhost:8080"
        SHOP5_DB="${SHOP5_DB:-shop5.db}" $SHOP5 web --port 8080 --dev
        ;;
    build)
        (cd web && npm run build)
        ;;
    *)
        usage
        ;;
esac
