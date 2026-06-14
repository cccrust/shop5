import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../context/AuthContext";
import type { Product, Review } from "../types";

export default function ProductDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { user } = useAuth();
  const [product, setProduct] = useState<Product | null>(null);
  const [reviews, setReviews] = useState<Review[]>([]);
  const [loading, setLoading] = useState(true);
  const [qty, setQty] = useState(1);

  useEffect(() => {
    (async () => {
      if (!id) return;
      try {
        const pid = parseInt(id);
        const [p, rv] = await Promise.all([
          api.products.get(pid),
          api.reviews.listByProduct(pid),
        ]);
        setProduct(p);
        setReviews(rv);
      } catch {
        // ignore
      } finally {
        setLoading(false);
      }
    })();
  }, [id]);

  const handleAddToCart = async () => {
    if (!product || !user) return;
    try {
      await api.cart.add(user.id, product.id, qty);
      alert("已加入購物車");
    } catch {
      alert("加入失敗");
    }
  };

  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  const renderStars = (rating: number) => {
    const full = "★".repeat(Math.round(rating));
    const empty = "☆".repeat(5 - Math.round(rating));
    return full + empty;
  };

  if (loading || !product) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button onClick={() => navigate(-1)} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <span className="text-sm text-gray-400">商品詳情</span>
      </div>

      <div className="p-4">
        <div className="w-full aspect-square bg-gray-800 rounded-xl mb-4 flex items-center justify-center text-gray-600">
          商品圖
        </div>

        <h1 className="text-xl font-bold text-white">{product.title}</h1>
        <p className="text-2xl text-blue-400 font-bold mt-2">{formatPrice(product.price)}</p>

        <div className="flex items-center gap-3 mt-3 text-sm text-gray-500">
          <span>庫存 {product.stock}</span>
          <span>已售 {product.sales_count}</span>
          {product.review_count > 0 && (
            <span className="text-yellow-400">{renderStars(product.rating)} {product.rating.toFixed(1)} ({product.review_count})</span>
          )}
        </div>

        {product.description && (
          <p className="text-sm text-gray-400 mt-4">{product.description}</p>
        )}

        <div className="flex items-center gap-3 mt-6">
          <button
            onClick={() => setQty(Math.max(1, qty - 1))}
            className="w-10 h-10 rounded-full bg-gray-800 text-white flex items-center justify-center"
          >
            -
          </button>
          <span className="text-lg font-bold text-white w-8 text-center">{qty}</span>
          <button
            onClick={() => setQty(Math.min(product.stock, qty + 1))}
            className="w-10 h-10 rounded-full bg-gray-800 text-white flex items-center justify-center"
          >
            +
          </button>
          <button
            onClick={handleAddToCart}
            disabled={product.stock === 0}
            className="flex-1 bg-blue-500 text-white rounded-full py-3 text-sm font-bold hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition"
          >
            加入購物車
          </button>
        </div>
      </div>

      {reviews.length > 0 && (
        <div className="border-t border-gray-800">
          <div className="px-4 py-3 text-sm font-bold text-white border-b border-gray-800">
            商品評價 ({reviews.length})
          </div>
          {reviews.map((rv) => (
            <div key={rv.id} className="px-4 py-3 border-b border-gray-800">
              <div className="flex items-center gap-2 text-sm">
                <span className="text-yellow-400">{renderStars(rv.rating)}</span>
                <span className="text-gray-500">{rv.created_at}</span>
              </div>
              {rv.content && <p className="text-sm text-gray-300 mt-1">{rv.content}</p>}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
