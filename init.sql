-- 初始化瑜伽约课数据库
-- Initialize yoga booking database

-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    open_id VARCHAR(255) UNIQUE NOT NULL,
    nick_name VARCHAR(255),
    avatar_url TEXT,
    phone VARCHAR(20),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_admin BOOLEAN DEFAULT FALSE
);

-- 创建后台管理员表
CREATE TABLE IF NOT EXISTS admin_users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- 创建课程类型枚举
CREATE TYPE lesson_type AS ENUM ('team', 'small_class', 'private', 'equipment_small_class', 'workshop');

-- 创建难度等级枚举  
CREATE TYPE difficulty_level AS ENUM ('beginner', 'intermediate', 'advanced', 'all_levels');

-- 创建会员卡类型枚举
CREATE TYPE membership_card_type AS ENUM ('unlimited', 'count_based');

-- 创建会员卡状态枚举
CREATE TYPE membership_card_status AS ENUM ('active', 'expired', 'suspended', 'used_up');

-- 创建教师表
CREATE TABLE IF NOT EXISTS teachers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    avatar_url TEXT,
    bio TEXT, -- 教师简历
    certifications TEXT[], -- 认证资质数组
    specialties TEXT[], -- 专长领域数组
    experience_years INTEGER DEFAULT 0, -- 教学经验年数
    average_rating DECIMAL(2,1) DEFAULT 0.0 CHECK (average_rating >= 0.0 AND average_rating <= 5.0),
    total_ratings INTEGER DEFAULT 0, -- 总评分次数
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- 创建地点/教室表
CREATE TABLE IF NOT EXISTS locations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE, -- 教室名称，如"A教室", "空中瑜伽室", "普拉提室"
    description TEXT, -- 教室描述
    capacity INTEGER NOT NULL DEFAULT 20, -- 教室容量
    equipment TEXT[], -- 教室设备，如["瑜伽垫", "空中吊带", "普拉提器械"]
    facilities TEXT[], -- 教室设施，如["更衣室", "淋浴间", "储物柜"]
    floor_number INTEGER, -- 楼层
    room_number VARCHAR(50), -- 房间号
    is_accessible BOOLEAN DEFAULT TRUE, -- 无障碍设施
    booking_enabled BOOLEAN DEFAULT TRUE, -- 是否允许预订
    hourly_rate DECIMAL(10,2), -- 每小时租金（如果支持教室租赁）
    images TEXT[], -- 教室图片
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    
    CONSTRAINT valid_capacity CHECK (capacity > 0)
);

-- 创建课程表 (增强版)
CREATE TABLE IF NOT EXISTS lessons (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    teacher_id INTEGER REFERENCES teachers(id),
    location_id INTEGER REFERENCES locations(id),
    lesson_type lesson_type NOT NULL DEFAULT 'team',
    difficulty_level difficulty_level NOT NULL DEFAULT 'all_levels',
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    max_students INTEGER NOT NULL,
    current_students INTEGER DEFAULT 0,
    price DECIMAL(10,2) DEFAULT 0.00, -- 课程价格
    equipment_required TEXT[], -- 所需器材
    prerequisites TEXT, -- 先决条件
    cancellation_policy TEXT, -- 取消政策
    notes TEXT, -- 课程备注
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    
    -- 添加约束确保时间合理性
    CONSTRAINT valid_time_range CHECK (end_time > start_time),
    CONSTRAINT valid_max_students CHECK (max_students > 0),
    CONSTRAINT valid_current_students CHECK (current_students >= 0 AND current_students <= max_students)
);

-- 创建预约状态枚举
CREATE TYPE booking_status AS ENUM ('pending', 'confirmed', 'cancelled', 'completed', 'no_show');

