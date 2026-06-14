import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { useAuth } from "../context/AuthContext";

export default function Register() {
  const navigate = useNavigate();
  const { register } = useAuth();
  const [username, setUsername] = useState("");
  const [displayName, setDisplayName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [role, setRole] = useState("buyer");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    if (!username.trim() || !displayName.trim() || !password.trim()) {
      setError("請填寫必要欄位");
      return;
    }
    if (password !== confirmPassword) {
      setError("兩次密碼輸入不一致");
      return;
    }
    if (password.length < 4) {
      setError("密碼長度至少 4 碼");
      return;
    }
    setLoading(true);
    try {
      await register({
        username: username.trim(),
        display_name: displayName.trim(),
        email: email.trim(),
        password,
        role,
      });
      navigate("/");
    } catch (err: any) {
      setError(err.message || "註冊失敗");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen px-6">
      <h1 className="text-2xl font-bold text-white mb-2">註冊</h1>
      <p className="text-sm text-gray-500 mb-8">建立新帳號</p>
      <form onSubmit={handleSubmit} className="w-full max-w-sm space-y-4">
        <div>
          <label className="text-xs text-gray-500 block mb-1">帳號 *</label>
          <input
            type="text"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
            autoFocus
          />
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">顯示名稱 *</label>
          <input
            type="text"
            value={displayName}
            onChange={(e) => setDisplayName(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          />
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">Email</label>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          />
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">密碼 *</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          />
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">確認密碼 *</label>
          <input
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          />
        </div>
        <div>
          <label className="text-xs text-gray-500 block mb-1">角色</label>
          <select
            value={role}
            onChange={(e) => setRole(e.target.value)}
            className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-600 text-white"
          >
            <option value="buyer">買家</option>
            <option value="seller">賣家</option>
          </select>
        </div>
        {error && <p className="text-red-400 text-sm">{error}</p>}
        <button
          type="submit"
          disabled={loading}
          className="w-full py-2.5 rounded-lg bg-blue-500 text-white font-bold disabled:opacity-50"
        >
          {loading ? "註冊中..." : "註冊"}
        </button>
        <p className="text-center text-sm text-gray-500">
          已經有帳號？<Link to="/login" className="text-blue-400">登入</Link>
        </p>
      </form>
    </div>
  );
}
