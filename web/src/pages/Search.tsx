import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { api } from "../api/client";
import type { Product, Category } from "../types";

export default function SearchPage() {
  const [keyword, setKeyword] = useState("");
  const [categoryId, setCategoryId] = useState<number | undefined>();
  const [minPrice, setMinPrice] = useState("");
  const [maxPrice, setMaxPrice] = useState("");
  const [results, setResults] = useState<Product[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(false);
  const [searched, setSearched] = useState(false);

  useEffect(() => {
    api.categories.list().then(setCategories).catch(() => {});
  }, []);

  const doSearch = async () => {
    setLoading(true);
    setSearched(true);
    try {
      const r = await api.products.search({
        q: keyword || undefined,
        category_id: categoryId,
        min_price: minPrice ? Number(minPrice) : undefined,
        max_price: maxPrice ? Number(maxPrice) : undefined,
      });
      setResults(r);
    } catch {
      setResults([]);
    } finally {
      setLoading(false);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    doSearch();
  };

  return (
    <div className="p-4">
      <h1 className="text-xl font-bold mb-4">搜尋商品</h1>
      <form onSubmit={handleSubmit} className="space-y-3 mb-6">
        <input
          type="text"
          placeholder="關鍵字..."
          value={keyword}
          onChange={(e) => setKeyword(e.target.value)}
          className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white placeholder-gray-400"
        />
        <div className="flex gap-2">
          <select
            value={categoryId ?? ""}
            onChange={(e) => setCategoryId(e.target.value ? Number(e.target.value) : undefined)}
            className="flex-1 px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          >
            <option value="">全部分類</option>
            {categories.map((c) => (
              <option key={c.id} value={c.id}>{c.name}</option>
            ))}
          </select>
          <input
            type="number"
            placeholder="最低價"
            value={minPrice}
            onChange={(e) => setMinPrice(e.target.value)}
            className="w-24 px-2 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white placeholder-gray-400"
          />
          <input
            type="number"
            placeholder="最高價"
            value={maxPrice}
            onChange={(e) => setMaxPrice(e.target.value)}
            className="w-24 px-2 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white placeholder-gray-400"
          />
        </div>
        <button
          type="submit"
          className="w-full py-2 rounded-lg bg-orange-500 text-white font-bold hover:bg-orange-600"
        >
          搜尋
        </button>
      </form>

      {loading && <p className="text-gray-400">搜尋中...</p>}

      {searched && !loading && results.length === 0 && (
        <p className="text-gray-400">查無符合條件的商品</p>
      )}

      <div className="space-y-3">
        {results.map((p) => (
          <Link
            key={p.id}
            to={`/products/${p.id}`}
            className="block p-3 rounded-lg bg-gray-800 hover:bg-gray-700"
          >
            <div className="font-semibold">{p.title}</div>
            <div className="text-orange-400">NT${p.price}</div>
            {p.category_id && (
              <div className="text-xs text-gray-400 mt-1">
                分類: {categories.find(c => c.id === p.category_id)?.name ?? `#${p.category_id}`}
              </div>
            )}
          </Link>
        ))}
      </div>
    </div>
  );
}
