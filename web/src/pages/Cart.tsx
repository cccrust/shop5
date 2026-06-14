import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../context/AuthContext";
import type { CartItemWithProduct } from "../types";

export default function Cart() {
  const navigate = useNavigate();
  const { user } = useAuth();
  const [items, setItems] = useState<CartItemWithProduct[]>([]);
  const [loading, setLoading] = useState(true);

  const load = async () => {
    if (!user) return;
    try {
      const cart = await api.cart.myList();
      setItems(cart);
    } catch {
      // ignore
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(); }, [user]);

  const handleRemove = async (productId: number) => {
    if (!user) return;
    await api.cart.remove(user.id, productId);
    load();
  };

  const handleQtyChange = async (productId: number, newQty: number) => {
    if (!user) return;
    if (newQty <= 0) {
      await handleRemove(productId);
      return;
    }
    await api.cart.myUpdateQty(productId, newQty);
    load();
  };

  const handleCheckout = () => navigate("/checkout");

  const total = items.reduce((sum, i) => sum + i.price * i.quantity, 0);
  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800">
        <h2 className="font-bold text-white">購物車</h2>
      </div>

      {items.length === 0 ? (
        <div className="text-center py-20 text-gray-500">
          <p className="text-lg">購物車是空的</p>
          <p className="text-sm mt-1">前往首頁選購商品</p>
        </div>
      ) : (
        <>
          {items.map((item) => (
            <div key={item.product_id} className="border-b border-gray-800 px-4 py-3">
              <div className="flex items-center gap-3">
                <div className="w-16 h-16 rounded-lg bg-gray-800 flex items-center justify-center text-gray-600 text-xs shrink-0">
                  商品圖
                </div>
                <div className="flex-1 min-w-0">
                  <div className="font-bold text-white text-sm truncate">{item.title}</div>
                  <div className="text-sm text-blue-400">{formatPrice(item.price)}</div>
                  <div className="flex items-center gap-3 mt-2">
                    <button
                      onClick={() => handleQtyChange(item.product_id, item.quantity - 1)}
                      className="w-7 h-7 rounded-full bg-gray-700 text-white flex items-center justify-center text-sm"
                    >
                      -
                    </button>
                    <span className="text-sm font-bold text-white w-6 text-center">{item.quantity}</span>
                    <button
                      onClick={() => handleQtyChange(item.product_id, item.quantity + 1)}
                      className="w-7 h-7 rounded-full bg-gray-700 text-white flex items-center justify-center text-sm"
                    >
                      +
                    </button>
                    <span className="text-xs text-gray-500 ml-2">
                      小計 {formatPrice(item.price * item.quantity)}
                    </span>
                  </div>
                </div>
                <button
                  onClick={() => handleRemove(item.product_id)}
                  className="text-red-500 text-xs shrink-0 px-2"
                >
                  移除
                </button>
              </div>
            </div>
          ))}

          <div className="px-4 py-4 border-t border-gray-800">
            <div className="flex items-center justify-between mb-3">
              <span className="text-white font-bold">合計</span>
              <span className="text-xl font-bold text-blue-400">{formatPrice(total)}</span>
            </div>
            <button
              onClick={handleCheckout}
              className="w-full bg-blue-500 text-white rounded-full py-3 text-sm font-bold hover:bg-blue-600 transition"
            >
              結帳
            </button>
          </div>
        </>
      )}
    </div>
  );
}
