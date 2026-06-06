export interface ShopCategory {
  id: string;
  name: string;
  icon: string;
}

export interface ShopProductSku {
  id: string;
  name: string;
  price: number;
  stock: number;
  specs?: Record<string, string>; // e.g. { 'Color': 'Black', 'Storage': '256GB' }
  imageUrl?: string;
}

export interface ShopProductOption {
  name: string;
  values: string[];
}

export interface ShopProduct {
  id: string;
  categoryId: string;
  title: string;
  description: string;
  price: number; // minimum price
  originalPrice?: number;
  imageUrl: string;
  salesVolume: number;
  rating: number;
  tags: string[];
  options?: ShopProductOption[];
  skus?: ShopProductSku[];
  isVirtual?: boolean;
  isCoupon?: boolean;
}

export interface CartItem {
  id: string;
  productId: string;
  skuId?: string;
  quantity: number;
  selected: boolean;
}

export interface ShopOrder {
  id: string;
  items: CartItem[];
  total: number;
  status: "pending" | "paid" | "shipped" | "completed" | "cancelled";
  createdAt: number;
  coupons?: { item: CartItem; product: ShopProduct; code: string }[];
}

export interface ShopService {
  getCategories(): Promise<ShopCategory[]>;
  getProducts(categoryId?: string): Promise<ShopProduct[]>;
  getProductById(id: string): Promise<ShopProduct | null>;
  getCart(): Promise<CartItem[]>;
  addToCart(
    productId: string,
    quantity?: number,
    skuId?: string,
  ): Promise<void>;
  updateCartItem(cartItemId: string, quantity: number): Promise<void>;
  toggleCartItemSelection(cartItemId: string): Promise<void>;
  toggleAllCartItems(selected: boolean): Promise<void>;
  removeCartItem(cartItemId: string): Promise<void>;
  checkout(
    items?: CartItem[],
  ): Promise<{
    orderId: string;
    total: number;
    items: CartItem[];
    generatedCoupons?: { item: CartItem; product: ShopProduct; code: string }[];
  }>;
  toggleFavorite(productId: string): Promise<boolean>;
  isFavorite(productId: string): Promise<boolean>;
  getOrders(): Promise<ShopOrder[]>;
}

const MOCK_CATEGORIES: ShopCategory[] = [
  { id: "C_ELEC", name: "数码电器", icon: "💻" },
  { id: "C_CLOTHES", name: "服饰鞋包", icon: "👗" },
  { id: "C_FOOD", name: "食品生鲜", icon: "🍎" },
  { id: "C_BEAUTY", name: "美妆个护", icon: "💄" },
  { id: "C_HOME", name: "家居日用", icon: "🏠" },
  { id: "C_VIRTUAL", name: "虚拟服务", icon: "🎮" },
  { id: "C_COUPON", name: "积分与卡券", icon: "🎫" },
];

