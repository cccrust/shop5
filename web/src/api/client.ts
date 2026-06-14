import type { User, Product, CartItemWithProduct, Order, OrderWithItems, Category, Review, CartPreview, SellerStats, LoginResponse } from "../types";

let _token: string | null = localStorage.getItem("token");

export function setToken(token: string | null) {
  _token = token;
  if (token) localStorage.setItem("token", token);
  else localStorage.removeItem("token");
}

export function getToken(): string | null {
  return _token;
}

const BASE = "/api";

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const headers: Record<string, string> = { "Content-Type": "application/json" };
  if (_token) headers["Authorization"] = `Bearer ${_token}`;
  const res = await fetch(`${BASE}${path}`, {
    headers,
    ...options,
  });
  if (res.status === 401) {
    setToken(null);
    window.location.href = "/login";
    throw new Error("請先登入");
  }
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || `HTTP ${res.status}`);
  }
  return res.json();
}

export const api = {
  auth: {
    login: (username: string, password: string) =>
      request<LoginResponse>("/auth/login", { method: "POST", body: JSON.stringify({ username, password }) }),
    register: (data: { username: string; display_name: string; email?: string; password: string; role?: string }) =>
      request<LoginResponse>("/auth/register", { method: "POST", body: JSON.stringify(data) }),
    logout: () => request<{ ok: boolean }>("/auth/logout", { method: "POST" }),
    me: () => request<User>("/auth/me"),
  },
  users: {
    list: (search?: string) =>
      request<User[]>(`/users${search ? `?search=${encodeURIComponent(search)}` : ""}`),
    get: (id: number) => request<User>(`/users/${id}`),
    create: (data: { username: string; display_name: string; role?: string; bio?: string; email?: string; password?: string }) =>
      request<User>("/users", { method: "POST", body: JSON.stringify(data) }),
    update: (id: number, data: { display_name?: string; bio?: string; role?: string }) =>
      request<User>(`/users/${id}`, { method: "PUT", body: JSON.stringify(data) }),
  },
  products: {
    list: (params?: { seller_id?: number; status?: string; category_id?: number }) => {
      const q = new URLSearchParams();
      if (params?.seller_id) q.set("seller_id", String(params.seller_id));
      if (params?.status) q.set("status", params.status);
      if (params?.category_id) q.set("category_id", String(params.category_id));
      return request<Product[]>(`/products?${q}`);
    },
    get: (id: number) => request<Product>(`/products/${id}`),
    create: (data: { seller_id: number; title: string; price: number; stock: number; description?: string; category_id?: number }) =>
      request<Product>("/products", { method: "POST", body: JSON.stringify(data) }),
    update: (id: number, data: { title?: string; price?: number; stock?: number; status?: string; description?: string; category_id?: number }) =>
      request<Product>(`/products/${id}`, { method: "PUT", body: JSON.stringify(data) }),
    search: (params: { q?: string; category_id?: number; min_price?: number; max_price?: number; seller_id?: number }) => {
      const q = new URLSearchParams();
      if (params.q) q.set("q", params.q);
      if (params.category_id) q.set("category_id", String(params.category_id));
      if (params.min_price) q.set("min_price", String(params.min_price));
      if (params.max_price) q.set("max_price", String(params.max_price));
      if (params.seller_id) q.set("seller_id", String(params.seller_id));
      return request<Product[]>(`/products/search?${q}`);
    },
  },
  seller: {
    myOrders: () => request<Order[]>("/seller/me/orders"),
    myProducts: () => request<Product[]>("/seller/me/products"),
    myStats: () => request<SellerStats>("/seller/me/stats"),
    orders: (id: number) => request<Order[]>(`/seller/${id}/orders`),
    products: (id: number) => request<Product[]>(`/seller/${id}/products`),
    stats: (id: number) => request<SellerStats>(`/seller/${id}/stats`),
  },
  categories: {
    list: () => request<Category[]>("/categories"),
    create: (data: { name: string; parent_id?: number }) =>
      request<Category>("/categories", { method: "POST", body: JSON.stringify(data) }),
    delete: (id: number) => request<{ deleted: boolean }>(`/categories/${id}`, { method: "DELETE" }),
  },
  cart: {
    myList: () => request<CartItemWithProduct[]>("/cart/me"),
    myClear: () => request<{ ok: boolean }>("/cart/me", { method: "DELETE" }),
    myUpdateQty: (productId: number, quantity: number) =>
      request<{ ok: boolean }>(`/cart/me/${productId}`, { method: "PUT", body: JSON.stringify({ quantity }) }),
    list: (userId: number) => request<CartItemWithProduct[]>(`/cart/${userId}`),
    add: (userId: number, productId: number, quantity?: number) =>
      request<{ ok: boolean }>("/cart", {
        method: "POST",
        body: JSON.stringify({ user_id: userId, product_id: productId, quantity: quantity ?? 1 }),
      }),
    remove: (userId: number, productId: number) =>
      request<{ ok: boolean }>("/cart", {
        method: "DELETE",
        body: JSON.stringify({ user_id: userId, product_id: productId }),
      }),
    clear: (userId: number) =>
      request<{ ok: boolean }>(`/cart/${userId}`, { method: "DELETE" }),
    update: (userId: number, productId: number, quantity: number) =>
      request<{ ok: boolean }>(`/cart/${userId}/${productId}`, {
        method: "PUT",
        body: JSON.stringify({ quantity }),
      }),
  },
  orders: {
    myList: () => request<Order[]>("/orders/me"),
    myPreview: () => request<CartPreview>("/orders/me/preview", { method: "POST" }),
    myCreate: (note?: string) =>
      request<OrderWithItems>("/orders/me", { method: "POST", body: JSON.stringify({ note: note ?? "" }) }),
    preview: (buyerId: number) =>
      request<CartPreview>("/orders/preview", {
        method: "POST",
        body: JSON.stringify({ buyer_id: buyerId }),
      }),
    list: (params?: { buyer_id?: number; seller_id?: number }) => {
      const q = new URLSearchParams();
      if (params?.buyer_id) q.set("buyer_id", String(params.buyer_id));
      if (params?.seller_id) q.set("seller_id", String(params.seller_id));
      return request<Order[]>(`/orders?${q}`);
    },
    get: (id: number) => request<OrderWithItems>(`/orders/${id}`),
    create: (buyerId: number, note?: string) =>
      request<OrderWithItems>("/orders", {
        method: "POST",
        body: JSON.stringify({ buyer_id: buyerId, note: note ?? "" }),
      }),
    update: (id: number, status: string) =>
      request<Order>(`/orders/${id}`, {
        method: "PUT",
        body: JSON.stringify({ status }),
      }),
  },
  reviews: {
    listByProduct: (productId: number) => request<Review[]>(`/reviews/product/${productId}`),
    listByUser: (userId: number) => request<Review[]>(`/reviews/user/${userId}`),
    create: (data: { order_id: number; user_id: number; product_id: number; rating: number; content?: string }) =>
      request<Review>("/reviews", { method: "POST", body: JSON.stringify(data) }),
    get: (id: number) => request<Review>(`/reviews/${id}`),
    delete: (id: number) => request<void>(`/reviews/${id}`, { method: "DELETE" }),
  },
};
