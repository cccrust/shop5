import type { User, Product, CartItemWithProduct, Order, OrderWithItems, Category, Review, CartPreview, SellerStats } from "../types";

const BASE = "/api";

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { "Content-Type": "application/json", ...options?.headers },
    ...options,
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || `HTTP ${res.status}`);
  }
  return res.json();
}

export const api = {
  users: {
    list: (search?: string) =>
      request<User[]>(`/users${search ? `?search=${encodeURIComponent(search)}` : ""}`),
    get: (id: number) => request<User>(`/users/${id}`),
    create: (data: { username: string; display_name: string; role?: string; bio?: string }) =>
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
