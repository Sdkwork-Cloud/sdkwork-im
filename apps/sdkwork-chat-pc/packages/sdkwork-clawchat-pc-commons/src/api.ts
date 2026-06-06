export const mockConsoleFetch = async <T,>(endpoint: string, mockData: T, delayMs: number = 300): Promise<T> => {
  // 仅在控制台打印模拟请求，以标明实际请求会以 /console/api 开始
  console.log(`[Mock Fetch Backend] GET /console/api/v1${endpoint}`);
  await new Promise(resolve => setTimeout(resolve, delayMs));
  return mockData;
};

export const mockAdminFetch = async <T,>(endpoint: string, mockData: T, delayMs: number = 300): Promise<T> => {
  console.log(`[Mock Backend] GET /admin/api/v1${endpoint}`);
  await new Promise(resolve => setTimeout(resolve, delayMs));
  return mockData;
};

export const mockConsolePost = async <T,>(endpoint: string, payload: any, mockData: T, delayMs: number = 400): Promise<T> => {
  console.log(`[Mock Backend] POST /console/api/v1${endpoint}`, payload);
  await new Promise(resolve => setTimeout(resolve, delayMs));
  return mockData;
};

export const mockAdminPost = async <T,>(endpoint: string, payload: any, mockData: T, delayMs: number = 400): Promise<T> => {
  console.log(`[Mock Backend] POST /admin/api/v1${endpoint}`, payload);
  await new Promise(resolve => setTimeout(resolve, delayMs));
  return mockData;
};
