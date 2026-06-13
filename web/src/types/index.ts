export interface User {
  id: number;
  username: string;
  display_name: string;
  role: string;
  bio: string;
  avatar: string;
  created_at: string;
  updated_at: string;
}

export interface Product {
  id: number;
  seller_id: number;
  title: string;
  description: string;
  price: number;
  stock: number;
  status: string;
  sales_count: number;
  created_at: string;
  updated_at: string;
}

export interface CartItemWithProduct {
  id: number;
  user_id: number;
  product_id: number;
  quantity: number;
  title: string;
  price: number;
  stock: number;
}

export interface Order {
  id: number;
  buyer_id: number;
  seller_id: number;
  status: string;
  total: number;
  note: string;
  created_at: string;
  updated_at: string;
}

export interface OrderItem {
  id: number;
  order_id: number;
  product_id: number;
  product_title: string;
  product_price: number;
  quantity: number;
  subtotal: number;
}

export interface OrderWithItems {
  order: Order;
  items: OrderItem[];
}