const MOCK_PRODUCTS: ShopProduct[] = [
  {
    id: "P1",
    categoryId: "C_ELEC",
    title: "新款智能手机 Pro Max 256GB",
    description:
      "搭载最新一代芯片，超视网膜XDR显示屏，全天候电池续航。顶级摄像系统。",
    price: 7999,
    originalPrice: 9999,
    imageUrl:
      "https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?q=80&w=600&auto=format&fit=crop",
    salesVolume: 12500,
    rating: 4.9,
    tags: ["5G", "新品", "免息"],
    options: [
      { name: "Color", values: ["暗夜黑", "银色"] },
      { name: "Storage", values: ["256GB", "512GB"] },
    ],
    skus: [
      {
        id: "S1_1",
        name: "暗夜黑 256GB",
        price: 7999,
        stock: 100,
        specs: { Color: "暗夜黑", Storage: "256GB" },
      },
      {
        id: "S1_2",
        name: "银色 256GB",
        price: 7999,
        stock: 0,
        specs: { Color: "银色", Storage: "256GB" },
      },
      {
        id: "S1_3",
        name: "暗夜黑 512GB",
        price: 8999,
        stock: 0,
        specs: { Color: "暗夜黑", Storage: "512GB" },
      },
      {
        id: "S1_4",
        name: "银色 512GB",
        price: 8999,
        stock: 30,
        specs: { Color: "银色", Storage: "512GB" },
      },
    ],
  },
  {
    id: "P2",
    categoryId: "C_ELEC",
    title: "降噪无线蓝牙耳机",
    description: "主动降噪功能，通透模式，空间音频支持。舒适佩戴，长久续航。",
    price: 1899,
    imageUrl:
      "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?q=80&w=600&auto=format&fit=crop",
    salesVolume: 8500,
    rating: 4.8,
    tags: ["爆款", "包邮"],
    options: [{ name: "Color", values: ["标准白"] }],
    skus: [
      {
        id: "S2_1",
        name: "标准白",
        price: 1899,
        stock: 200,
        specs: { Color: "标准白" },
      },
    ],
  },
  {
    id: "P9",
    categoryId: "C_ELEC",
    title: "曲面电竞显示器 165Hz",
    description: "2K分辨率，1ms响应时间，沉浸式游戏体验。1500R曲率。",
    price: 1599,
    imageUrl:
      "https://images.unsplash.com/photo-1527443224154-c4a3942d3acf?q=80&w=600&auto=format&fit=crop",
    salesVolume: 4200,
    rating: 4.7,
    tags: ["电竞", "外设"],
  },
  {
    id: "P13",
    categoryId: "C_ELEC",
    title: "轻薄商务笔记本电脑 14英寸",
    description: "12代酷睿处理器，16G内存，1TB固态硬盘，金属机身超长续航。",
    price: 4999,
    originalPrice: 5499,
    imageUrl:
      "https://images.unsplash.com/photo-1496181133206-80ce9b88a853?q=80&w=600&auto=format&fit=crop",
    salesVolume: 3200,
    rating: 4.9,
    tags: ["办公", "轻薄"],
  },
  {
    id: "P14",
    categoryId: "C_ELEC",
    title: "机械键盘 青轴 87键",
    description: "全键无冲，RGB背光，清脆手感，游戏办公利器。",
    price: 299,
    imageUrl:
      "https://images.unsplash.com/photo-1595225476474-87563907a212?q=80&w=600&auto=format&fit=crop",
    salesVolume: 18000,
    rating: 4.8,
    tags: ["外设", "满减"],
  },
  {
    id: "P4",
    categoryId: "C_CLOTHES",
    title: "秋冬新款高级感羊绒大衣",
    description: "100%纯羊绒，修身显瘦，多色可选。气质首选，保暖升级。",
    price: 1299,
    imageUrl:
      "https://images.unsplash.com/photo-1539533113208-f6df8cc8b543?q=80&w=600&auto=format&fit=crop",
    salesVolume: 1500,
    rating: 4.9,
    tags: ["女装", "热销"],
    options: [
      { name: "Color", values: ["卡其色", "黑色"] },
      { name: "Size", values: ["S", "M", "L"] },
    ],
    skus: [
      {
        id: "S4_1",
        name: "卡其色 S",
        price: 1299,
        stock: 20,
        specs: { Color: "卡其色", Size: "S" },
      },
      {
        id: "S4_2",
        name: "卡其色 M",
        price: 1299,
        stock: 0,
        specs: { Color: "卡其色", Size: "M" },
      },
      {
        id: "S4_3",
        name: "黑色 M",
        price: 1299,
        stock: 30,
        specs: { Color: "黑色", Size: "M" },
      },
      {
        id: "S4_4",
        name: "黑色 L",
        price: 1299,
        stock: 15,
        specs: { Color: "黑色", Size: "L" },
      },
    ],
  },
  {
    id: "P15",
    categoryId: "C_CLOTHES",
    title: "经典百搭小白鞋",
    description: "真皮材质，橡胶舒适软底，透气耐磨，四季适宜。",
    price: 199,
    originalPrice: 299,
    imageUrl:
      "https://images.unsplash.com/photo-1549298916-b41d501d3772?q=80&w=600&auto=format&fit=crop",
    salesVolume: 88000,
    rating: 4.8,
    tags: ["爆款鞋履", "休闲"],
    options: [{ name: "Size", values: ["36", "37", "38", "39", "40"] }],
    skus: [
      {
        id: "S15_1",
        name: "37码",
        price: 199,
        stock: 100,
        specs: { Size: "37" },
      },
      {
        id: "S15_2",
        name: "38码",
        price: 199,
        stock: 120,
        specs: { Size: "38" },
      },
    ],
  },
  {
    id: "P16",
    categoryId: "C_CLOTHES",
    title: "纯棉宽松连帽卫衣",
    description: "高级灰显白，320g重磅纯棉，不易变形，秋冬内搭外穿皆宜。",
    price: 159,
    imageUrl:
      "https://images.unsplash.com/photo-1556821840-3a63f95609a7?q=80&w=600&auto=format&fit=crop",
    salesVolume: 12000,
    rating: 4.7,
    tags: ["上装", "情侣款"],
  },
  {
    id: "P5",
    categoryId: "C_FOOD",
    title: "进口智利车厘子JJJ级 5斤装",
    description: "原箱进口，新鲜采摘，果径28-30mm，脆甜多汁。",
    price: 299,
    originalPrice: 359,
    imageUrl:
      "https://images.unsplash.com/photo-1528821128474-27f963b062bf?q=80&w=600&auto=format&fit=crop",
    salesVolume: 50000,
    rating: 4.6,
    tags: ["生鲜", "百亿补贴"],
  },
  {
    id: "P11",
    categoryId: "C_FOOD",
    title: "原切安格斯M5和牛眼肉牛排",
    description: "冷链极速配送，静腌厚切，口感软嫩多汁。",
    price: 158,
    imageUrl:
      "https://images.unsplash.com/photo-1546964124-0cce460f38ef?q=80&w=600&auto=format&fit=crop",
    salesVolume: 8900,
    rating: 4.9,
    tags: ["牛排", "生鲜包邮"],
  },
  {
    id: "P17",
    categoryId: "C_FOOD",
    title: "每日坚果 30包混合干果实惠装",
    description: "科学配比，无多余添加，孕妇儿童亦可放心食用。",
    price: 69,
    originalPrice: 99,
    imageUrl:
      "https://images.unsplash.com/photo-1596591606975-97ee5cef3a1e?q=80&w=600&auto=format&fit=crop",
    salesVolume: 210000,
    rating: 4.9,
    tags: ["零食", "健康"],
  },
  {
    id: "P8",
    categoryId: "C_BEAUTY",
    title: "小棕瓶抗老修护精华露",
    description: "深度修护，淡化细纹，紧致肌肤天然屏障。",
    price: 850,
    originalPrice: 1150,
    imageUrl:
      "https://images.unsplash.com/photo-1620916566398-39f1143ab7be?q=80&w=600&auto=format&fit=crop",
    salesVolume: 23000,
    rating: 4.9,
    tags: ["护肤", "爆款"],
    options: [{ name: "Capacity", values: ["30ml", "50ml", "75ml"] }],
    skus: [
      {
        id: "S8_1",
        name: "30ml",
        price: 650,
        stock: 300,
        specs: { Capacity: "30ml" },
      },
      {
        id: "S8_2",
        name: "50ml",
        price: 850,
        stock: 200,
        specs: { Capacity: "50ml" },
      },
      {
        id: "S8_3",
        name: "75ml",
        price: 1150,
        stock: 100,
        specs: { Capacity: "75ml" },
      },
    ],
  },
  {
    id: "P18",
    categoryId: "C_BEAUTY",
    title: "柔雾哑光唇膏 口红",
    description: "丝绒质地，浓郁显色不拔干，百搭显白热门色号推荐。",
    price: 320,
    imageUrl:
      "https://images.unsplash.com/photo-1586495777744-4413f21062fa?q=80&w=600&auto=format&fit=crop",
    salesVolume: 43000,
    rating: 4.8,
    tags: ["彩妆", "情人节礼物"],
  },
  {
    id: "P3",
    categoryId: "C_HOME",
    title: "智能扫拖一体机器人",
    description: "激光导航，自动回充，大吸力，全自动清洁。",
    price: 3299,
    originalPrice: 3599,
    imageUrl:
      "https://images.unsplash.com/photo-1589939705384-5185137a7f0f?q=80&w=600&auto=format&fit=crop",
    salesVolume: 3200,
    rating: 4.7,
    tags: ["智能家居"],
  },
  {
    id: "P10",
    categoryId: "C_HOME",
    title: "全自动意式胶囊咖啡机",
    description: "小巧迷你，19bar高压萃取，醇香油脂一键即享。",
    price: 599,
    originalPrice: 899,
    imageUrl:
      "https://images.unsplash.com/photo-1495908333425-29a1e0918c5f?q=80&w=600&auto=format&fit=crop",
    salesVolume: 12000,
    rating: 4.8,
    tags: ["家电", "精品推荐"],
  },
  {
    id: "P19",
    categoryId: "C_HOME",
    title: "100%纯棉无印风简约四件套",
    description: "长绒棉亲肤透气，A类面料，裸睡更安心，多色可选。",
    price: 249,
    imageUrl:
      "https://images.unsplash.com/photo-1522771739844-6a9f6d5f14af?q=80&w=600&auto=format&fit=crop",
    salesVolume: 35000,
    rating: 4.8,
    tags: ["家纺", "精选"],
  },
  {
    id: "P6",
    categoryId: "C_VIRTUAL",
    title: "ClawChat 会员季度卡",
    description: "专属标识，更多存储空间，高级网络通话权限。",
    price: 68,
    originalPrice: 88,
    imageUrl:
      "https://images.unsplash.com/photo-1614680376573-df3480f0c6ff?q=80&w=600&auto=format&fit=crop",
    salesVolume: 34000,
    rating: 5.0,
    tags: ["会员", "即时生效"],
    isVirtual: true,
    options: [{ name: "Type", values: ["季度卡 (3个月)", "年度卡 (12个月)"] }],
    skus: [
      {
        id: "S6_1",
        name: "季度卡 (3个月)",
        price: 68,
        stock: 999999,
        specs: { Type: "季度卡 (3个月)" },
      },
      {
        id: "S6_2",
        name: "年度卡 (12个月)",
        price: 238,
        stock: 999999,
        specs: { Type: "年度卡 (12个月)" },
      },
    ],
  },
  {
    id: "P20",
    categoryId: "C_VIRTUAL",
    title: "专业摄影后期课程 (视频版)",
    description: "顶尖摄影师授课，零基础入门到高级调色，永久有效。",
    price: 199,
    imageUrl:
      "https://images.unsplash.com/photo-1542038784456-1ea8e935640e?q=80&w=600&auto=format&fit=crop",
    salesVolume: 12000,
    rating: 4.9,
    tags: ["知识付费", "摄影"],
    isVirtual: true,
  },
  {
    id: "P7",
    categoryId: "C_COUPON",
    title: "瑞幸咖啡即用代金券",
    description: "全国门店通用，不限饮品类型，扫码即刻兑换，有效期30天。",
    price: 25,
    originalPrice: 29,
    imageUrl:
      "https://images.unsplash.com/photo-1559525839-b184a4d698c7?q=80&w=600&auto=format&fit=crop",
    salesVolume: 120500,
    rating: 4.9,
    tags: ["代金券", "秒发", "通用"],
    isVirtual: true,
    isCoupon: true,
    options: [
      {
        name: "FaceValue",
        values: [
          "29元面值 (售价25)",
          "55元面值 (售价48)",
          "100元面值 (售价88)",
        ],
      },
    ],
    skus: [
      {
        id: "S7_1",
        name: "29元面值 (售价25)",
        price: 25,
        stock: 50000,
        specs: { FaceValue: "29元面值 (售价25)" },
      },
      {
        id: "S7_2",
        name: "55元面值 (售价48)",
        price: 48,
        stock: 30000,
        specs: { FaceValue: "55元面值 (售价48)" },
      },
      {
        id: "S7_3",
        name: "100元面值 (售价88)",
        price: 88,
        stock: 15000,
        specs: { FaceValue: "100元面值 (售价88)" },
      },
    ],
  },
  {
    id: "P12",
    categoryId: "C_COUPON",
    title: "超市100元购物卡电子卡",
    description: "线下门店与线上商城通用，购买后立刻绑定账户使用。",
    price: 95,
    originalPrice: 100,
    imageUrl:
      "https://images.unsplash.com/photo-1607519961633-ab58807b1b59?q=80&w=600&auto=format&fit=crop",
    salesVolume: 56000,
    rating: 5.0,
    tags: ["购物卡", "充值"],
    isVirtual: true,
    isCoupon: true,
  },
  {
    id: "P21",
    categoryId: "C_VIRTUAL",
    title: "ClawChat AI 深度推理算力卡 (10M Tokens)",
    description: "可用于调用 ClawChat 平台内各主流大语言模型，包含 GPT-4o, Claude-3.5, Gemini-1.5, DeepSeek-R1。",
    price: 99,
    originalPrice: 150,
    imageUrl:
      "https://images.unsplash.com/photo-1620121692029-d088224ddc74?q=80&w=600&auto=format&fit=crop",
    salesVolume: 245000,
    rating: 5.0,
    tags: ["算力券", "API", "实时生效"],
    isVirtual: true,
    isCoupon: true,
  },
  {
    id: "P22",
    categoryId: "C_VIRTUAL",
    title: "AI 绘画企业级算力包 (100小时)",
    description: "提供高性能 GPU 算力，支持极速出图与高分辨率放大功能。",
    price: 139,
    originalPrice: 260,
    imageUrl:
      "https://images.unsplash.com/photo-1542831371-29b0f74f9713?q=80&w=600&auto=format&fit=crop",
    salesVolume: 56000,
    rating: 4.8,
    tags: ["AI绘画", "算力包", "秒发"],
    isVirtual: true,
    isCoupon: true,
  },
  {
    id: "P23",
    categoryId: "C_COUPON",
    title: "多平台年度影视会员兑换卡",
    description: "联合VIP年卡兑换券，视频平台皆可使用，追剧不等待。",
    price: 198,
    originalPrice: 398,
    imageUrl:
      "https://images.unsplash.com/photo-1593784991095-a205069470b6?q=80&w=600&auto=format&fit=crop",
    salesVolume: 128000,
    rating: 4.9,
    tags: ["兑换券", "联合年卡"],
    isVirtual: true,
    isCoupon: true,
  },
];

