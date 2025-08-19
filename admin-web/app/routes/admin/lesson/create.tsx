import React, { useState, useEffect } from 'react';
import { Form, Input, Select, DatePicker, InputNumber, Button, Space, Card, Typography, Spin, message, Breadcrumb, Badge } from 'antd';
import { useNavigate, useSearchParams } from 'react-router';
import dayjs from 'dayjs';
import { ArrowLeftOutlined, HomeOutlined, PlusOutlined } from '@ant-design/icons';

const { Title } = Typography;
const { Option } = Select;
const { RangePicker } = DatePicker;

// Types for Teacher and Location
interface Teacher {
  id: number;
  name: string;
  description?: string;
  avatar_url?: string;
  bio?: string;
  certifications?: string[];
  specialties?: string[];
  experience_years: number;
  average_rating?: number;
  total_ratings: number;
  created_at: string;
  updated_at: string;
  is_active: boolean;
}

interface Location {
  id: number;
  name: string;
  description?: string;
  capacity: number;
  equipment?: string[];
  facilities?: string[];
  floor_number?: number;
  room_number?: string;
  is_accessible: boolean;
  booking_enabled: boolean;
  hourly_rate?: number;
  images?: string[];
  created_at: string;
  updated_at: string;
  is_active: boolean;
}

