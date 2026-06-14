import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { useAuth } from "../context/AuthContext";

export default function Login() {
  const navigate = useNavigate();
  const { login } = useAuth();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    if (!username.trim() || !password.trim()) {
      setError("請輸入帳號和密碼");
      return;
    }
    setLoading(true);
    try {
      await login(username.trim(), password);
      navigate("/");
    } catch (err: any) {
      setError(err.message || "登入失敗");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen px-6">
      <h1 className="text-2xl font-bold text-white mb-2">Shop5</h1>
      <p className="text-sm text-gray-500 mb-8">登入你的帳號</p>
      <form onSubmit={handleSubmit} className="w-full max-w-sm space-y-4">
        <div>
          <label className="text-xs text-gray-500 block mb-1">帳號</label>
          <input
            type="text"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
            autoFocus
          />
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">密碼</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          />
        </div>
        {error && <p className="text-red-400 text-sm">{error}</p>}
        <button
          type="submit"
          disabled={loading}
          className="w-full py-2.5 rounded-lg bg-blue-500 text-white font-bold disabled:opacity-50"
        >
          {loading ? "登入中..." : "登入"}
        </button>
        <p className="text-center text-sm text-gray-500">
          還沒有帳號？<Link to="/register" className="text-blue-400">註冊</Link>
        </p>
      </form>
    </div>
  );
}
