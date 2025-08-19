// Mock authentication for demo purposes since backend needs fixes
export interface LoginData {
  username: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user_id: number;
  username: string;
}

export const mockLogin = async (data: LoginData): Promise<LoginResponse> => {
  // Simulate API delay
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  if (data.username === 'admin' && data.password === 'admin123') {
    return {
      token: 'mock_admin_token_' + Date.now(),
      user_id: 1,
      username: 'admin'
    };
  }
  
  throw new Error('Invalid credentials');
};

export const mockVerifyToken = async (token: string): Promise<{valid: boolean, user_id: number, username: string}> => {
  await new Promise(resolve => setTimeout(resolve, 500));
  
  if (token.startsWith('mock_admin_token_')) {
    return {
      valid: true,
      user_id: 1,
      username: 'admin'
    };
  }
  
  throw new Error('Invalid token');
};

// Mock data for demo
export const mockNotices = [
  {
    id: 1,
    title: '欢迎来到LC PILATES空中普拉提',
    content: '欢迎大家加入我们的瑜伽大家庭，开启健康生活新篇章！这里有专业的导师团队，完善的设施设备，丰富多样的课程选择。',
    author: '管理员',
    priority: 10,
    is_active: true,
    created_at: '2024-01-15 10:30:00'
  },
  {
    id: 2,
    title: '新会员优惠活动',
    content: '新会员首月享受8折优惠！购买年卡更有超值礼品赠送，名额有限，先到先得。',
    author: '市场部',
    priority: 8,
    is_active: true,
    created_at: '2024-01-14 14:20:00'
  }
];

export const mockTeachers = [
  {
    id: 1,
    name: '张老师',
    description: '资深瑜伽导师，专业教学10年经验',
    avatar_url: 'teacher1.jpg',
    bio: '拥有丰富的瑜伽教学经验，擅长哈他瑜伽和阴瑜伽',
    certifications: ['RYT-200', 'RYT-500'],
    specialties: ['哈他瑜伽', '阴瑜伽', '初学者指导'],
    experience_years: 10,
    average_rating: 4.8,
    total_ratings: 156,
    is_active: true,
    created_at: '2023-06-01 09:00:00'
  },
  {
    id: 2,
    name: '李老师',
    description: '普拉提专业导师',
    avatar_url: 'teacher2.jpg',
    bio: '专业普拉提导师，注重身体力量训练和体态矫正',
    certifications: ['Pilates Certificate'],
    specialties: ['普拉提', '体态矫正', '力量训练'],
    experience_years: 8,
    average_rating: 4.9,
    total_ratings: 98,
    is_active: true,
    created_at: '2023-07-15 11:30:00'
  }
];

export const mockPosters = [
  {
    id: 1,
    title: '瑜伽生活，从这里开始',
    image: 'banner1.jpg',
    link_url: '/pages/booking/booking',
    sort_order: 1,
    is_active: true,
    start_date: null,
    end_date: null,
    created_at: '2024-01-01 12:00:00'
  },
  {
    id: 2,
    title: '明星教师介绍',
    image: 'banner2.jpg',
    link_url: '/pages/teacher/teacher?id=1',
    sort_order: 2,
    is_active: true,
    start_date: null,
    end_date: null,
    created_at: '2024-01-02 12:00:00'
  }
];

export const mockActionButtons = [
  {
    id: 1,
    name: '瑜伽',
    icon: 'https://cdn.example.com/icons/yoga.png',
    link: '/pages/lessons/lessons',
    sort_order: 1,
    is_active: true,
    created_at: '2024-01-01 12:00:00'
  },
  {
    id: 2,
    name: '约团课',
    icon: 'https://cdn.example.com/icons/group-class.png',
    link: '/pages/booking/booking',
    sort_order: 2,
    is_active: true,
    created_at: '2024-01-01 12:00:00'
  }
];