class MockShopService implements ShopService {
  private cart: CartItem[] = [
    {
      id: "CART1",
      productId: "P2",
      skuId: "S2_1",
      quantity: 1,
      selected: true,
    },
  ];
  private favorites: Set<string> = new Set(["P1", "P4"]);
  private orders: ShopOrder[] = [];

  async getCategories(): Promise<ShopCategory[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve(MOCK_CATEGORIES), 300),
    );
  }

  async getProducts(categoryId?: string): Promise<ShopProduct[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        if (categoryId) {
          resolve(MOCK_PRODUCTS.filter((p) => p.categoryId === categoryId));
        } else {
          resolve(MOCK_PRODUCTS);
        }
      }, 300);
    });
  }

  async getProductById(id: string): Promise<ShopProduct | null> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(MOCK_PRODUCTS.find((p) => p.id === id) || null);
      }, 300);
    });
  }

  async getCart(): Promise<CartItem[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...this.cart]), 300),
    );
  }

  async addToCart(
    productId: string,
    quantity = 1,
    skuId?: string,
  ): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const existing = this.cart.find(
          (c) => c.productId === productId && c.skuId === skuId,
        );
        if (existing) {
          existing.quantity += quantity;
        } else {
          this.cart.push({
            id: `CART_${Date.now()}`,
            productId,
            skuId,
            quantity,
            selected: true,
          });
        }
        resolve();
      }, 300);
    });
  }

  async updateCartItem(cartItemId: string, quantity: number): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const item = this.cart.find((c) => c.id === cartItemId);
        if (item) {
          item.quantity = quantity;
        }
        resolve();
      }, 300);
    });
  }

  async toggleCartItemSelection(cartItemId: string): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const item = this.cart.find((c) => c.id === cartItemId);
        if (item) {
          item.selected = !item.selected;
        }
        resolve();
      }, 300);
    });
  }

  async toggleAllCartItems(selected: boolean): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        this.cart.forEach((item) => (item.selected = selected));
        resolve();
      }, 300);
    });
  }

  async removeCartItem(cartItemId: string): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        this.cart = this.cart.filter((c) => c.id !== cartItemId);
        resolve();
      }, 300);
    });
  }

  async checkout(
    items?: CartItem[],
  ): Promise<{
    orderId: string;
    total: number;
    items: CartItem[];
    generatedCoupons?: { item: CartItem; product: ShopProduct; code: string }[];
  }> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const itemsToCheckout = items || this.cart.filter((c) => c.selected);
        let total = 0;
        const generatedCoupons: {
          item: CartItem;
          product: ShopProduct;
          code: string;
        }[] = [];
        const orderId = `ord_${Date.now()}`;

        for (const item of itemsToCheckout) {
          const product = MOCK_PRODUCTS.find((p) => p.id === item.productId);
          if (product) {
            const sku = product.skus?.find((s) => s.id === item.skuId);
            const price = sku ? sku.price : product.price;
            total += price * item.quantity;

            if (product.isCoupon) {
              for (let i = 0; i < item.quantity; i++) {
                const code = `${orderId.replace("ord_", "")}-${Math.floor(1000 + Math.random() * 9000)}-${i}`;
                generatedCoupons.push({ item, product, code });
              }
            }
          }
        }

        if (!items) {
          this.cart = this.cart.filter((c) => !c.selected);
        }

        const newOrder: ShopOrder = {
          id: orderId,
          items: itemsToCheckout,
          total,
          status: "paid",
          createdAt: Date.now(),
          coupons: generatedCoupons,
        };
        this.orders.push(newOrder);

        resolve({ orderId, total, items: itemsToCheckout, generatedCoupons });
      }, 500);
    });
  }

  async toggleFavorite(productId: string): Promise<boolean> {
    return new Promise((resolve) => {
      setTimeout(() => {
        if (this.favorites.has(productId)) {
          this.favorites.delete(productId);
          resolve(false);
        } else {
          this.favorites.add(productId);
          resolve(true);
        }
      }, 200);
    });
  }

  async isFavorite(productId: string): Promise<boolean> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(this.favorites.has(productId));
      }, 100);
    });
  }

  async getOrders(): Promise<ShopOrder[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([...this.orders].sort((a, b) => b.createdAt - a.createdAt));
      }, 300);
    });
  }
}

export const shopService = new MockShopService();
