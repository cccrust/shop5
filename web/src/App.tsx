import { Routes, Route, Navigate, Outlet } from "react-router-dom";
import { useAuth } from "./context/AuthContext";
import Layout from "./components/Layout";
import ProductList from "./pages/ProductList";
import ProductDetail from "./pages/ProductDetail";
import ProductEdit from "./pages/ProductEdit";
import Cart from "./pages/Cart";
import Checkout from "./pages/Checkout";
import OrderConfirm from "./pages/OrderConfirm";
import OrderList from "./pages/OrderList";
import OrderDetail from "./pages/OrderDetail";
import UserList from "./pages/UserList";
import UserDetail from "./pages/UserDetail";
import SearchPage from "./pages/Search";
import SellerOrders from "./pages/SellerOrders";
import SellerProducts from "./pages/SellerProducts";
import SellerDashboard from "./pages/SellerDashboard";
import Login from "./pages/Login";
import Register from "./pages/Register";

function ProtectedRoute() {
  const { user, loading } = useAuth();
  if (loading) return <div className="text-center py-20 text-gray-500">載入中...</div>;
  if (!user) return <Navigate to="/login" replace />;
  return <Outlet />;
}

function SellerRoute() {
  const { user, loading } = useAuth();
  if (loading) return <div className="text-center py-20 text-gray-500">載入中...</div>;
  if (!user) return <Navigate to="/login" replace />;
  if (user.role !== "seller") return <div className="text-center py-20 text-gray-500">只有賣家可以存取此頁面</div>;
  return <Outlet />;
}

export default function App() {
  const { loading } = useAuth();

  if (loading) {
    return <div className="text-center py-20 text-gray-500">載入中...</div>;
  }

  return (
    <Routes>
      <Route path="/login" element={<Login />} />
      <Route path="/register" element={<Register />} />
      <Route element={<Layout />}>
        <Route path="/" element={<ProductList />} />
        <Route path="/products/:id" element={<ProductDetail />} />
        <Route path="/search" element={<SearchPage />} />
        <Route element={<ProtectedRoute />}>
          <Route path="/cart" element={<Cart />} />
          <Route path="/checkout" element={<Checkout />} />
          <Route path="/order/confirm/:id" element={<OrderConfirm />} />
          <Route path="/orders" element={<OrderList />} />
          <Route path="/orders/:id" element={<OrderDetail />} />
          <Route path="/users" element={<UserList />} />
          <Route path="/users/:id" element={<UserDetail />} />
        </Route>
        <Route element={<SellerRoute />}>
          <Route path="/products/new" element={<ProductEdit />} />
          <Route path="/seller/orders" element={<SellerOrders />} />
          <Route path="/seller/products" element={<SellerProducts />} />
          <Route path="/seller/products/new" element={<ProductEdit />} />
          <Route path="/seller/dashboard" element={<SellerDashboard />} />
        </Route>
      </Route>
    </Routes>
  );
}