-- 创建预约表 (增强版)
CREATE TABLE IF NOT EXISTS bookings (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    lesson_id INTEGER REFERENCES lessons(id),
    booking_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    status booking_status DEFAULT 'confirmed',
    notes TEXT,
    payment_status VARCHAR(20) DEFAULT 'pending', -- pending, paid, refunded
    payment_amount DECIMAL(10,2),
    cancellation_reason TEXT,
    cancelled_at TIMESTAMP WITH TIME ZONE,
    attended BOOLEAN DEFAULT NULL, -- NULL表示未确定，TRUE/FALSE表示是否出席
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, lesson_id)
);

-- 创建教师评分表
CREATE TABLE IF NOT EXISTS teacher_ratings (
    id SERIAL PRIMARY KEY,
    teacher_id INTEGER REFERENCES teachers(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    lesson_id INTEGER REFERENCES lessons(id) ON DELETE CASCADE,
    rating DECIMAL(2,1) NOT NULL CHECK (rating >= 0.0 AND rating <= 5.0),
    review TEXT,
    rating_categories JSONB, -- 存储不同维度的评分，如：{"teaching": 4.5, "communication": 5.0, "professionalism": 4.0}
    is_anonymous BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- 确保每个用户对每个老师的每堂课只能评分一次
    UNIQUE(user_id, teacher_id, lesson_id)
);

-- 创建评分标准表
CREATE TABLE IF NOT EXISTS rating_criteria (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 评分标准名称，如"教学能力"、"沟通技巧"等
    description TEXT,
    weight DECIMAL(3,2) DEFAULT 1.00, -- 权重，用于计算加权平均分
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 创建通知公告表
CREATE TABLE IF NOT EXISTS notices (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    priority INTEGER DEFAULT 0
);

-- 创建调试日志表（用于记录设备信息）
CREATE TABLE IF NOT EXISTS debug_logs (
    id SERIAL PRIMARY KEY,
    open_id VARCHAR(255),
    brand VARCHAR(255),
    model VARCHAR(255),
    pixel_ratio DECIMAL,
    screen_height INTEGER,
    screen_width INTEGER,
    version VARCHAR(255),
    sdk_version VARCHAR(255),
    platform VARCHAR(255),
    ip_address VARCHAR(45),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 创建轮播图表 (poster表)
CREATE TABLE IF NOT EXISTS posters (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    image VARCHAR(255) NOT NULL, -- 图片文件名
    link_url TEXT, -- 跳转链接
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    start_date TIMESTAMP WITH TIME ZONE,
    end_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 创建功能按钮表 (用于首页actions)
CREATE TABLE IF NOT EXISTS action_buttons (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    icon VARCHAR(100),
    action_type VARCHAR(20) DEFAULT 'navigate', -- navigate, external, function
    action_value INTEGER NOT NULL, -- 对应的动作值
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 创建商城信息表
CREATE TABLE IF NOT EXISTS market_info (
    id SERIAL PRIMARY KEY,
    slogan TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 创建会员卡套餐表 (卡的模板)
CREATE TABLE IF NOT EXISTS membership_plans (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL, -- 套餐名称，如"年卡", "半年卡", "20次卡"
    description TEXT,
    card_type membership_card_type NOT NULL, -- unlimited 或 count_based
    validity_days INTEGER NOT NULL, -- 有效期天数
    total_classes INTEGER, -- 如果是次数卡，总次数；如果是不限次卡则为NULL
    price DECIMAL(10,2) NOT NULL, -- 价格
    original_price DECIMAL(10,2), -- 原价，用于显示优惠
    applicable_lesson_types lesson_type[], -- 适用的课程类型数组，NULL表示全部适用
    max_bookings_per_day INTEGER DEFAULT 1, -- 每天最多可约课数
    transfer_allowed BOOLEAN DEFAULT FALSE, -- 是否允许转让
    refund_allowed BOOLEAN DEFAULT FALSE, -- 是否允许退款
    benefits TEXT[], -- 会员卡特权描述
    restrictions TEXT[], -- 使用限制描述
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- 约束：次数卡必须有总次数，不限次卡不需要
    CONSTRAINT check_count_based_has_total_classes CHECK (
        (card_type = 'count_based' AND total_classes IS NOT NULL AND total_classes > 0) OR 
        (card_type = 'unlimited' AND total_classes IS NULL)
    ),
    CONSTRAINT check_validity_days_positive CHECK (validity_days > 0),
    CONSTRAINT check_price_non_negative CHECK (price >= 0)
);

-- 创建用户会员卡表 (用户实际持有的卡)
CREATE TABLE IF NOT EXISTS user_membership_cards (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plan_id INTEGER NOT NULL REFERENCES membership_plans(id),
    card_number VARCHAR(50) UNIQUE NOT NULL, -- 卡号，自动生成
    status membership_card_status DEFAULT 'active',
    
    -- 卡的基本信息 (从plan复制过来，避免plan变更影响已售出的卡)
    card_type membership_card_type NOT NULL,
    plan_name VARCHAR(255) NOT NULL, -- 套餐名称快照
    validity_days INTEGER NOT NULL, -- 有效期天数快照
    total_classes INTEGER, -- 总次数快照 (仅次数卡)
    remaining_classes INTEGER, -- 剩余次数 (仅次数卡)
    
    -- 时间信息
    purchased_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 购买时间
    activated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 激活时间
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL, -- 过期时间
    
    -- 价格信息
    purchase_price DECIMAL(10,2) NOT NULL,
    discount_amount DECIMAL(10,2) DEFAULT 0, -- 优惠金额
    actual_paid DECIMAL(10,2) NOT NULL, -- 实际支付金额
    
    -- 使用限制 (从plan复制)
    applicable_lesson_types lesson_type[],
    max_bookings_per_day INTEGER DEFAULT 1,
    transfer_allowed BOOLEAN DEFAULT FALSE,
    refund_allowed BOOLEAN DEFAULT FALSE,
    
    -- 状态信息
    suspended_at TIMESTAMP WITH TIME ZONE, -- 暂停时间
    suspended_reason TEXT, -- 暂停原因
    notes TEXT, -- 备注
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- 约束
    CONSTRAINT check_remaining_classes_valid CHECK (
        (card_type = 'unlimited') OR 
        (card_type = 'count_based' AND remaining_classes >= 0 AND remaining_classes <= total_classes)
    ),
    CONSTRAINT check_actual_paid_valid CHECK (actual_paid >= 0),
    CONSTRAINT check_discount_valid CHECK (discount_amount >= 0),
    CONSTRAINT check_price_calculation CHECK (actual_paid = purchase_price - discount_amount)
);

-- 创建会员卡使用记录表
CREATE TABLE IF NOT EXISTS membership_card_usage (
    id SERIAL PRIMARY KEY,
    user_card_id INTEGER NOT NULL REFERENCES user_membership_cards(id) ON DELETE CASCADE,
    booking_id INTEGER REFERENCES bookings(id) ON DELETE SET NULL, -- 关联的预约记录
    lesson_id INTEGER NOT NULL REFERENCES lessons(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    
    -- 使用信息
    usage_type VARCHAR(20) NOT NULL DEFAULT 'booking', -- booking, refund
    classes_consumed INTEGER DEFAULT 1, -- 消耗的次数，退款时为负数
    used_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- 使用时卡的状态快照
    remaining_classes_before INTEGER, -- 使用前剩余次数
    remaining_classes_after INTEGER, -- 使用后剩余次数
    
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT check_classes_consumed_not_zero CHECK (classes_consumed != 0)
);

-- 创建索引 (优化版)
CREATE INDEX IF NOT EXISTS idx_users_open_id ON users(open_id);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

CREATE INDEX IF NOT EXISTS idx_admin_users_username ON admin_users(username);
CREATE INDEX IF NOT EXISTS idx_admin_users_active ON admin_users(is_active);

CREATE INDEX IF NOT EXISTS idx_teachers_active ON teachers(is_active);
CREATE INDEX IF NOT EXISTS idx_teachers_rating ON teachers(average_rating DESC);

CREATE INDEX IF NOT EXISTS idx_locations_active ON locations(is_active);
CREATE INDEX IF NOT EXISTS idx_locations_booking_enabled ON locations(booking_enabled);
CREATE INDEX IF NOT EXISTS idx_locations_capacity ON locations(capacity);
CREATE INDEX IF NOT EXISTS idx_locations_name ON locations(name);

CREATE INDEX IF NOT EXISTS idx_lessons_start_time ON lessons(start_time);
CREATE INDEX IF NOT EXISTS idx_lessons_teacher_id ON lessons(teacher_id);
CREATE INDEX IF NOT EXISTS idx_lessons_location_id ON lessons(location_id);
CREATE INDEX IF NOT EXISTS idx_lessons_type ON lessons(lesson_type);
CREATE INDEX IF NOT EXISTS idx_lessons_difficulty ON lessons(difficulty_level);
CREATE INDEX IF NOT EXISTS idx_lessons_active ON lessons(is_active);
CREATE INDEX IF NOT EXISTS idx_lessons_time_active ON lessons(start_time, is_active);

CREATE INDEX IF NOT EXISTS idx_bookings_user_id ON bookings(user_id);
CREATE INDEX IF NOT EXISTS idx_bookings_lesson_id ON bookings(lesson_id);
CREATE INDEX IF NOT EXISTS idx_bookings_status ON bookings(status);
CREATE INDEX IF NOT EXISTS idx_bookings_created_at ON bookings(created_at);
CREATE INDEX IF NOT EXISTS idx_bookings_user_lesson ON bookings(user_id, lesson_id);

CREATE INDEX IF NOT EXISTS idx_teacher_ratings_teacher_id ON teacher_ratings(teacher_id);
CREATE INDEX IF NOT EXISTS idx_teacher_ratings_user_id ON teacher_ratings(user_id);
CREATE INDEX IF NOT EXISTS idx_teacher_ratings_lesson_id ON teacher_ratings(lesson_id);
CREATE INDEX IF NOT EXISTS idx_teacher_ratings_created_at ON teacher_ratings(created_at);

CREATE INDEX IF NOT EXISTS idx_notices_created_at ON notices(created_at);
CREATE INDEX IF NOT EXISTS idx_notices_active ON notices(is_active);

CREATE INDEX IF NOT EXISTS idx_debug_logs_open_id ON debug_logs(open_id);
CREATE INDEX IF NOT EXISTS idx_debug_logs_created_at ON debug_logs(created_at);

CREATE INDEX IF NOT EXISTS idx_posters_active ON posters(is_active);
CREATE INDEX IF NOT EXISTS idx_posters_sort ON posters(sort_order);

CREATE INDEX IF NOT EXISTS idx_action_buttons_active ON action_buttons(is_active);
CREATE INDEX IF NOT EXISTS idx_action_buttons_sort ON action_buttons(sort_order);

-- 会员卡相关索引
CREATE INDEX IF NOT EXISTS idx_membership_plans_active ON membership_plans(is_active);
CREATE INDEX IF NOT EXISTS idx_membership_plans_type ON membership_plans(card_type);
CREATE INDEX IF NOT EXISTS idx_membership_plans_sort ON membership_plans(sort_order);

CREATE INDEX IF NOT EXISTS idx_user_membership_cards_user_id ON user_membership_cards(user_id);
CREATE INDEX IF NOT EXISTS idx_user_membership_cards_status ON user_membership_cards(status);
CREATE INDEX IF NOT EXISTS idx_user_membership_cards_expires ON user_membership_cards(expires_at);
CREATE INDEX IF NOT EXISTS idx_user_membership_cards_card_number ON user_membership_cards(card_number);
CREATE INDEX IF NOT EXISTS idx_user_membership_cards_user_status ON user_membership_cards(user_id, status);

CREATE INDEX IF NOT EXISTS idx_membership_card_usage_user_card ON membership_card_usage(user_card_id);
CREATE INDEX IF NOT EXISTS idx_membership_card_usage_booking ON membership_card_usage(booking_id);
CREATE INDEX IF NOT EXISTS idx_membership_card_usage_user ON membership_card_usage(user_id);
CREATE INDEX IF NOT EXISTS idx_membership_card_usage_used_at ON membership_card_usage(used_at);

-- 插入示例数据

-- 插入地点/教室数据
INSERT INTO locations (name, description, capacity, equipment, facilities, floor_number, room_number, hourly_rate, images) VALUES 
('A教室（主厅）', '宽敞明亮的主教室，适合大团课和各种瑜伽练习', 30, ARRAY['瑜伽垫', '瑜伽砖', '拉力带', '音响设备'], ARRAY['更衣室', '储物柜', '饮水机', '空调'], 2, 'A201', 200.00, ARRAY['room_a1.jpg', 'room_a2.jpg']),
('B教室（小班课）', '温馨的小班教室，适合精品小班课程', 15, ARRAY['瑜伽垫', '瑜伽砖', '抱枕', '毛毯'], ARRAY['储物柜', '空调', '镜子'], 2, 'B202', 150.00, ARRAY['room_b1.jpg']),
('空中瑜伽室', '专业的空中瑜伽练习室，配备专业吊床设备', 12, ARRAY['空中吊床', '瑜伽垫', '安全垫', '专业音响'], ARRAY['更衣室', '淋浴间', '储物柜'], 3, 'C301', 300.00, ARRAY['aerial_room1.jpg', 'aerial_room2.jpg']),
('普拉提器械室', '配备专业普拉提器械的练习室', 8, ARRAY['Reformer床', '稳踏椅', '梯桶', '弹簧床'], ARRAY['储物柜', '毛巾架', '空调'], 3, 'C302', 400.00, ARRAY['pilates_room1.jpg']),
('冥想室', '安静舒适的冥想和修复瑜伽练习空间', 20, ARRAY['冥想坐垫', '毛毯', '精油香薰', '柔和灯光'], ARRAY['静音空调', '储物柜'], 1, 'D101', 120.00, ARRAY['meditation_room1.jpg']),
('私教室', '一对一私教课程专用房间', 2, ARRAY['全套瑜伽用品', '辅助道具', '音响设备'], ARRAY['独立更衣区', '储物空间'], 1, 'D102', 500.00, ARRAY['private_room1.jpg'])
ON CONFLICT (name) DO NOTHING;

-- 插入教师数据
INSERT INTO teachers (name, description, avatar_url, bio, certifications, specialties, experience_years) VALUES 
('张老师', '资深瑜伽导师，专业教学10年经验', 'teacher1.jpg', '拥有丰富的瑜伽教学经验，擅长哈他瑜伽和阴瑜伽', ARRAY['RYT-200', 'RYT-500'], ARRAY['哈他瑜伽', '阴瑜伽', '初学者指导'], 10),
('李老师', '普拉提专业导师', 'teacher2.jpg', '专业普拉提导师，注重身体力量训练和体态矫正', ARRAY['Pilates Certificate'], ARRAY['普拉提', '体态矫正', '力量训练'], 8),
('王老师', '空中瑜伽导师', 'teacher3.jpg', '空中瑜伽专业导师，带你体验不一样的瑜伽练习', ARRAY['Aerial Yoga Certificate'], ARRAY['空中瑜伽', '体式创新'], 6)
ON CONFLICT DO NOTHING;

-- 插入通知公告数据
INSERT INTO notices (title, content, author) VALUES 
('欢迎来到瑜伽馆', '欢迎大家加入我们的瑜伽大家庭，开启健康生活新篇章！', '管理员'),
('课程安排通知', '本周新增晚间课程，欢迎大家踊跃报名参加。', '教务处'),
('节假日营业通知', '春节期间营业时间调整，具体安排请查看详情。', '管理员')
ON CONFLICT DO NOTHING;

-- 插入轮播图数据
INSERT INTO posters (title, image, link_url, sort_order) VALUES 
('瑜伽生活，从这里开始', 'banner1.jpg', '/pages/booking/booking', 1),
('明星教师介绍', 'banner2.jpg', '/pages/teacher/teacher?id=1', 2),
('课程类型介绍', 'banner3.jpg', '/pages/booking/booking', 3)
ON CONFLICT DO NOTHING;

-- 插入功能按钮数据
INSERT INTO action_buttons (name, icon, action_value, sort_order) VALUES 
('立即预约', 'calendar', 2, 1),
('今日一言', 'quote', 3, 2),
('数独游戏', 'game', 5, 3),
('积分商城', 'gift', 6, 4),
('通知公告', 'notice', 7, 5)
ON CONFLICT DO NOTHING;

-- 插入商城信息数据
INSERT INTO market_info (slogan, description) VALUES 
('积分兑换好礼，健康生活更精彩', '使用课程积分兑换精美礼品，让瑜伽练习更有动力！')
ON CONFLICT DO NOTHING;

-- 插入会员卡套餐示例数据
INSERT INTO membership_plans (name, description, card_type, validity_days, total_classes, price, original_price, applicable_lesson_types, max_bookings_per_day, benefits, restrictions, sort_order) VALUES 
-- 不限次卡
('年卡', '365天内无限次上课，适合长期练习的会员', 'unlimited', 365, NULL, 2680.00, 3200.00, NULL, 2, ARRAY['全年无限次课程', '优先预约权', '会员专享活动'], ARRAY['每日最多预约2节课', '需提前24小时取消'], 1),
('半年卡', '180天内无限次上课，体验瑜伽生活方式', 'unlimited', 180, NULL, 1580.00, 1800.00, NULL, 2, ARRAY['半年无限次课程', '优先预约权'], ARRAY['每日最多预约2节课', '需提前24小时取消'], 2),
('季度卡', '90天内无限次上课，短期集中训练', 'unlimited', 90, NULL, 880.00, 1000.00, NULL, 1, ARRAY['季度无限次课程'], ARRAY['每日最多预约1节课'], 3),

-- 次数卡 - 通用
('20次卡', '20次课程，有效期6个月，适合新手体验', 'count_based', 180, 20, 1500.00, 1600.00, NULL, 2, ARRAY['20次任意课程', '6个月有效期'], ARRAY['逾期作废', '不可转让'], 6),
('10次卡', '10次课程，有效期3个月，轻度练习', 'count_based', 90, 10, 800.00, 900.00, NULL, 1, ARRAY['10次任意课程', '3个月有效期'], ARRAY['逾期作废'], 7),
('5次卡', '5次课程，有效期1个月，体验课程', 'count_based', 30, 5, 450.00, 500.00, NULL, 1, ARRAY['5次任意课程', '1个月有效期'], ARRAY['逾期作废'], 8),

-- 次数卡 - 专项
('私教10次卡', '10次私教课程，专业一对一指导', 'count_based', 180, 10, 3500.00, 4000.00, ARRAY['private'], 1, ARRAY['专业私教指导', '个性化训练计划'], ARRAY['仅限私教课程', '需提前预约'], 4),
('小班课15次卡', '15次小班课程，精品小班教学', 'count_based', 120, 15, 1800.00, 2000.00, ARRAY['small_class'], 2, ARRAY['精品小班教学', '更多关注'], ARRAY['仅限小班课程'], 5)

ON CONFLICT DO NOTHING;

-- 插入管理员用户 (密码: admin123, 实际应用中应该使用bcrypt加密)
INSERT INTO admin_users (username, password_hash) VALUES 
('admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewKyUK0Gg5hGD5aS')
ON CONFLICT (username) DO NOTHING;