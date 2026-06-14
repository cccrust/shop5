import { NavLink, Outlet, useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";

export default function Layout() {
  const { user, logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = async () => {
    await logout();
    navigate("/login");
  };

  return (
    <div className="max-w-lg mx-auto min-h-screen flex flex-col">
      <div className="flex items-center justify-between px-4 py-2 border-b border-gray-800">
        {user ? (
          <>
            <div className="flex items-center gap-2">
              <div className="w-7 h-7 rounded-full bg-gray-700 flex items-center justify-center text-xs font-bold text-white shrink-0">
                {user.display_name[0]}
              </div>
              <span className="text-xs text-gray-400">{user.display_name}</span>
              <span className="text-[10px] text-gray-600">{user.role === "seller" ? "賣家" : "買家"}</span>
            </div>
            <button
              onClick={handleLogout}
              className="text-xs text-gray-500 hover:text-white transition"
            >
              登出
            </button>
          </>
        ) : (
          <div className="flex items-center gap-3 w-full justify-end">
            <button
              onClick={() => navigate("/login")}
              className="text-xs text-gray-400 hover:text-white transition"
            >
              登入
            </button>
            <button
              onClick={() => navigate("/register")}
              className="text-xs px-3 py-1 rounded bg-orange-500 text-white font-bold"
            >
              註冊
            </button>
          </div>
        )}
      </div>
      <main className="flex-1 pb-16"><Outlet /></main>
      <nav className="fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 z-50">
          <div className="max-w-lg mx-auto flex justify-around">
            <NavLink to="/" end className={({ isActive }) => navClass(isActive)}>
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
              </svg>
              <span className="text-[10px]">首頁</span>
            </NavLink>
            <NavLink to="/search" className={({ isActive }) => navClass(isActive)}>
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
              <span className="text-[10px]">搜尋</span>
            </NavLink>
            <NavLink to="/cart" className={({ isActive }) => navClass(isActive)}>
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 100 4 2 2 0 000-4z" />
              </svg>
              <span className="text-[10px]">購物車</span>
            </NavLink>
            <NavLink to="/orders" className={({ isActive }) => navClass(isActive)}>
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
              <span className="text-[10px]">訂單</span>
            </NavLink>
            <NavLink to="/users" className={({ isActive }) => navClass(isActive)}>
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z" />
              </svg>
              <span className="text-[10px]">使用者</span>
            </NavLink>
          </div>
        </nav>
    </div>
  );
}

function navClass(isActive: boolean) {
  return `flex flex-col items-center py-2 px-4 transition ${
    isActive ? "text-white" : "text-gray-600"
  }`;
}
