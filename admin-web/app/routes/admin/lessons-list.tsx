import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { ProList } from '@ant-design/pro-components';
import { Typography, Tag, Avatar, Button, Modal, Card, Spin, message, Space } from 'antd';
import { useSearchParams, useNavigate } from 'react-router';
import dayjs from 'dayjs';
import { CalendarOutlined, UserOutlined, EnvironmentOutlined, ClockCircleOutlined, PlusOutlined } from '@ant-design/icons';
import { getLessonTypeStr, getDifficultyLevelStr, type Lesson } from '~/utils/lesson';

const { Title, Text } = Typography;

const LessonsListPage: React.FC = () => {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const [lessons, setLessons] = useState<Lesson[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedLesson, setSelectedLesson] = useState<Lesson | null>(null);
  const [modalVisible, setModalVisible] = useState(false);
  const [modalLoading, setModalLoading] = useState(false);

  // Get date from URL params - memoized to prevent infinite loops
  const dateParam = searchParams.get('date');
  const selectedDate = useMemo(() => {
    return dateParam ? dayjs(dateParam) : dayjs();
  }, [dateParam]);

  // Fetch lessons data - memoized to prevent unnecessary re-creation
  const fetchLessons = useCallback(async () => {
    setLoading(true);
    try {
      // Create date range for the selected date (00:00:00 to 23:59:59)
      const startOfDay = selectedDate.startOf('day');
      const endOfDay = selectedDate.endOf('day');

      // Convert to Unix timestamps for the API
      const start = startOfDay.unix();
      const end = endOfDay.unix();

      const response = await fetch(`/api/admin/lessons?start=${start}&end=${end}&limit=100&offset=0`);
      if (!response.ok) {
        throw new Error('Failed to fetch lessons');
      }
      const data = await response.json();

      // No need for client-side filtering since backend handles date range
      setLessons(Array.isArray(data) ? data : []);
    } catch (error) {
      console.error('Error fetching lessons:', error);
      message.error('Failed to load lessons');
      setLessons([]);
    } finally {
      setLoading(false);
    }
  }, [selectedDate]);

  // Fetch specific lesson details
  const fetchLessonDetail = async (lessonId: number) => {
    setModalLoading(true);
    try {
      const response = await fetch(`/api/admin/lesson?id=${lessonId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch lesson details');
      }
      const lessonDetail = await response.json();
      setSelectedLesson(lessonDetail);
    } catch (error) {
      console.error('Error fetching lesson details:', error);
      message.error('Failed to load lesson details');
    } finally {
      setModalLoading(false);
    }
  };

  useEffect(() => {
    fetchLessons();
  }, [fetchLessons]);

  // Handle lesson click
  const handleLessonClick = async (lesson: Lesson) => {
    setModalVisible(true);
    await fetchLessonDetail(lesson.id);
  };

  // Handle create lesson
  const handleCreateLesson = () => {
    // Navigate to the dedicated create lesson page
    navigate(`/admin/lesson/create?date=${selectedDate.format('YYYY-MM-DD')}`);
  };

  // Get lesson type color
  const getLessonTypeColor = (type: string) => {
    const colors: Record<string, string> = {
      'team': 'blue',
      'small_class': 'green',
      'private': 'purple',
      'equipment_small_class': 'orange',
      'workshop': 'red',
    };
    return colors[type] || 'default';
  };

  // Get difficulty level color
  const getDifficultyColor = (level: string) => {
    const colors: Record<string, string> = {
      'beginner': 'green',
      'intermediate': 'orange',
      'advanced': 'red',
      'all_levels': 'blue',
    };
    return colors[level] || 'default';
  };

  return (
    <div style={{ padding: '24px' }}>
      <div style={{ marginBottom: '24px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <Title level={2}>
          <CalendarOutlined style={{ marginRight: '8px' }} />
          {selectedDate.format('YYYY年MM月DD日')} 课程列表
        </Title>
        <Space>
          <Button 
            type="primary" 
            icon={<PlusOutlined />} 
            onClick={handleCreateLesson}
          >
            创建课程
          </Button>
          <Button onClick={() => navigate('/admin/lessions')}>
            返回日历
          </Button>
        </Space>
      </div>

      <ProList<Lesson>
        loading={loading}
        dataSource={lessons}
        className="custom-lessons-list"
        metas={{
          title: {
            dataIndex: 'title',
            title: '课程名称',
          },
          avatar: {
            render: () => <Avatar icon={<CalendarOutlined />} />,
          },
          description: {
            render: (_, lesson) => (
              <div>
                <div style={{ marginBottom: '8px' }}>
                  <Tag color={getLessonTypeColor(lesson.lesson_type)}>
                    {getLessonTypeStr(lesson.lesson_type)}
                  </Tag>
                  <Tag color={getDifficultyColor(lesson.difficulty_level)}>
                    {getDifficultyLevelStr(lesson.difficulty_level)}
                  </Tag>
                  {!lesson.is_active && (
                    <Tag color="red">未激活</Tag>
                  )}
                </div>
                <div style={{ color: '#666', fontSize: '14px' }}>
                  <ClockCircleOutlined style={{ marginRight: '4px' }} />
                  {dayjs(lesson.start_time).format('HH:mm')} - {dayjs(lesson.end_time).format('HH:mm')}
                  {lesson.teacher && (
                    <>
                      <UserOutlined style={{ marginLeft: '16px', marginRight: '4px' }} />
                      {lesson.teacher.name}
                    </>
                  )}
                  {lesson.location && (
                    <>
                      <EnvironmentOutlined style={{ marginLeft: '16px', marginRight: '4px' }} />
                      {lesson.location.name}
                    </>
                  )}
                </div>
              </div>
            ),
          },
          content: {
            render: (_, lesson) => (
              <div style={{ fontSize: '14px', color: '#666' }}>
                {lesson.description || '暂无课程描述'}
              </div>
            ),
          },
          actions: {
            render: (_, lesson) => [
              <Button
                key="view"
                type="link"
                onClick={() => handleLessonClick(lesson)}
              >
                查看详情
              </Button>,
            ],
          },
        }}
        rowKey="id"
        pagination={{
          pageSize: 10,
          showSizeChanger: true,
          showQuickJumper: true,
          showTotal: (total, range) => `第 ${range[0]}-${range[1]} 条/总共 ${total} 条`,
        }}
        locale={{
          emptyText: '当天暂无课程安排',
        }}
        onRow={(lesson) => ({
          onClick: () => handleLessonClick(lesson),
          style: { cursor: 'pointer' },
        })}
      />

      {/* Lesson Detail Modal - Same as lessions.tsx */}
      <Modal
        title="课程详情"
        open={modalVisible}
        onCancel={() => {
          setModalVisible(false);
          setSelectedLesson(null);
        }}
        footer={null}
        width={800}
      >
        <Spin spinning={modalLoading}>
          {selectedLesson && (
            <div>
              <Card>
                <Title level={3}>{selectedLesson.title}</Title>

                <div style={{ marginBottom: '16px' }}>
                  <Tag color={getLessonTypeColor(selectedLesson.lesson_type)}>
                    {getLessonTypeStr(selectedLesson.lesson_type)}
                  </Tag>
                  <Tag color={getDifficultyColor(selectedLesson.difficulty_level)}>
                    {getDifficultyLevelStr(selectedLesson.difficulty_level)}
                  </Tag>
                  {!selectedLesson.is_active && (
                    <Tag color="red">未激活</Tag>
                  )}
                </div>

                {selectedLesson.description && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>课程描述：</Text>
                    <div>{selectedLesson.description}</div>
                  </div>
                )}

                <div style={{ marginBottom: '16px' }}>
                  <Text strong>时间：</Text>
                  <div>
                    {dayjs(selectedLesson.start_time).format('YYYY-MM-DD HH:mm')} - {dayjs(selectedLesson.end_time).format('HH:mm')}
                  </div>
                </div>

                {selectedLesson.teacher && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>授课老师：</Text>
                    <div>
                      <Text>{selectedLesson.teacher.name}</Text>
                      {selectedLesson.teacher.experience_years > 0 && (
                        <Text type="secondary"> ({selectedLesson.teacher.experience_years}年经验)</Text>
                      )}
                    </div>
                    {selectedLesson.teacher.specialties && selectedLesson.teacher.specialties.length > 0 && (
                      <div style={{ marginTop: '4px' }}>
                        <Text type="secondary">专长：{selectedLesson.teacher.specialties.join(', ')}</Text>
                      </div>
                    )}
                  </div>
                )}

                {selectedLesson.location && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>上课地点：</Text>
                    <div>
                      <Text>{selectedLesson.location.name}</Text>
                      <Text type="secondary"> (容量: {selectedLesson.location.capacity}人)</Text>
                    </div>
                    {selectedLesson.location.description && (
                      <div style={{ marginTop: '4px' }}>
                        <Text type="secondary">{selectedLesson.location.description}</Text>
                      </div>
                    )}
                  </div>
                )}

                <div style={{ marginBottom: '16px' }}>
                  <Text strong>人数：</Text>
                  <span>
                    {selectedLesson.current_students} / {selectedLesson.max_students}
                  </span>
                </div>

                {selectedLesson.price && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>价格：</Text>
                    <Text>¥{selectedLesson.price}</Text>
                  </div>
                )}

                {selectedLesson.equipment_required && selectedLesson.equipment_required.length > 0 && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>所需设备：</Text>
                    <div>
                      {selectedLesson.equipment_required.map((equipment, index) => (
                        <Tag key={index}>{equipment}</Tag>
                      ))}
                    </div>
                  </div>
                )}

                {selectedLesson.prerequisites && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>先决条件：</Text>
                    <div>{selectedLesson.prerequisites}</div>
                  </div>
                )}

                {selectedLesson.cancellation_policy && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>取消政策：</Text>
                    <div>{selectedLesson.cancellation_policy}</div>
                  </div>
                )}

                {selectedLesson.notes && (
                  <div style={{ marginBottom: '16px' }}>
                    <Text strong>备注：</Text>
                    <div>{selectedLesson.notes}</div>
                  </div>
                )}
              </Card>
            </div>
          )}
        </Spin>
      </Modal>

      {/* Custom styling for ProList items */}
      <style jsx global>{`
        .custom-lessons-list .ant-list .ant-list-item {
          padding: 12px 16px !important;
          border-radius: 10px !important;
          margin-bottom: 8px;
          background: #fff;
          border: 1px solid #f0f0f0;
          transition: all 0.2s;
        }
        
        .custom-lessons-list .ant-list .ant-list-item:hover {
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
          border-color: #d9d9d9;
        }
      `}</style>
    </div>
  );
};

export default LessonsListPage;