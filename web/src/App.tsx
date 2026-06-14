import { Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import ProductList from "./pages/ProductList";
import ProductDetail from "./pages/ProductDetail";
import Cart from "./pages/Cart";
import OrderList from "./pages/OrderList";
import OrderDetail from "./pages/OrderDetail";
import UserList from "./pages/UserList";
import UserDetail from "./pages/UserDetail";
import SearchPage from "./pages/Search";
import SellerOrders from "./pages/SellerOrders";
import SellerProducts from "./pages/SellerProducts";
import ProductEdit from "./pages/ProductEdit";

export default function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<ProductList />} />
        <Route path="/products/:id" element={<ProductDetail />} />
        <Route path="/products/new" element={<ProductEdit />} />
        <Route path="/cart" element={<Cart />} />
        <Route path="/orders" element={<OrderList />} />
        <Route path="/orders/:id" element={<OrderDetail />} />
        <Route path="/users" element={<UserList />} />
        <Route path="/users/:id" element={<UserDetail />} />
        <Route path="/search" element={<SearchPage />} />
        <Route path="/seller/orders" element={<SellerOrders />} />
        <Route path="/seller/products" element={<SellerProducts />} />
        <Route path="/seller/products/new" element={<ProductEdit />} />
      </Routes>
    </Layout>
  );
}