const CreateLessonPage: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const [form] = Form.useForm();

  // States
  const [loading, setLoading] = useState(false);
  const [teachers, setTeachers] = useState<Teacher[]>([]);
  const [locations, setLocations] = useState<Location[]>([]);
  const [teachersLoading, setTeachersLoading] = useState(false);
  const [locationsLoading, setLocationsLoading] = useState(false);

  // Get preselected date from URL params
  const preselectedDate = searchParams.get('date');

  // Fetch teachers for form selection
  const fetchTeachers = async () => {
    setTeachersLoading(true);
    try {
      const response = await fetch('/api/admin/teachers');
      if (!response.ok) {
        throw new Error('Failed to fetch teachers');
      }
      const data = await response.json();
      setTeachers(Array.isArray(data) ? data.filter((t: Teacher) => t.is_active) : []);
    } catch (error) {
      console.error('Error fetching teachers:', error);
      message.error('获取老师列表失败');
    } finally {
      setTeachersLoading(false);
    }
  };

  // Fetch locations for form selection
  const fetchLocations = async () => {
    setLocationsLoading(true);
    try {
      const response = await fetch('/api/admin/locations');
      if (!response.ok) {
        throw new Error('Failed to fetch locations');
      }
      const data = await response.json();
      setLocations(Array.isArray(data) ? data.filter((l: Location) => l.is_active && l.booking_enabled) : []);
    } catch (error) {
      console.error('Error fetching locations:', error);
      message.error('获取教室列表失败');
    } finally {
      setLocationsLoading(false);
    }
  };

  // Handle create lesson
  const handleCreateLesson = async (values: any) => {
    setLoading(true);
    try {
      // Find selected teacher and location objects
      const selectedTeacher = teachers.find(t => t.id === values.teacher_id);
      const selectedLocation = locations.find(l => l.id === values.location_id);

      if (!selectedTeacher || !selectedLocation) {
        message.error('请选择有效的老师和教室');
        return;
      }

      const lessonData = {
        title: values.title,
        description: values.description,
        teacher: selectedTeacher,
        location: selectedLocation,
        lesson_type: values.lesson_type,
        difficulty_level: values.difficulty_level,
        start_time: values.time_range[0].toISOString(),
        end_time: values.time_range[1].toISOString(),
        max_students: values.max_students,
        current_students: 0,
        price: values.price,
        equipment_required: values.equipment_required,
        prerequisites: values.prerequisites,
        cancellation_policy: values.cancellation_policy,
        notes: values.notes,
        is_active: true,
        id: 0, // Will be set by backend
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      const response = await fetch('/api/admin/lesson', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(lessonData),
      });

      if (!response.ok) {
        throw new Error('Failed to create lesson');
      }

      const result = await response.json();
      message.success('课程创建成功！');

      // Navigate back to the lessons list page with the date
      const date = values.time_range[0].format('YYYY-MM-DD');
      navigate(`/admin/lessons-list?date=${date}`);

    } catch (error) {
      console.error('Error creating lesson:', error);
      message.error('创建课程失败，请重试');
    } finally {
      setLoading(false);
    }
  };

  // Handle back navigation
  const handleBack = () => {
    if (preselectedDate) {
      navigate(`/admin/lessons-list?date=${preselectedDate}`);
    } else {
      navigate('/admin/lessions');
    }
  };

  useEffect(() => {
    // Fetch teachers and locations on mount
    fetchTeachers();
    fetchLocations();

    // Set default form values if date is provided
    if (preselectedDate) {
      const date = dayjs(preselectedDate);
      form.setFieldsValue({
        time_range: [
          date.hour(9).minute(0),
          date.hour(10).minute(0),
        ],
      });
    }
  }, [preselectedDate]);

  return (
    <div style={{ padding: '24px', maxWidth: '800px', margin: '0 auto' }}>
      {/* Breadcrumb */}
      <Breadcrumb style={{ marginBottom: '16px' }}>
        <Breadcrumb.Item>
          <HomeOutlined />
          <span style={{ marginLeft: '4px' }}>管理后台</span>
        </Breadcrumb.Item>
        <Breadcrumb.Item>
          <span onClick={() => navigate('/admin/lessions')} style={{ cursor: 'pointer' }}>
            课程管理
          </span>
        </Breadcrumb.Item>
        <Breadcrumb.Item>创建课程</Breadcrumb.Item>
      </Breadcrumb>

      {/* Header */}
      <div style={{ marginBottom: '24px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center' }}>
          <Button
            icon={<ArrowLeftOutlined />}
            onClick={handleBack}
            style={{ marginRight: '16px' }}
          >
            返回
          </Button>
          <Title level={2} style={{ margin: 0 }}>
            <PlusOutlined style={{ marginRight: '8px' }} />
            创建新课程
          </Title>
        </div>
      </div>

      {/* Form Card */}

      <Card>
        <Spin spinning={teachersLoading || locationsLoading}>
          <Form
            form={form}
            layout="vertical"
            onFinish={handleCreateLesson}
            autoComplete="off"
            initialValues={{
              lesson_type: 'team',
              difficulty_level: 'all_levels',
              max_students: 10,
              is_active: true,
            }}
          >
            <Form.Item shouldUpdate>
              {() => {
                return <pre>{JSON.stringify(form.getFieldsValue(), null, 2)}</pre>;
              }}
            </Form.Item>
            <Form.Item
              name="title"
              label="课程名称"
              rules={[{ required: true, message: '请输入课程名称' }]}
            >
              <Input size="large" placeholder="请输入课程名称" />
            </Form.Item>

            <Form.Item
              name="description"
              label="课程描述"
            >
              <Input.TextArea rows={3} placeholder="请输入课程描述" />
            </Form.Item>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <Form.Item
                name="teacher_id"
                label="授课老师"
                rules={[{ required: true, message: '请选择授课老师' }]}
              >
                <Select
                  size="large"
                  placeholder="请选择授课老师"
                  showSearch
                  loading={teachersLoading}
                  filterOption={(input, option) =>
                    option?.children?.toString().toLowerCase().includes(input.toLowerCase())
                  }
                >
                  {teachers.map(teacher => (
                    <Option key={teacher.id} value={teacher.id}>
                      {teacher.name}
                      {teacher.specialties && teacher.specialties.length > 0 && (
                        <span style={{ color: '#999', marginLeft: '8px' }}>
                          ({teacher.specialties.join(', ')})
                        </span>
                      )}
                    </Option>
                  ))}
                </Select>
              </Form.Item>

              <Form.Item
                name="location_id"
                label="上课地点"
                rules={[{ required: true, message: '请选择上课地点' }]}
              >
                <Select
                  size="large"
                  placeholder="请选择上课地点"
                  showSearch
                  loading={locationsLoading}
                  filterOption={(input, option) =>
                    option?.children?.toString().toLowerCase().includes(input.toLowerCase())
                  }
                >
                  {locations.map(location => (
                    <Option key={location.id} value={location.id}>
                      {location.name}
                      <span style={{ color: '#999', marginLeft: '8px' }}>
                        (容量: {location.capacity}人)
                      </span>
                    </Option>
                  ))}
                </Select>
              </Form.Item>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <Form.Item
                name="lesson_type"
                label="课程类型"
                rules={[{ required: true, message: '请选择课程类型' }]}
              >
                <Select size="large" placeholder="请选择课程类型">
                  <Option value="team">团体课</Option>
                  <Option value="small_class">小班课</Option>
                  <Option value="private">私教课</Option>
                  <Option value="equipment_small_class">器械小班课</Option>
                  <Option value="workshop">工作坊</Option>
                </Select>
              </Form.Item>

              <Form.Item
                name="difficulty_level"
                label="难度等级"
                rules={[{ required: true, message: '请选择难度等级' }]}
              >
                <Select size="large" placeholder="请选择难度等级">
                  <Option value="beginner">
                    <Badge color="green" text="初级" />
                  </Option>
                  <Option value="intermediate">
                    <Badge color="yellow" text="中级" />
                  </Option>
                  <Option value="advanced">
                    <Badge color="red" text="高级" />
                  </Option>
                  <Option value="all_levels">适合所有等级</Option>
                </Select>
              </Form.Item>
            </div>

            <Form.Item
              name="time_range"
              label="上课时间"
              rules={[{ required: true, message: '请选择上课时间' }]}
            >
              <RangePicker
                size="large"
                showTime={{ format: 'HH:mm' }}
                format="YYYY-MM-DD HH:mm"
                placeholder={['开始时间', '结束时间']}
                style={{ width: '100%' }}
              />
            </Form.Item>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <Form.Item
                name="max_students"
                label="最大学员数"
                rules={[{ required: true, message: '请输入最大学员数' }]}
              >
                <InputNumber
                  size="large"
                  min={1}
                  max={100}
                  placeholder="请输入最大学员数"
                  style={{ width: '100%' }}
                />
              </Form.Item>

              <Form.Item
                name="price"
                label="课程价格（元）"
              >
                <InputNumber
                  size="large"
                  min={0}
                  precision={2}
                  placeholder="请输入课程价格"
                  style={{ width: '100%' }}
                />
              </Form.Item>
            </div>

            <Form.Item
              name="equipment_required"
              label="所需设备"
            >
              <Select
                mode="tags"
                placeholder="请输入所需设备，按回车添加"
                style={{ width: '100%' }}
              />
            </Form.Item>

            <Form.Item
              name="prerequisites"
              label="先决条件"
            >
              <Input.TextArea rows={2} placeholder="请输入课程先决条件" />
            </Form.Item>

            <Form.Item
              name="cancellation_policy"
              label="取消政策"
            >
              <Input.TextArea rows={2} placeholder="请输入取消政策" />
            </Form.Item>

            <Form.Item
              name="notes"
              label="备注"
            >
              <Input.TextArea rows={2} placeholder="请输入备注信息" />
            </Form.Item>

            <Form.Item style={{ marginBottom: 0, marginTop: '32px' }}>
              <Space size="middle">
                <Button
                  type="primary"
                  htmlType="submit"
                  loading={loading}
                  size="large"
                  style={{ minWidth: '120px' }}
                >
                  创建课程
                </Button>
                <Button
                  size="large"
                  onClick={handleBack}
                  style={{ minWidth: '120px' }}
                >
                  取消
                </Button>
              </Space>
            </Form.Item>
          </Form>
        </Spin>
      </Card>
    </div>
  );
};

export default CreateLessonPage;