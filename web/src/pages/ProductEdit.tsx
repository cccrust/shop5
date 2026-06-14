import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import { useAuth } from "../context/AuthContext";
import type { Category } from "../types";

export default function ProductEdit() {
  const navigate = useNavigate();
  const { user } = useAuth();
  const [title, setTitle] = useState("");
  const [price, setPrice] = useState("");
  const [stock, setStock] = useState("");
  const [description, setDescription] = useState("");
  const [categoryId, setCategoryId] = useState<number | undefined>();
  const [categories, setCategories] = useState<Category[]>([]);
  const [error, setError] = useState("");
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (!user) return;
    (async () => {
      try {
        const cats = await api.categories.list();
        setCategories(cats);
      } catch {
        // ignore
      }
    })();
  }, [user]);

  if (!user) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) { setError("請輸入商品名稱"); return; }
    if (!price || Number(price) < 0) { setError("請輸入有效的價格"); return; }
    if (!stock || Number(stock) < 0) { setError("請輸入有效的庫存"); return; }
    setSaving(true);
    try {
      const p = await api.products.create({
        seller_id: user.id,
        title: title.trim(),
        price: Number(price),
        stock: Number(stock),
        description,
        category_id: categoryId,
      });
      navigate(`/products/${p.id}`);
    } catch (err) {
      setError(String(err));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800 flex items-center gap-3">
        <button onClick={() => navigate("/seller/products")} className="text-white">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <span className="text-sm text-gray-400">新增商品</span>
      </div>

      <form onSubmit={handleSubmit} className="p-4 space-y-4">
        <div>
          <label className="text-xs text-gray-500 block mb-1">商品名稱</label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          />
        </div>
        <div className="flex gap-3">
          <div className="flex-1">
            <label className="text-xs text-gray-500 block mb-1">價格 (NT$)</label>
            <input
              type="number"
              value={price}
              onChange={(e) => setPrice(e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
            />
          </div>
          <div className="flex-1">
            <label className="text-xs text-gray-500 block mb-1">庫存</label>
            <input
              type="number"
              value={stock}
              onChange={(e) => setStock(e.target.value)}
              className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
            />
          </div>
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">分類</label>
          <select
            value={categoryId ?? ""}
            onChange={(e) => setCategoryId(e.target.value ? Number(e.target.value) : undefined)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          >
            <option value="">不分類</option>
            {categories.map((c) => (
              <option key={c.id} value={c.id}>{c.name}</option>
            ))}
          </select>
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">描述</label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={3}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white resize-none"
          />
        </div>
        {error && <p className="text-red-400 text-sm">{error}</p>}
        <button
          type="submit"
          disabled={saving}
          className="w-full py-2.5 rounded-lg bg-orange-500 text-white font-bold disabled:opacity-50"
        >
          {saving ? "儲存中..." : "建立商品"}
        </button>
      </form>
    </div>
  );
}
