import { Product, Shop, CustomerServiceMessage } from "../types";

const INITIAL_SHOPS: Shop[] = [
  {
    id: "shop_1",
    name: "官方严选旗舰店",
    logo: "https://picsum.photos/seed/shop1/100/100",
    fansCount: "25.6w",
    rating: "4.9",
    isOfficial: true,
    tags: ["品牌直采", "正品保证"],
    description: "为您提供高品质、高性价比的生活好物。",
  },
  {
    id: "shop_2",
    name: "极客数码生活",
    logo: "https://picsum.photos/seed/shop2/100/100",
    fansCount: "8.2w",
    rating: "4.7",
    isOfficial: false,
    tags: ["7天无理由退货"],
    description: "最新鲜的数码体验，最酷的极客装备。",
  },
];

const INITIAL_PRODUCTS: Product[] = [
  {
    id: "p1",
    title: "2026新款 智能降噪蓝牙耳机 续航50小时",
    price: "299",
    image: "https://picsum.photos/seed/product1/300/300",
    sales: "已售 1.2w",
    description: "高品质降噪耳机，续航无忧。",
    shopId: "shop_2",
  },
  {
    id: "p2",
    title: "便携式迷你筋膜枪 肌肉放松 按摩仪",
    price: "159",
    image: "https://picsum.photos/seed/product2/300/300",
    sales: "已售 8000+",
    description: "迷你便携，深度按摩。",
    shopId: "shop_1",
  },
  {
    id: "p3",
    title: "全棉亲肤四季法兰绒毛毯 宿舍单人",
    price: "89",
    originalPrice: "129",
    image: "https://picsum.photos/seed/product3/300/300",
    sales: "已售 2000+",
    description: "柔软亲肤，保暖舒适。",
    shopId: "shop_1",
  },
  {
    id: "p4",
    title: "智能温控养生壶 不锈钢玻璃材质 1.5L",
    price: "129",
    image: "https://picsum.photos/seed/product4/300/300",
    sales: "已售 5.5w",
    description: "智能温控，健康养生。",
    shopId: "shop_1",
  },
  {
    id: "p_virtual_1",
    title: "星巴克 咖啡星冰乐 电子代金券/兑换券",
    price: "28", // min price
    image: "https://picsum.photos/seed/coffee/300/300",
    sales: "已售 10w+",
    description: "购买后立即发码，支持全国门店核销。",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "denomination",
        name: "面值",
        options: [
          { id: "d_28", name: "28元代金券" },
          { id: "d_32", name: "32元代金券" },
          { id: "d_50", name: "50元代金券" },
        ]
      }
    ],
    skus: [
      { id: "sku_1", specValues: { "denomination": "d_28" }, price: "28", stock: 999 },
      { id: "sku_2", specValues: { "denomination": "d_32" }, price: "32", stock: 999 },
      { id: "sku_3", specValues: { "denomination": "d_50" }, price: "48", originalPrice: "50", stock: 999 }
    ]
  },
  {
    id: "p_virtual_2",
    title: "高端AI实战俱乐部 / 技术大牛社群",
    price: "199",
    image: "https://picsum.photos/seed/community/300/300",
    sales: "已售 500+",
    description: "购买后将自动拉您进入专属高端交流群，群内有大佬在线解答各种技术难题，定期分享干货。",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "group_chat",
    specs: [
      {
        id: "group_type",
        name: "圈子类型",
        options: [
          { id: "g_ai_1", name: "大模型应用实战圈 (-群)" },
          { id: "g_ai_2", name: "大模型应用实战圈 (二群)" },
          { id: "g_web_1", name: "全栈开发交流圈" }
        ]
      }
    ],
    skus: [
      { id: "sku_gc_1", specValues: { "group_type": "g_ai_1" }, price: "199", stock: 100 },
      { id: "sku_gc_2", specValues: { "group_type": "g_ai_2" }, price: "199", stock: 50 },
      { id: "sku_gc_3", specValues: { "group_type": "g_web_1" }, price: "99", stock: 200 }
    ]
  }
];

const CATEGORIES = [
  "推荐",
  "数码家电",
  "生活日用",
  "服饰箱包",
  "食品饮料",
  "美妆个护",
];

const STORAGE_KEY_PRODUCTS = "clawchat_products";
const STORAGE_KEY_SHOPS = "clawchat_shops";

