import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { Order } from "../types";

const STATUS_FLOW = ["pending", "paid", "shipped", "delivered"];

export default function SellerOrders() {
  const navigate = useNavigate();
  const [orders, setOrders] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);
  const [uid, setUid] = useState<number>(0);

  useEffect(() => {
    (async () => {
      try {
        const users = await api.users.list();
        const first = users[0];
        if (!first) return;
        setUid(first.id);
        const seller = first.role === "seller" ? first : users.find((u) => u.role === "seller");
        if (!seller) return;
        setUid(seller.id);
        const o = await api.seller.orders(seller.id);
        setOrders(o);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  const nextStatus = (status: string) => {
    const idx = STATUS_FLOW.indexOf(status);
    return idx < STATUS_FLOW.length - 1 ? STATUS_FLOW[idx + 1] : null;
  };

  const advanceStatus = async (orderId: number, status: string) => {
    const next = nextStatus(status);
    if (!next) return;
    try {
      await api.orders.update(orderId, next);
      setOrders((prev) => prev.map((o) => (o.id === orderId ? { ...o, status: next } : o)));
    } catch {
      // ignore
    }
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button onClick={() => navigate(`/users/${uid}`)} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <span className="text-sm text-gray-400">賣家訂單管理</span>
      </div>

      {orders.length === 0 ? (
        <div className="text-center py-20 text-gray-500">暫無訂單</div>
      ) : (
        <div>
          {orders.map((o) => (
            <div
              key={o.id}
              onClick={() => navigate(`/orders/${o.id}`)}
              className="px-4 py-3 border-b border-gray-800 hover:bg-gray-900/50 transition cursor-pointer"
            >
              <div className="flex items-center justify-between">
                <div>
                  <span className="text-sm font-bold text-white">訂單 #{o.id}</span>
                  <span className="text-xs text-gray-500 ml-2">買家 #{o.buyer_id}</span>
                </div>
                <span className="text-xs px-2 py-0.5 rounded bg-gray-700 text-gray-300">{o.status}</span>
              </div>
              <div className="text-sm text-orange-400 mt-1">NT${o.total.toLocaleString()}</div>
              <div className="text-xs text-gray-600 mt-0.5">{o.created_at}</div>
              {nextStatus(o.status) && (
                <button
                  onClick={(e) => { e.stopPropagation(); advanceStatus(o.id, o.status); }}
                  className="mt-2 text-xs px-3 py-1 rounded bg-blue-600 text-white"
                >
                  設為 {nextStatus(o.status)}
                </button>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
