import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { CartPreview } from "../types";

export default function Checkout() {
  const navigate = useNavigate();
  const [preview, setPreview] = useState<CartPreview | null>(null);
  const [loading, setLoading] = useState(true);
  const [userId, setUserId] = useState(0);
  const [note, setNote] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        const users = await api.users.list();
        if (!users || users.length === 0) return;
        const uid = users[0]!.id;
        setUserId(uid);
        const p = await api.orders.preview(uid);
        setPreview(p);
      } catch (e: any) {
        alert(e.message);
        navigate("/cart");
      } finally {
        setLoading(false);
      }
    })();
  }, [navigate]);

  const handleSubmit = async () => {
    if (!preview) return;
    setSubmitting(true);
    try {
      const result = await api.orders.create(userId, note);
      navigate(`/order/confirm/${result.order.id}`);
    } catch (e: any) {
      alert(e.message);
      setSubmitting(false);
    }
  };

  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  if (loading || !preview) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button onClick={() => navigate("/cart")} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <span className="text-sm text-gray-400">結帳</span>
      </div>

      <div className="px-4 py-3 border-b border-gray-800">
        <h3 className="text-sm font-bold text-white mb-1">賣家</h3>
        <p className="text-sm text-gray-400">{preview.seller_name}</p>
      </div>

      <div className="px-4 py-3 border-b border-gray-800">
        <h3 className="text-sm font-bold text-white mb-2">商品明細 ({preview.item_count} 件)</h3>
        {preview.items.map((item) => (
          <div key={item.product_id} className="flex items-center justify-between py-2">
            <div className="flex-1 min-w-0">
              <div className="text-sm text-white truncate">{item.product_title}</div>
              <div className="text-xs text-gray-500">{formatPrice(item.product_price)} x {item.quantity}</div>
            </div>
            <span className="text-sm font-bold text-white ml-2">{formatPrice(item.subtotal)}</span>
          </div>
        ))}
      </div>

      <div className="px-4 py-3 border-b border-gray-800">
        <h3 className="text-sm font-bold text-white mb-2">備註</h3>
        <textarea
          value={note}
          onChange={(e) => setNote(e.target.value)}
          placeholder="訂單備註（選填）"
          className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm border border-gray-700 resize-none h-20"
        />
      </div>

      <div className="px-4 py-4">
        <div className="flex items-center justify-between mb-4">
          <span className="text-white font-bold">合計</span>
          <span className="text-xl font-bold text-blue-400">{formatPrice(preview.total)}</span>
        </div>
        <button
          onClick={handleSubmit}
          disabled={submitting}
          className="w-full bg-blue-500 text-white rounded-full py-3 text-sm font-bold hover:bg-blue-600 disabled:opacity-50 transition"
        >
          {submitting ? "處理中..." : "確認下單"}
        </button>
      </div>
    </div>
  );
}
