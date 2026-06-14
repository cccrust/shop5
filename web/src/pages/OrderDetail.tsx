import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../context/AuthContext";
import type { OrderWithItems } from "../types";

const STATUS_LABEL: Record<string, string> = {
  pending: "待處理",
  paid: "已付款",
  shipped: "已出貨",
  delivered: "已送達",
  cancelled: "已取消",
};

export default function OrderDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { user } = useAuth();
  const [data, setData] = useState<OrderWithItems | null>(null);
  const [loading, setLoading] = useState(true);
  const [rating, setRating] = useState(5);
  const [reviewContent, setReviewContent] = useState("");
  const [reviewing, setReviewing] = useState(false);
  const [reviewProductId, setReviewProductId] = useState<number | null>(null);

  useEffect(() => {
    if (!id) return;
    const oid = parseInt(id);
    api.orders.get(oid).then((d) => {
      if (!d) return;
      setData(d);
    }).catch(() => {}).finally(() => setLoading(false));
  }, [id]);

  const handleReview = async (productId: number) => {
    if (!data || !user) return;
    try {
      await api.reviews.create({
        order_id: data.order.id,
        user_id: user.id,
        product_id: productId,
        rating,
        content: reviewContent,
      });
      alert("評價已送出");
      setReviewing(false);
      setReviewProductId(null);
      setReviewContent("");
    } catch (e: any) {
      alert(e.message);
    }
  };

  const formatPrice = (p: number) => `NT$${p.toLocaleString()}`;

  if (loading || !data) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  const { order, items } = data;

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button onClick={() => navigate("/orders")} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <span className="text-sm text-gray-400">訂單 #{order.id}</span>
      </div>

      <div className="px-4 py-4 border-b border-gray-800">
        <div className="flex items-center justify-between">
          <span className="text-white font-bold">訂單狀態</span>
          <span className={`text-sm px-3 py-1 rounded-full font-medium ${
            order.status === "delivered" ? "bg-green-900 text-green-400" :
            order.status === "cancelled" ? "bg-red-900 text-red-400" :
            "bg-yellow-900 text-yellow-400"
          }`}>
            {STATUS_LABEL[order.status] || order.status}
          </span>
        </div>
        {order.note && (
          <p className="text-sm text-gray-500 mt-2">備註：{order.note}</p>
        )}
        <p className="text-xs text-gray-600 mt-1">建立於 {order.created_at}</p>
      </div>

      <div className="px-4 py-3 border-b border-gray-800">
        <h3 className="text-sm font-bold text-white mb-2">明細</h3>
        {items.map((item) => (
          <div key={item.id} className="flex items-center justify-between py-2">
            <div className="flex-1 min-w-0">
              <div className="text-sm text-white truncate">{item.product_title}</div>
              <div className="text-xs text-gray-500">{formatPrice(item.product_price)} x {item.quantity}</div>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-sm font-bold text-white ml-2">{formatPrice(item.subtotal)}</span>
              {order.status === "delivered" && (
                <button
                  onClick={() => { setReviewProductId(item.product_id); setReviewing(true); }}
                  className="text-xs px-2 py-1 rounded bg-gray-700 text-yellow-400"
                >
                  評價
                </button>
              )}
            </div>
          </div>
        ))}
      </div>

      {reviewing && reviewProductId !== null && (
        <div className="px-4 py-4 border-b border-gray-800">
          <h3 className="text-sm font-bold text-white mb-3">撰寫評價</h3>
          <div className="flex items-center gap-2 mb-3">
            <span className="text-sm text-gray-400">評分：</span>
            {[1,2,3,4,5].map((n) => (
              <button
                key={n}
                onClick={() => setRating(n)}
                className={`text-xl ${n <= rating ? "text-yellow-400" : "text-gray-600"}`}
              >
                {n <= rating ? "★" : "☆"}
              </button>
            ))}
          </div>
          <textarea
            value={reviewContent}
            onChange={(e) => setReviewContent(e.target.value)}
            placeholder="撰寫評價內容（選填）"
            className="w-full bg-gray-800 text-white rounded-lg px-3 py-2 text-sm border border-gray-700 resize-none h-20"
          />
          <div className="flex gap-2 mt-3">
            <button
              onClick={() => { setReviewing(false); setReviewProductId(null); }}
              className="px-4 py-2 rounded bg-gray-700 text-sm text-white"
            >
              取消
            </button>
            <button
              onClick={() => handleReview(reviewProductId!)}
              className="px-4 py-2 rounded bg-blue-500 text-sm text-white font-bold"
            >
              送出評價
            </button>
          </div>
        </div>
      )}

      <div className="px-4 py-4 flex items-center justify-between">
        <span className="text-white font-bold">總計</span>
        <span className="text-xl font-bold text-blue-400">{formatPrice(order.total)}</span>
      </div>
    </div>
  );
}
