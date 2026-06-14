import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { SellerStats } from "../types";

export default function SellerDashboard() {
  const navigate = useNavigate();
  const [stats, setStats] = useState<SellerStats | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    (async () => {
      try {
        const users = await api.users.list();
        const seller = users.find((u) => u.role === "seller") || users[0];
        if (!seller) return;
        const s = await api.seller.stats(seller.id);
        setStats(s);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  if (loading || !stats) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  const maxRevenue = Math.max(...stats.daily.map((d) => d.revenue), 1);

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button onClick={() => navigate(-1)} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <span className="text-sm text-gray-400">賣家儀表板</span>
      </div>

      <div className="grid grid-cols-3 gap-2 p-4 border-b border-gray-800">
        <div className="bg-gray-800 rounded-xl p-3 text-center">
          <div className="text-2xl font-bold text-white">{stats.total_orders}</div>
          <div className="text-xs text-gray-500 mt-1">訂單數</div>
        </div>
        <div className="bg-gray-800 rounded-xl p-3 text-center">
          <div className="text-lg font-bold text-blue-400">{formatPrice(stats.total_revenue)}</div>
          <div className="text-xs text-gray-500 mt-1">總營收</div>
        </div>
        <div className="bg-gray-800 rounded-xl p-3 text-center">
          <div className="text-lg font-bold text-green-400">{formatPrice(stats.avg_order_value)}</div>
          <div className="text-xs text-gray-500 mt-1">平均客單</div>
        </div>
      </div>

      {stats.daily.length > 0 && (
        <div className="px-4 py-4 border-b border-gray-800">
          <h3 className="text-sm font-bold text-white mb-3">近 30 日營收趨勢</h3>
          <div className="flex items-end gap-1 h-28">
            {stats.daily.slice(0, 14).reverse().map((d) => (
              <div key={d.date} className="flex-1 flex flex-col items-center gap-1">
                <span className="text-[10px] text-gray-500">{formatPrice(d.revenue)}</span>
                <div
                  className="w-full bg-blue-500 rounded-t"
                  style={{ height: `${(d.revenue / maxRevenue) * 100}%`, minHeight: d.revenue > 0 ? "4px" : "0" }}
                />
                <span className="text-[10px] text-gray-600">{d.date.slice(5)}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      <div className="px-4 py-4">
        <h3 className="text-sm font-bold text-white mb-3">熱銷商品</h3>
        {stats.top_products.length === 0 ? (
          <div className="text-center py-10 text-gray-500 text-sm">尚無銷售資料</div>
        ) : (
          <div className="space-y-2">
            {stats.top_products.map((p, i) => (
              <div key={p.id} className="bg-gray-800 rounded-xl p-3 flex items-center gap-3">
                <div className="w-8 h-8 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold text-gray-400 shrink-0">
                  {i + 1}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="text-sm font-bold text-white truncate">{p.title}</div>
                  <div className="text-xs text-gray-500">{formatPrice(p.price)}</div>
                </div>
                <div className="text-right">
                  <div className="text-sm font-bold text-blue-400">{p.sales_count}</div>
                  <div className="text-[10px] text-gray-500">已售</div>
                </div>
                <div className="text-right ml-2">
                  <div className="text-sm font-bold text-green-400">{formatPrice(p.total_revenue)}</div>
                  <div className="text-[10px] text-gray-500">營收</div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
