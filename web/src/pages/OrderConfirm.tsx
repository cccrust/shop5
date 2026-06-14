import { useParams, useNavigate } from "react-router-dom";

export default function OrderConfirm() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col items-center justify-center py-20 px-4">
      <div className="w-20 h-20 rounded-full bg-green-900 flex items-center justify-center mb-6">
        <svg className="w-10 h-10 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <h2 className="text-xl font-bold text-white mb-2">訂單已送出！</h2>
      <p className="text-sm text-gray-400 mb-8">訂單 #{id} 已成功建立</p>
      <div className="flex gap-3 w-full max-w-xs">
        <button
          onClick={() => navigate(`/orders/${id}`)}
          className="flex-1 bg-blue-500 text-white rounded-full py-3 text-sm font-bold hover:bg-blue-600 transition"
        >
          檢視訂單
        </button>
        <button
          onClick={() => navigate("/")}
          className="flex-1 bg-gray-700 text-white rounded-full py-3 text-sm font-bold hover:bg-gray-600 transition"
        >
          繼續購物
        </button>
      </div>
    </div>
  );
}
