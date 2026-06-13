import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { User } from "../types";

export default function UserDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [user, setUser] = useState<User | null>(null);
  const [products, setProducts] = useState<{ id: number; title: string; price: number }[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    const uid = parseInt(id);
    Promise.all([
      api.users.get(uid),
      api.products.list({ seller_id: uid }),
    ]).then(([u, p]) => {
      setUser(u);
      setProducts(p);
    }).catch(() => {}).finally(() => setLoading(false));
  }, [id]);

  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  if (loading || !user) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button onClick={() => navigate("/users")} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <span className="text-sm text-gray-400">使用者</span>
      </div>

      <div className="px-4 py-4 border-b border-gray-800">
        <div className="flex items-center gap-4">
          <div className="w-16 h-16 rounded-full bg-gray-700 flex items-center justify-center text-2xl font-bold shrink-0">
            {user.display_name[0]}
          </div>
          <div>
            <h2 className="text-xl font-bold text-white">{user.display_name}</h2>
            <p className="text-sm text-gray-500">@{user.username}</p>
            <span className="text-xs text-gray-600">{user.role === "seller" ? "賣家" : "買家"}</span>
          </div>
        </div>
        {user.bio && <p className="text-sm text-gray-400 mt-3">{user.bio}</p>}
      </div>

      {user.role === "seller" && products.length > 0 && (
        <div>
          <div className="px-4 py-2 border-b border-gray-800 text-sm text-gray-500">
            商品 ({products.length})
          </div>
          {products.map((p) => (
            <div
              key={p.id}
              onClick={() => navigate(`/products/${p.id}`)}
              className="flex items-center gap-3 px-4 py-3 border-b border-gray-800 hover:bg-gray-900/50 transition cursor-pointer"
            >
              <div className="w-12 h-12 rounded-lg bg-gray-800 flex items-center justify-center text-gray-600 text-xs shrink-0">
                商品圖
              </div>
              <div className="flex-1 min-w-0">
                <div className="text-sm font-bold text-white truncate">{p.title}</div>
                <div className="text-sm text-blue-400">{formatPrice(p.price)}</div>
              </div>
            </div>
          ))}
        </div>
      )}

      {user.role === "seller" && products.length === 0 && (
        <div className="text-center py-10 text-gray-500 text-sm">該賣家目前尚無商品</div>
      )}
    </div>
  );
}
