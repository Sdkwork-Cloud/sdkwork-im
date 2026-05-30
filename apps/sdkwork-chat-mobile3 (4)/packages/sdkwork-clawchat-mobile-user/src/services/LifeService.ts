import { Smartphone, Zap, Umbrella, Coffee } from "lucide-react";

export interface LifeServiceItem {
  iconName: string;
  label: string;
  color: string;
}

const MOCK_LIFE_SERVICES: LifeServiceItem[] = [
  { iconName: "Smartphone", label: "手机充值", color: "text-blue-500" },
  { iconName: "Zap", label: "生活缴费", color: "text-yellow-500" },
  { iconName: "Umbrella", label: "城市服务", color: "text-green-500" },
  { iconName: "Coffee", label: "外卖", color: "text-orange-500" },
];

export const LifeService = {
  getLifeServices: async (): Promise<LifeServiceItem[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...MOCK_LIFE_SERVICES]), 100),
    );
  },
};
