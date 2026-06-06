export interface AttendanceStats {
  presentDays: number;
  lateTimes: number;
  earlyLeaveTimes: number;
  missedPunches: number;
}

export interface AttendanceRecord {
  id: string;
  date: string;
  type: string;
  status: "pending" | "resolved";
}

export interface AttendanceStatus {
  isCheckedIn: boolean;
  isCheckedOut: boolean;
  checkInTime: string | null;
  checkOutTime: string | null;
  location: string;
  inRange: boolean;
}

export const attendanceService = {
  getStats: async (): Promise<AttendanceStats> => {
    return {
      presentDays: 12,
      lateTimes: 0,
      earlyLeaveTimes: 0,
      missedPunches: 1
    };
  },

  getRecords: async (): Promise<AttendanceRecord[]> => {
    return [
      { id: "1", date: "5月20日 星期四", type: "下班缺卡", status: "pending" },
      { id: "2", date: "5月10日 星期一", type: "迟到 15分钟", status: "resolved" }
    ];
  },

  getCurrentStatus: async (): Promise<AttendanceStatus> => {
    return {
      isCheckedIn: false,
      isCheckedOut: false,
      checkInTime: null,
      checkOutTime: null,
      location: "科技园C栋",
      inRange: true
    };
  },

  punchIn: async (): Promise<{ success: boolean; time: string }> => {
    const time = new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    return { success: true, time };
  },

  punchOut: async (): Promise<{ success: boolean; time: string }> => {
    const time = new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    return { success: true, time };
  },

  submitAppeal: async (recordId: string, reason: string): Promise<boolean> => {
    // Submit appeal logic here
    console.log(`Submitting appeal for ${recordId}: ${reason}`);
    return true;
  }
};
