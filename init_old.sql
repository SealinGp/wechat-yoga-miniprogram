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

-- 创建课程表 (增强版)
CREATE TABLE IF NOT EXISTS lessons (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    teacher_id INTEGER REFERENCES teachers(id),
    lesson_type lesson_type NOT NULL DEFAULT 'team',
    difficulty_level difficulty_level NOT NULL DEFAULT 'all_levels',
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    max_students INTEGER NOT NULL,
    current_students INTEGER DEFAULT 0,
    venue VARCHAR(255),
    price DECIMAL(10,2) DEFAULT 0.00, -- 课程价格
    equipment_required TEXT[], -- 所需器材
    prerequisites TEXT, -- 先决条件
    cancellation_policy TEXT, -- 取消政策
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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 创建轮播图表
CREATE TABLE IF NOT EXISTS banners (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    image_url TEXT NOT NULL,
    link_url TEXT,
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
    link_type VARCHAR(20) DEFAULT 'page', -- page, url, miniprogram
    link_target TEXT NOT NULL, -- 跳转目标
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
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
    applicable_lesson_types lesson_type[], -- 适用的课程类型数组，NULL表示全部适用
    max_bookings_per_day INTEGER DEFAULT 1, -- 每天最多可约课数
    transfer_allowed BOOLEAN DEFAULT FALSE, -- 是否允许转让
    refund_allowed BOOLEAN DEFAULT FALSE, -- 是否允许退款
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
    original_total_classes INTEGER, -- 原始总次数 (仅次数卡)
    remaining_classes INTEGER, -- 剩余次数 (仅次数卡)
    
    -- 时间信息
    purchased_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 购买时间
    activated_at TIMESTAMP WITH TIME ZONE, -- 激活时间 (可能延后激活)
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
        (card_type = 'count_based' AND remaining_classes >= 0 AND remaining_classes <= original_total_classes)
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
    usage_type VARCHAR(20) NOT NULL DEFAULT 'booking', -- booking, cancellation, refund
    classes_consumed INTEGER DEFAULT 1, -- 消耗的次数，取消时为负数
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

CREATE INDEX IF NOT EXISTS idx_teachers_active ON teachers(is_active);
CREATE INDEX IF NOT EXISTS idx_teachers_rating ON teachers(average_rating DESC);

CREATE INDEX IF NOT EXISTS idx_lessons_start_time ON lessons(start_time);
CREATE INDEX IF NOT EXISTS idx_lessons_teacher_id ON lessons(teacher_id);
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

-- 创建会员卡号生成函数
CREATE OR REPLACE FUNCTION generate_card_number()
RETURNS TEXT AS $$
BEGIN
    RETURN 'MC' || TO_CHAR(CURRENT_TIMESTAMP, 'YYYYMMDD') || LPAD(NEXTVAL('user_membership_cards_id_seq')::TEXT, 6, '0');
END;
$$ LANGUAGE plpgsql;

-- 创建触发器函数：自动生成卡号和设置过期时间
CREATE OR REPLACE FUNCTION set_membership_card_defaults()
RETURNS TRIGGER AS $$
DECLARE
    plan_record membership_plans%ROWTYPE;
BEGIN
    -- 获取套餐信息
    SELECT * INTO plan_record FROM membership_plans WHERE id = NEW.plan_id;
    
    -- 如果没有设置卡号，自动生成
    IF NEW.card_number IS NULL OR NEW.card_number = '' THEN
        NEW.card_number := generate_card_number();
    END IF;
    
    -- 从套餐复制基本信息
    NEW.card_type := plan_record.card_type;
    NEW.applicable_lesson_types := plan_record.applicable_lesson_types;
    NEW.max_bookings_per_day := plan_record.max_bookings_per_day;
    NEW.transfer_allowed := plan_record.transfer_allowed;
    NEW.refund_allowed := plan_record.refund_allowed;
    
    -- 设置次数信息 (仅次数卡)
    IF plan_record.card_type = 'count_based' THEN
        NEW.original_total_classes := plan_record.total_classes;
        NEW.remaining_classes := plan_record.total_classes;
    END IF;
    
    -- 设置激活时间 (如果未设置)
    IF NEW.activated_at IS NULL THEN
        NEW.activated_at := NEW.purchased_at;
    END IF;
    
    -- 计算过期时间
    NEW.expires_at := NEW.activated_at + (plan_record.validity_days || ' days')::INTERVAL;
    
    -- 计算实际支付金额
    NEW.actual_paid := NEW.purchase_price - COALESCE(NEW.discount_amount, 0);
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器：会员卡创建时自动设置默认值
CREATE OR REPLACE TRIGGER trigger_set_membership_card_defaults
    BEFORE INSERT ON user_membership_cards
    FOR EACH ROW
    EXECUTE FUNCTION set_membership_card_defaults();

-- 创建触发器函数：自动更新会员卡状态
CREATE OR REPLACE FUNCTION update_membership_card_status()
RETURNS TRIGGER AS $$
BEGIN
    -- 检查是否过期
    IF NEW.expires_at <= CURRENT_TIMESTAMP AND NEW.status = 'active' THEN
        NEW.status := 'expired';
    END IF;
    
    -- 检查次数卡是否用完
    IF NEW.card_type = 'count_based' AND NEW.remaining_classes = 0 AND NEW.status = 'active' THEN
        NEW.status := 'used_up';
    END IF;
    
    NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器：会员卡更新时自动检查状态
CREATE OR REPLACE TRIGGER trigger_update_membership_card_status
    BEFORE UPDATE ON user_membership_cards
    FOR EACH ROW
    EXECUTE FUNCTION update_membership_card_status();

-- 创建触发器函数：记录会员卡使用
CREATE OR REPLACE FUNCTION record_membership_card_usage()
RETURNS TRIGGER AS $$
DECLARE
    user_card_record user_membership_cards%ROWTYPE;
    classes_to_consume INTEGER := 1;
BEGIN
    -- 获取用户的有效会员卡 (优先使用次数卡，再使用不限次卡)
    SELECT * INTO user_card_record
    FROM user_membership_cards 
    WHERE user_id = NEW.user_id 
    AND status = 'active'
    AND expires_at > CURRENT_TIMESTAMP
    AND (
        applicable_lesson_types IS NULL OR 
        (SELECT lesson_type FROM lessons WHERE id = NEW.lesson_id) = ANY(applicable_lesson_types)
    )
    ORDER BY 
        CASE WHEN card_type = 'count_based' THEN 0 ELSE 1 END, -- 优先次数卡
        expires_at ASC -- 优先即将过期的
    LIMIT 1;
    
    -- 如果找到了有效的会员卡
    IF FOUND THEN
        -- 记录使用记录
        INSERT INTO membership_card_usage (
            user_card_id, booking_id, lesson_id, user_id, usage_type,
            classes_consumed, remaining_classes_before, remaining_classes_after
        ) VALUES (
            user_card_record.id, NEW.id, NEW.lesson_id, NEW.user_id, 'booking',
            classes_to_consume, user_card_record.remaining_classes,
            CASE WHEN user_card_record.card_type = 'count_based' 
                 THEN user_card_record.remaining_classes - classes_to_consume 
                 ELSE user_card_record.remaining_classes END
        );
        
        -- 如果是次数卡，扣除次数
        IF user_card_record.card_type = 'count_based' THEN
            UPDATE user_membership_cards 
            SET remaining_classes = remaining_classes - classes_to_consume,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = user_card_record.id;
        END IF;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器：预约确认时自动使用会员卡
CREATE OR REPLACE TRIGGER trigger_use_membership_card
    AFTER INSERT ON bookings
    FOR EACH ROW
    WHEN (NEW.status = 'confirmed')
    EXECUTE FUNCTION record_membership_card_usage();

-- 创建触发器函数：自动更新教师平均评分
CREATE OR REPLACE FUNCTION update_teacher_average_rating()
RETURNS TRIGGER AS $$
BEGIN
    -- 重新计算教师的平均评分和总评分次数
    UPDATE teachers SET 
        average_rating = (
            SELECT COALESCE(AVG(rating), 0.0)
            FROM teacher_ratings 
            WHERE teacher_id = COALESCE(NEW.teacher_id, OLD.teacher_id)
        ),
        total_ratings = (
            SELECT COUNT(*)
            FROM teacher_ratings 
            WHERE teacher_id = COALESCE(NEW.teacher_id, OLD.teacher_id)
        ),
        updated_at = CURRENT_TIMESTAMP
    WHERE id = COALESCE(NEW.teacher_id, OLD.teacher_id);
    
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- 创建触发器：评分变更时自动更新教师平均分
CREATE OR REPLACE TRIGGER trigger_update_teacher_rating
    AFTER INSERT OR UPDATE OR DELETE ON teacher_ratings
    FOR EACH ROW
    EXECUTE FUNCTION update_teacher_average_rating();

-- 创建触发器函数：自动更新课程的当前学员数
CREATE OR REPLACE FUNCTION update_lesson_current_students()
RETURNS TRIGGER AS $$
BEGIN
    -- 重新计算课程的当前学员数
    UPDATE lessons SET 
        current_students = (
            SELECT COUNT(*)
            FROM bookings 
            WHERE lesson_id = COALESCE(NEW.lesson_id, OLD.lesson_id)
            AND status = 'confirmed'
        ),
        updated_at = CURRENT_TIMESTAMP
    WHERE id = COALESCE(NEW.lesson_id, OLD.lesson_id);
    
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- 创建触发器：预约状态变更时自动更新课程学员数
CREATE OR REPLACE TRIGGER trigger_update_lesson_students
    AFTER INSERT OR UPDATE OR DELETE ON bookings
    FOR EACH ROW
    EXECUTE FUNCTION update_lesson_current_students();

-- 插入评分标准示例数据
INSERT INTO rating_criteria (name, description, weight) VALUES 
('教学能力', '老师的专业教学技能和指导能力', 1.5),
('沟通技巧', '与学员的互动和沟通效果', 1.2),
('专业素养', '职业态度和专业知识', 1.3),
('课程设计', '课程安排和内容设计的合理性', 1.0),
('个人魅力', '老师的个人魅力和感染力', 0.8)
ON CONFLICT (name) DO NOTHING;

-- 插入功能按钮示例数据
INSERT INTO action_buttons (id, name, icon, link_target, sort_order) VALUES 
(2, '约课', 'book', '/pages/booking/booking', 1),
(3, '私教', 'one', '/pages/one/one', 2),
(5, '数独', 'game', '/pages/sudoku/sudoku', 3),
(6, '商城', 'market', '/pages/market/market', 4),
(7, '公告', 'notice', '/pages/notices/notices', 5)
ON CONFLICT (id) DO NOTHING;

-- 插入示例教师 (增强版)
INSERT INTO teachers (name, description, avatar_url, bio, certifications, specialties, experience_years, average_rating, total_ratings) VALUES 
('张老师', '资深瑜伽教练，专业哈他瑜伽', 'https://example.com/avatar1.jpg', '拥有10年瑜伽教学经验，擅长哈他瑜伽和阴瑜伽', ARRAY['RYT-500认证', '阴瑜伽导师认证'], ARRAY['哈他瑜伽', '阴瑜伽', '冥想'], 10, 4.5, 25),
('李老师', '流瑜伽专家，温和亲切', 'https://example.com/avatar2.jpg', '专注于流瑜伽和vinyasa练习，教学风格温和耐心', ARRAY['RYT-200认证', 'Vinyasa专业认证'], ARRAY['流瑜伽', 'Vinyasa', '体式调整'], 6, 4.3, 18),
('王老师', '阴瑜伽导师，注重身心平衡', 'https://example.com/avatar3.jpg', '心理学背景，结合瑜伽和正念练习', ARRAY['心理咨询师证书', '阴瑜伽认证'], ARRAY['阴瑜伽', '正念练习', '冥想指导'], 8, 4.7, 32)
ON CONFLICT DO NOTHING;

-- 插入示例课程
INSERT INTO lessons (title, description, teacher_id, lesson_type, difficulty_level, start_time, end_time, max_students, venue, price) VALUES 
('哈他瑜伽基础班', '适合初学者的哈他瑜伽课程', 1, 'team', 'beginner', CURRENT_TIMESTAMP + INTERVAL '1 day', CURRENT_TIMESTAMP + INTERVAL '1 day 1 hour 30 minutes', 15, 'A教室', 68.00),
('流瑜伽进阶', '动态流瑜伽练习，提升力量和柔韧性', 2, 'small_class', 'intermediate', CURRENT_TIMESTAMP + INTERVAL '2 days', CURRENT_TIMESTAMP + INTERVAL '2 days 1 hour', 8, 'B教室', 88.00),
('阴瑜伽与冥想', '深度放松的阴瑜伽结合正念冥想', 3, 'small_class', 'all_levels', CURRENT_TIMESTAMP + INTERVAL '3 days', CURRENT_TIMESTAMP + INTERVAL '3 days 1 hour 15 minutes', 10, 'C教室', 78.00)
ON CONFLICT DO NOTHING;

-- 插入示例通知
INSERT INTO notices (title, content, author) VALUES 
('欢迎使用瑜伽约课系统', '感谢您使用我们的瑜伽约课小程序，请注意课程时间安排。', '管理员'),
('新课程上线通知', '本月新增多种瑜伽课程类型，欢迎体验。', '管理员'),
('教师评分功能上线', '现在可以对上过的课程进行评分和评价，帮助其他学员选择合适的老师。', '管理员')
ON CONFLICT DO NOTHING;

-- 插入示例轮播图
INSERT INTO banners (title, image_url, link_url, sort_order) VALUES 
('新学员专享优惠', 'banner1.jpg', '/pages/market/market', 1),
('明星教师介绍', 'banner2.jpg', '/pages/teacher/teacher?id=1', 2),
('课程类型介绍', 'banner3.jpg', '/pages/booking/booking', 3)
ON CONFLICT DO NOTHING;

-- 插入会员卡套餐示例数据
INSERT INTO membership_plans (name, description, card_type, validity_days, total_classes, price, applicable_lesson_types, max_bookings_per_day, sort_order) VALUES 
-- 不限次卡
('年卡', '365天内无限次上课，适合长期练习的会员', 'unlimited', 365, NULL, 2680.00, NULL, 2, 1),
('半年卡', '180天内无限次上课，体验瑜伽生活方式', 'unlimited', 180, NULL, 1580.00, NULL, 2, 2),
('季度卡', '90天内无限次上课，短期集中训练', 'unlimited', 90, NULL, 880.00, NULL, 1, 3),

-- 次数卡 - 通用
('50次卡', '50次课程，有效期12个月，适合所有课程类型', 'count_based', 365, 50, 3200.00, NULL, 3, 4),
('30次卡', '30次课程，有效期8个月，性价比之选', 'count_based', 240, 30, 2100.00, NULL, 2, 5),
('20次卡', '20次课程，有效期6个月，适合新手体验', 'count_based', 180, 20, 1500.00, NULL, 2, 6),
('10次卡', '10次课程，有效期3个月，短期体验', 'count_based', 90, 10, 800.00, NULL, 1, 7),

-- 次数卡 - 团课专享
('30次团课卡', '30次团课专享，有效期8个月', 'count_based', 240, 30, 1800.00, ARRAY['team'::lesson_type], 2, 8),
('20次团课卡', '20次团课专享，有效期6个月', 'count_based', 180, 20, 1200.00, ARRAY['team'::lesson_type], 2, 9),

-- 次数卡 - 私教专享  
('10次私教卡', '10次私教课程，有效期6个月', 'count_based', 180, 10, 2800.00, ARRAY['private'::lesson_type], 1, 10),
('5次私教卡', '5次私教课程，有效期3个月', 'count_based', 90, 5, 1500.00, ARRAY['private'::lesson_type], 1, 11),

-- 次数卡 - 小班专享
('15次小班卡', '15次小班课程，有效期6个月', 'count_based', 180, 15, 1350.00, ARRAY['small_class'::lesson_type], 1, 12)
ON CONFLICT DO NOTHING;

-- 创建存储过程

-- fn_index: 返回首页数据 (使用新表结构)
CREATE OR REPLACE FUNCTION fn_index()
RETURNS json
LANGUAGE sql
AS $$
SELECT jsonb_build_object(
    'poster', (
        SELECT COALESCE(json_agg(jsonb_build_object(
            'id', id,
            'title', title,
            'image', image_url,
            'link_url', link_url
        ) ORDER BY sort_order), '[]'::json)
        FROM banners 
        WHERE is_active = true
        AND (start_date IS NULL OR start_date <= CURRENT_TIMESTAMP)
        AND (end_date IS NULL OR end_date >= CURRENT_TIMESTAMP)
    ),
    'booked', (
        SELECT COALESCE(json_agg(jsonb_build_object(
            'id', b.id,
            'nick_name', u.nick_name,
            'avatar_url', u.avatar_url,
            'lesson_title', l.title,
            'booking_time', extract(epoch from b.booking_time)::bigint
        ) ORDER BY b.booking_time DESC), '[]'::json)
        FROM bookings b
        JOIN users u ON u.id = b.user_id
        JOIN lessons l ON l.id = b.lesson_id
        WHERE b.status = 'confirmed'
        LIMIT 10
    ),
    'actions', (
        SELECT COALESCE(json_agg(jsonb_build_object(
            'id', id,
            'name', name,
            'icon', icon,
            'link_type', link_type,
            'link_target', link_target
        ) ORDER BY sort_order), '[]'::json)
        FROM action_buttons 
        WHERE is_active = true
    ),
    'teachers', (
        SELECT COALESCE(json_agg(jsonb_build_object(
            'id', id,
            'name', name,
            'thumbnail', avatar_url,
            'introduction', description,
            'bio', bio,
            'specialties', specialties,
            'experience_years', experience_years,
            'average_rating', average_rating,
            'total_ratings', total_ratings
        ) ORDER BY average_rating DESC, total_ratings DESC), '[]'::json)
        FROM teachers
        WHERE is_active = true
    ),
    'market', jsonb_build_object('id', 1, 'slogan', 'LC PILATES 空中普拉提 - 您的健康生活伙伴'),
    'notices', (
        SELECT COALESCE(json_agg(jsonb_build_object(
            'id', id,
            'title', title,
            'content', LEFT(content, 100) || CASE WHEN LENGTH(content) > 100 THEN '...' ELSE '' END,
            'author', author,
            'priority', priority,
            'updated_time', extract(epoch from created_at)::bigint
        ) ORDER BY priority DESC, created_at DESC), '[]'::json)
        FROM notices
        WHERE is_active = true
        LIMIT 5
    )
);
$$;

-- fn_user_query: 查询用户信息
CREATE OR REPLACE FUNCTION fn_user_query(in_id text)
RETURNS json
LANGUAGE sql
AS $$
SELECT row_to_json(t)
FROM (
    SELECT id, avatar_url, nick_name, 
           CASE WHEN is_admin THEN 1 ELSE 0 END as user_type
    FROM users
    WHERE open_id = in_id
) as t;
$$;

-- fn_user_update: 更新用户信息
CREATE OR REPLACE FUNCTION fn_user_update(obj json)
RETURNS integer
LANGUAGE plpgsql
AS $$
DECLARE
    result_id integer := 0;
BEGIN
    INSERT INTO users (open_id, nick_name, avatar_url, phone, created_at, updated_at)
    VALUES (
        obj ->> 'open_id',
        COALESCE(obj ->> 'nick_name', ''),
        obj ->> 'avatar_url',
        obj ->> 'phone',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
    )
    ON CONFLICT (open_id) DO UPDATE SET
        nick_name = COALESCE(EXCLUDED.nick_name, users.nick_name),
        avatar_url = COALESCE(EXCLUDED.avatar_url, users.avatar_url),
        phone = COALESCE(EXCLUDED.phone, users.phone),
        updated_at = CURRENT_TIMESTAMP
    RETURNING id INTO result_id;
    
    RETURN result_id;
END;
$$;

-- fn_user_book_statistics: 用户预约统计
CREATE OR REPLACE FUNCTION fn_user_book_statistics(in_id text)
RETURNS json
LANGUAGE sql
AS $$
SELECT row_to_json(t)
FROM (
    SELECT u.id, u.avatar_url, u.nick_name,
           CASE WHEN u.is_admin THEN 1 ELSE 0 END as user_type,
           COUNT(b.id) FILTER (WHERE b.status = 'confirmed') as total_bookings
    FROM users u
    LEFT JOIN bookings b ON u.id = b.user_id
    WHERE u.open_id = in_id
    GROUP BY u.id, u.avatar_url, u.nick_name, u.is_admin
) as t;
$$;

-- fn_debug: 调试日志记录
CREATE OR REPLACE FUNCTION fn_debug(obj json, in_ip text)
RETURNS integer
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO debug_logs (
        open_id, brand, model, pixel_ratio, screen_height, screen_width,
        version, sdk_version, platform, created_at
    )
    VALUES (
        obj ->> 'open_id',
        obj ->> 'brand',
        obj ->> 'model',
        (obj ->> 'pixel_ratio')::decimal,
        (obj ->> 'screen_height')::integer,
        (obj ->> 'screen_width')::integer,
        obj ->> 'version',
        obj ->> 'sdk_version',
        obj ->> 'platform',
        CURRENT_TIMESTAMP
    );
    RETURN 1;
END;
$$;