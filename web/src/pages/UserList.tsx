import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api/client";
import type { User } from "../types";

export default function UserList() {
  const navigate = useNavigate();
  const [users, setUsers] = useState<User[]>([]);
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.users.list().then(setUsers).catch(() => {}).finally(() => setLoading(false));
  }, []);

  const filtered = users.filter((u) =>
    !search || u.username.includes(search) || u.display_name.includes(search)
  );

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <div>
      <div className="px-4 py-3 border-b border-gray-800">
        <h2 className="font-bold text-white">使用者</h2>
      </div>
      <div className="px-4 py-2 border-b border-gray-800">
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="搜尋使用者..."
          className="w-full bg-gray-900 text-white rounded-full px-4 py-2 text-sm placeholder-gray-500 outline-none focus:ring-1 focus:ring-gray-600"
        />
      </div>
      {filtered.map((u) => (
        <div
          key={u.id}
          onClick={() => navigate(`/users/${u.id}`)}
          className="flex items-center gap-3 px-4 py-3 border-b border-gray-800 hover:bg-gray-900/50 transition cursor-pointer"
        >
          <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center text-sm font-bold shrink-0">
            {u.display_name[0]}
          </div>
          <div>
            <div className="font-bold text-white text-sm">{u.display_name}</div>
            <div className="text-xs text-gray-500">@{u.username} · {u.role}</div>
          </div>
        </div>
      ))}
      {filtered.length === 0 && (
        <div className="text-center py-20 text-gray-500">查無使用者</div>
      )}
    </div>
  );
}
