import { useState, useEffect } from "react";
import { useNavigate, Link } from "react-router-dom";
import { api } from "../api/client";
import type { Product } from "../types";

export default function SellerProducts() {
  const navigate = useNavigate();
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [uid, setUid] = useState<number>(0);

  useEffect(() => {
    (async () => {
      try {
        const users = await api.users.list();
        const seller = users.find((u) => u.role === "seller") || users[0];
        if (!seller) return;
        setUid(seller.id);
        const p = await api.seller.products(seller.id);
        setProducts(p);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  const toggleStatus = async (id: number, current: string) => {
    const newStatus = current === "active" ? "inactive" : "active";
    try {
      await api.products.update(id, { status: newStatus });
      setProducts((prev) => prev.map((p) => (p.id === id ? { ...p, status: newStatus } : p)));
    } catch {
      // ignore
    }
  };

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <button onClick={() => navigate(`/users/${uid}`)} className="text-white">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <span className="text-sm text-gray-400">賣家商品管理</span>
        </div>
        <Link
          to="/seller/products/new"
          className="text-xs px-3 py-1.5 rounded bg-orange-500 text-white font-bold"
        >
          + 新增
        </Link>
      </div>

      {products.length === 0 ? (
        <div className="text-center py-20 text-gray-500">暫無商品</div>
      ) : (
        <div>
          {products.map((p) => (
            <div
              key={p.id}
              className="flex items-center gap-3 px-4 py-3 border-b border-gray-800"
            >
              <div
                onClick={() => navigate(`/products/${p.id}`)}
                className="flex-1 min-w-0 cursor-pointer hover:bg-gray-900/50 transition"
              >
                <div className="text-sm font-bold text-white truncate">{p.title}</div>
                <div className="text-sm text-orange-400">NT${p.price.toLocaleString()}</div>
                <div className="text-xs text-gray-600">庫存 {p.stock}</div>
              </div>
              <button
                onClick={() => toggleStatus(p.id, p.status)}
                className={`text-xs px-2 py-1 rounded shrink-0 ${
                  p.status === "active"
                    ? "bg-green-700 text-green-200"
                    : "bg-gray-700 text-gray-400"
                }`}
              >
                {p.status === "active" ? "上架中" : "已下架"}
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
