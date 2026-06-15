export interface CalendarEvent {
  id: string;
  title: string;
  date: string; // YYYY-MM-DD
  time: string; // HH:mm
  color: string;
  location?: string;
  description?: string;
  guests?: string[];
}

export interface CalendarList {
  id: string;
  name: string;
  color: string;
  checked: boolean;
}

export interface CalendarService {
  getEvents(startDate: string, endDate: string): Promise<CalendarEvent[]>;
  createEvent(event: Omit<CalendarEvent, 'id'>): Promise<CalendarEvent>;
  updateEvent(id: string, updates: Partial<CalendarEvent>): Promise<CalendarEvent>;
  deleteEvent(id: string): Promise<void>;
  getCalendars(): Promise<CalendarList[]>;
  createCalendar(calendar: Omit<CalendarList, 'id'>): Promise<CalendarList>;
  updateCalendar(id: string, updates: Partial<CalendarList>): Promise<CalendarList>;
  deleteCalendar(id: string): Promise<void>;
}

const today = new Date();
const year = today.getFullYear();
const month = String(today.getMonth() + 1).padStart(2, '0');
const day = String(today.getDate()).padStart(2, '0');

const tomorrow = new Date(today);
tomorrow.setDate(tomorrow.getDate() + 1);
const tYear = tomorrow.getFullYear();
const tMonth = String(tomorrow.getMonth() + 1).padStart(2, '0');
const tDay = String(tomorrow.getDate()).padStart(2, '0');

let mockEvents: CalendarEvent[] = [
  { id: '1', title: '产品迭代评审会', date: `${year}-${month}-${day}`, time: '10:00', color: '#ea4335', location: '会议室 A', description: '本次会议旨在对齐下个季度的产品目标，整理需要解决的关键技术债务和体验优化点，请提前准备好相关材料。', guests: ['张三', '李四', '王五', '赵六', '孙七', '周八'] },
  { id: '2', title: '设计稿走查', date: `${year}-${month}-${day}`, time: '14:30', color: '#1a73e8', location: '线上会议 (Zoom)', guests: ['李四', '设计师小王'] },
  { id: '3', title: '客户需求对齐', date: `${year}-${month}-${day}`, time: '16:00', color: '#fbbc05' },
  { id: '4', title: '周总结会议', date: `${tYear}-${tMonth}-${tDay}`, time: '15:00', color: '#34a853' },
  { id: '5', title: '性能优化讨论', date: `${tYear}-${tMonth}-${tDay}`, time: '10:30', color: '#a142f4' }
];

let mockCalendars: CalendarList[] = [
  { id: '1', name: '我的工作日程', color: '#ea4335', checked: true },
  { id: '2', name: '团队会议', color: '#1a73e8', checked: true },
  { id: '3', name: '个人备忘录', color: '#fbbc05', checked: true },
  { id: '4', name: '节假日', color: '#34a853', checked: false },
];

class MockCalendarService implements CalendarService {
  async getEvents(startDate: string, endDate: string): Promise<CalendarEvent[]> {
    return new Promise(resolve => {
      setTimeout(() => {
        // filter by date range later if needed. For now return all.
        resolve([...mockEvents]);
      }, 200);
    });
  }

  async createEvent(event: Omit<CalendarEvent, 'id'>): Promise<CalendarEvent> {
    return new Promise(resolve => {
      setTimeout(() => {
        const newEvent: CalendarEvent = {
          ...event,
          id: Date.now().toString(),
        };
        mockEvents = [...mockEvents, newEvent];
        resolve(newEvent);
      }, 300);
    });
  }

  async updateEvent(id: string, updates: Partial<CalendarEvent>): Promise<CalendarEvent> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const idx = mockEvents.findIndex(e => e.id === id);
        if (idx === -1) return reject(new Error('Event not found'));
        mockEvents[idx] = { ...mockEvents[idx], ...updates };
        resolve(mockEvents[idx]);
      }, 300);
    });
  }

  async deleteEvent(id: string): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        mockEvents = mockEvents.filter(e => e.id !== id);
        resolve();
      }, 300);
    });
  }

  async getCalendars(): Promise<CalendarList[]> {
    return new Promise((resolve) => {
      setTimeout(() => resolve([...mockCalendars]), 200);
    });
  }

  async createCalendar(calendar: Omit<CalendarList, 'id'>): Promise<CalendarList> {
    return new Promise(resolve => {
      setTimeout(() => {
        const newCalendar: CalendarList = {
          ...calendar,
          id: Date.now().toString(),
        };
        mockCalendars = [...mockCalendars, newCalendar];
        resolve(newCalendar);
      }, 200);
    });
  }

  async updateCalendar(id: string, updates: Partial<CalendarList>): Promise<CalendarList> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const idx = mockCalendars.findIndex(c => c.id === id);
        if (idx === -1) return reject(new Error('Calendar not found'));
        mockCalendars[idx] = { ...mockCalendars[idx], ...updates };
        resolve(mockCalendars[idx]);
      }, 200);
    });
  }

  async deleteCalendar(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        mockCalendars = mockCalendars.filter(c => c.id !== id);
        resolve();
      }, 200);
    });
  }
}

export const calendarService = new MockCalendarService();