let MOCK_PRODUCTS: Product[] = [];
let MOCK_SHOPS: Shop[] = [];
let MOCK_CUSTOMER_SERVICE_MESSAGES: Record<string, CustomerServiceMessage[]> = {};

const loadData = () => {
  if (MOCK_PRODUCTS.length > 0 && MOCK_SHOPS.length > 0) {
    if (!MOCK_PRODUCTS.find(p => p.id === "p_virtual_1" && p.virtualType === "coupon") || !MOCK_PRODUCTS.find(p => p.id === "p_virtual_2" && p.virtualType === "group_chat")) {
      MOCK_PRODUCTS = [...INITIAL_PRODUCTS];
      MOCK_SHOPS = [...INITIAL_SHOPS];
      saveData();
    }
    return { products: MOCK_PRODUCTS, shops: MOCK_SHOPS };
  }
  try {
    const productsData = localStorage.getItem(STORAGE_KEY_PRODUCTS);
    const shopsData = localStorage.getItem(STORAGE_KEY_SHOPS);
    if (productsData && shopsData) {
      MOCK_PRODUCTS = JSON.parse(productsData);
      MOCK_SHOPS = JSON.parse(shopsData);
      if (!MOCK_PRODUCTS.find(p => p.id === "p_virtual_1" && p.virtualType === "coupon") || !MOCK_PRODUCTS.find(p => p.id === "p_virtual_2" && p.virtualType === "group_chat")) {
        MOCK_PRODUCTS = [...INITIAL_PRODUCTS];
        MOCK_SHOPS = [...INITIAL_SHOPS];
        saveData();
      }
    } else {
      MOCK_PRODUCTS = [...INITIAL_PRODUCTS];
      MOCK_SHOPS = [...INITIAL_SHOPS];
      saveData();
    }
  } catch (e) {
    MOCK_PRODUCTS = [...INITIAL_PRODUCTS];
    MOCK_SHOPS = [...INITIAL_SHOPS];
  }
  return { products: MOCK_PRODUCTS, shops: MOCK_SHOPS };
};

const saveData = () => {
  try {
    localStorage.setItem(STORAGE_KEY_PRODUCTS, JSON.stringify(MOCK_PRODUCTS));
    localStorage.setItem(STORAGE_KEY_SHOPS, JSON.stringify(MOCK_SHOPS));
  } catch (e) {
    console.error("Failed to save products data", e);
  }
};

export const ProductService = {
  getProducts: async (): Promise<Product[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadData().products]), 300),
    );
  },
  getProductById: async (id: string): Promise<Product | null> => {
    return new Promise((resolve) =>
      setTimeout(() => {
        resolve(loadData().products.find((p) => p.id === id) || null);
      }, 200),
    );
  },
  getProductsByShop: async (shopId: string): Promise<Product[]> => {
    return new Promise((resolve) =>
      setTimeout(() => {
        resolve(loadData().products.filter((p) => p.shopId === shopId));
      }, 200),
    );
  },
  getShopById: async (id: string): Promise<Shop | null> => {
    return new Promise((resolve) =>
      setTimeout(() => {
        resolve(loadData().shops.find((s) => s.id === id) || null);
      }, 200),
    );
  },
  getCategories: async (): Promise<string[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...CATEGORIES]), 100),
    );
  },
  getCustomerServiceMessages: async (shopId: string): Promise<CustomerServiceMessage[]> => {
    if (!MOCK_CUSTOMER_SERVICE_MESSAGES[shopId]) {
      MOCK_CUSTOMER_SERVICE_MESSAGES[shopId] = [
        {
          id: "msg_1",
          content: "您好！欢迎来到我们的店铺，请问有什么可以帮助您的吗？",
          senderId: "agent",
          senderType: "agent",
          timestamp: Date.now() - 60000,
        }
      ];
    }
    return MOCK_CUSTOMER_SERVICE_MESSAGES[shopId];
  },
  sendCustomMessage: async (shopId: string, msg: CustomerServiceMessage): Promise<void> => {
    if (!MOCK_CUSTOMER_SERVICE_MESSAGES[shopId]) {
      MOCK_CUSTOMER_SERVICE_MESSAGES[shopId] = [];
    }
    MOCK_CUSTOMER_SERVICE_MESSAGES[shopId].push(msg);
  }
};
