export interface Device {
  id: string;
  name: string;
  type: 'camera' | 'speaker' | 'display' | 'sensor' | 'other';
  status: 'online' | 'offline' | 'error' | 'unactivated';
  agentId?: string; // Associated Agent ID
  macAddress: string;
  firmwareVersion: string;
}

export interface DeviceService {
  getDevices(): Promise<Device[]>;
  getDevice(id: string): Promise<Device | undefined>;
  addDevice(device: Omit<Device, 'id'>): Promise<Device>;
  updateDevice(id: string, device: Partial<Device>): Promise<void>;
  deleteDevice(id: string): Promise<void>;
  bindAgent(deviceId: string, agentId: string): Promise<void>;
  unbindAgent(deviceId: string): Promise<void>;
  activateDevice(deviceId: string, activationCode: string): Promise<void>;
}

const STANDARD_AGENT_ID_PATTERN = /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u;

function requireStandardAgentBindingId(agentId: string): string {
  const normalized = agentId.trim();
  if (!STANDARD_AGENT_ID_PATTERN.test(normalized)) {
    throw new Error('Device agent binding id must use the standard agent. id format');
  }
  return normalized;
}

const mockDevices: Device[] = [
  {
    id: 'dev-001',
    name: '会议室主摄像头',
    type: 'camera',
    status: 'online',
    agentId: 'agent.device.vision',
    macAddress: '00:1A:2B:3C:4D:5E',
    firmwareVersion: '1.2.4',
  },
  {
    id: 'dev-002',
    name: '前台迎宾屏幕',
    type: 'display',
    status: 'online',
    macAddress: '00:1A:2B:3C:4D:5F',
    firmwareVersion: '2.0.1',
  },
  {
    id: 'dev-003',
    name: '休息区智能音箱',
    type: 'speaker',
    status: 'offline',
    agentId: 'agent.device.voice',
    macAddress: '00:1A:2B:3C:4D:60',
    firmwareVersion: '1.0.5',
  },
  {
    id: 'dev-004',
    name: '车间测试设备',
    type: 'sensor',
    status: 'unactivated',
    macAddress: '00:1A:2B:3C:4D:61',
    firmwareVersion: '0.9.0',
  }
];

class MockDeviceService implements DeviceService {
  async getDevices(): Promise<Device[]> {
    return new Promise(resolve => setTimeout(() => resolve([...mockDevices]), 300));
  }

  async getDevice(id: string): Promise<Device | undefined> {
    return new Promise(resolve => setTimeout(() => resolve(mockDevices.find(d => d.id === id)), 100));
  }

  async addDevice(device: Omit<Device, 'id'>): Promise<Device> {
    return new Promise(resolve => {
      setTimeout(() => {
        const newDevice: Device = {
          ...device,
          id: `dev-${Date.now()}`
        };
        mockDevices.push(newDevice);
        resolve(newDevice);
      }, 300);
    });
  }

  async updateDevice(id: string, device: Partial<Device>): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const index = mockDevices.findIndex(d => d.id === id);
        if (index > -1) {
          mockDevices[index] = { ...mockDevices[index], ...device };
        }
        resolve();
      }, 300);
    });
  }

  async deleteDevice(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const index = mockDevices.findIndex(d => d.id === id);
        if (index > -1) {
          mockDevices.splice(index, 1);
        }
        resolve();
      }, 300);
    });
  }

  async bindAgent(deviceId: string, agentId: string): Promise<void> {
    return this.updateDevice(deviceId, { agentId: requireStandardAgentBindingId(agentId) });
  }

  async unbindAgent(deviceId: string): Promise<void> {
    return this.updateDevice(deviceId, { agentId: undefined });
  }

  async activateDevice(deviceId: string, activationCode: string): Promise<void> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (activationCode.length < 6) {
          reject(new Error('无效的激活码'));
          return;
        }
        this.updateDevice(deviceId, { status: 'offline' }).then(resolve);
      }, 500);
    });
  }
}

export const deviceService = new MockDeviceService();
