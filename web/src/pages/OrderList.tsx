import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../context/AuthContext";
import type { Order } from "../types";

const STATUS_LABEL: Record<string, string> = {
  pending: "待處理",
  paid: "已付款",
  shipped: "已出貨",
  delivered: "已送達",
  cancelled: "已取消",
};

export default function OrderList() {
  const navigate = useNavigate();
  const { user } = useAuth();
  const [orders, setOrders] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!user) return;
    (async () => {
      try {
        const o = await api.orders.myList();
        setOrders(o);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    })();
  }, [user]);

  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800">
        <h2 className="font-bold text-white">訂單</h2>
      </div>

      {orders.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">尚無訂單</p>
          <p className="text-sm mt-1">前往首頁選購商品</p>
        </div>
      ) : (
        orders.map((o) => (
          <div
            key={o.id}
            onClick={() => navigate(`/orders/${o.id}`)}
            className="border-b border-gray-800 px-4 py-3 hover:bg-gray-900/50 transition cursor-pointer"
          >
            <div className="flex items-center justify-between">
              <div>
                <span className="text-white font-bold text-sm">訂單 #{o.id}</span>
                <span className="text-xs text-gray-500 ml-2">{new Date(o.created_at).toLocaleDateString("zh-TW")}</span>
              </div>
              <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${
                o.status === "delivered" ? "bg-green-900 text-green-400" :
                o.status === "cancelled" ? "bg-red-900 text-red-400" :
                o.status === "pending" ? "bg-yellow-900 text-yellow-400" :
                "bg-blue-900 text-blue-400"
              }`}>
                {STATUS_LABEL[o.status] || o.status}
              </span>
            </div>
            <div className="flex items-center justify-between mt-1">
              <span className="text-sm text-gray-500">{STATUS_LABEL[o.status] || o.status}</span>
              <span className="text-sm font-bold text-white">{formatPrice(o.total)}</span>
            </div>
          </div>
        ))
      )}
    </div>
  );
}
