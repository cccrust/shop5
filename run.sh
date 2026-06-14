#!/usr/bin/env bash
# 啟動後端 + 前端（開發模式）
set -e

# 清除佔用 port 8080/5173 的舊程序
lsof -ti:8080 2>/dev/null | xargs kill -9 2>/dev/null || true
lsof -ti:5173 2>/dev/null | xargs kill -9 2>/dev/null || true
sleep 0.5

cleanup() {
    echo ""
    echo "正在停止服務..."
    [ -n "$API_PID" ] && kill $API_PID 2>/dev/null
    exit 0
}
trap cleanup SIGINT SIGTERM

#echo "填入假資料..."
#bash seed.sh
#echo ""

echo "啟動 API 伺服器 (port 8080)..."
bash server.sh dev &
API_PID=$!

echo "啟動前端開發伺服器 (port 5173)..."
echo ""
(cd web && npm run dev)

cleanup
