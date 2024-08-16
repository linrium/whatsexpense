export interface Message {
  id: string;
  content: string;
  fromId: string;
  toId: string;
  replyTo: string | null;
  invoice: Invoice;
  transactions: Transaction[];
  createdAt: string;
  updatedAt: string;
}

export interface Invoice {
  id: string;
  userId: string;
  messageId: string;
  taxes: any[];
  discounts: any[];
  subtotal: number;
  total: number;
  currency: string;
  cardNumber: string | null;
  mediaPath: string | null;
  mediaType: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface Transaction {
  id: string;
  messageId: string;
  userId: string;
  title: string;
  amount: number;
  currency: string;
  categoryId: string;
  type: string;
  unit: string | null;
  quantity: number;
  issuedAt: string;
  createdAt: string;
  updatedAt: string;
}
