import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { Product } from "../types";

export default function ProductList() {
  const navigate = useNavigate();
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.products.list().then(setProducts).catch(() => {}).finally(() => setLoading(false));
  }, []);

  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800">
        <h1 className="text-lg font-bold text-white">Shop5</h1>
        <p className="text-xs text-gray-500 mt-0.5">所有商品</p>
      </div>
      {products.length === 0 ? (
        <div className="text-center py-20 text-gray-500">暫無商品</div>
      ) : (
        <div className="grid grid-cols-2 gap-0 divide-x divide-y divide-gray-800">
          {products.map((p) => (
            <div
              key={p.id}
              onClick={() => navigate(`/products/${p.id}`)}
              className="p-4 hover:bg-gray-900/50 transition cursor-pointer"
            >
              <div className="w-full aspect-square bg-gray-800 rounded-lg mb-3 flex items-center justify-center text-gray-600 text-sm">
                商品圖
              </div>
              <h3 className="text-sm font-bold text-white truncate">{p.title}</h3>
              <p className="text-sm text-blue-400 mt-1">{formatPrice(p.price)}</p>
              <p className="text-xs text-gray-600 mt-0.5">庫存 {p.stock}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
