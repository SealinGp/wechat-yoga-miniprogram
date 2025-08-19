import React, { useState, useEffect } from 'react';
import { Calendar, Modal, Card, Typography, Tag, Spin, message, ConfigProvider } from 'antd';
import type { CalendarProps } from 'antd';
import type { Dayjs } from 'dayjs';
import dayjs from 'dayjs';
import zhCN from 'antd/locale/zh_CN';
import 'dayjs/locale/zh-cn';
import { useNavigate } from 'react-router';
import { getLessonTypeStr, getDifficultyLevelStr, type Lesson } from '~/utils/lesson';

const { Title, Text } = Typography;

// Set dayjs locale to Chinese
dayjs.locale('zh-cn');

const LessonsPage: React.FC = () => {
  const navigate = useNavigate();
  const [lessons, setLessons] = useState<Lesson[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedLesson, setSelectedLesson] = useState<Lesson | null>(null);
  const [modalVisible, setModalVisible] = useState(false);
  const [modalLoading, setModalLoading] = useState(false);

  // Fetch lessons data
  const fetchLessons = async () => {
    setLoading(true);
    try {
      // Call the list_lessons API (admin_lessons endpoint)
      // For calendar view, get a broader date range (e.g., current month)
      const startOfMonth = dayjs().startOf('month').unix();
      const endOfMonth = dayjs().endOf('month').unix();
      const response = await fetch(`/api/admin/lessons?start=${startOfMonth}&end=${endOfMonth}&limit=1000&offset=0`);
      if (!response.ok) {
        throw new Error('Failed to fetch lessons');
      }
      const data = await response.json();
      setLessons(Array.isArray(data) ? data : []);
    } catch (error) {
      console.error('Error fetching lessons:', error);
      message.error('Failed to load lessons');
      setLessons([]);
    } finally {
      setLoading(false);
    }
  };

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
  }, []);

  // Group lessons by date for calendar display
  const getLessonsForDate = (date: Dayjs): Lesson[] => {
    const dateStr = date.format('YYYY-MM-DD');
    return lessons.filter(lesson => {
      const lessonDate = dayjs(lesson.start_time).format('YYYY-MM-DD');
      return lessonDate === dateStr;
    });
  };

  // Custom date cell render for calendar
  const dateCellRender: CalendarProps<Dayjs>['dateCellRender'] = (value) => {
    const dayLessons = getLessonsForDate(value);

    if (dayLessons.length === 0) return null;

    return (
      <div className="lessons-cell">
        {dayLessons.map((lesson) => (
          <div
            key={lesson.id}
            className="lesson-item"
            onClick={(e) => {
              e.stopPropagation();
              e.preventDefault();
              handleLessonClick(lesson.id);
            }}
            style={{
              background: '#f0f2f5',
              margin: '2px 0',
              padding: '2px 4px',
              borderRadius: '2px',
              fontSize: '12px',
              cursor: 'pointer',
              borderLeft: '3px solid #1890ff',
            }}
          >
            <div style={{ fontWeight: 'bold', color: '#1890ff' }}>
              {lesson.title}
            </div>
            {lesson.teacher && (
              <div style={{ color: '#666' }}>
                {lesson.teacher.name}
              </div>
            )}
            <div style={{ color: '#999' }}>
              {dayjs(lesson.start_time).format('HH:mm')} - {dayjs(lesson.end_time).format('HH:mm')}
            </div>
          </div>
        ))}
      </div>
    );
  };

  // Handle lesson click
  const handleLessonClick = async (lessonId: number) => {
    setModalVisible(true);
    await fetchLessonDetail(lessonId);
  };

  // Handle calendar cell click to navigate to lessons list
  const handleCellClick = (date: Dayjs) => {
    const dateString = date.format('YYYY-MM-DD');
    navigate(`/admin/lessons-list?date=${dateString}`);
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

  const getMonthData = (value: Dayjs) => {
    if (value.month() === 8) {
      return 1394;
    }
  };

  const monthCellRender = (value: Dayjs) => {
    const num = getMonthData(value);
    return num ? (
      <div className="notes-month">
        <section>{num}</section>
        <span>Backlog number</span>
      </div>
    ) : null;
  }

  const cellRender: CalendarProps<Dayjs>['cellRender'] = (current, info) => {
    if (info.type === 'date') return dateCellRender(current);
    if (info.type === 'month') return monthCellRender(current);
    return info.originNode;
  };

  return (
    <ConfigProvider locale={zhCN}>
      <div style={{ padding: '24px' }}>
        <Title level={2}>课程管理</Title>

        <Spin spinning={loading}>
          <Calendar
            cellRender={cellRender}
            onSelect={handleCellClick}
            style={{
              background: '#fff',
              borderRadius: '6px',
              padding: '16px',
            }}
          />
        </Spin>

      {/* Lesson Detail Modal */}
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


      <style jsx>{`
        .lessons-cell {
          max-height: 60px;
          overflow-y: auto;
        }
        
        .lesson-item:hover {
          background: #e6f7ff !important;
          transform: scale(1.02);
          transition: all 0.2s;
        }
        
        .ant-picker-calendar .ant-picker-calendar-date-content {
          height: 80px;
          overflow: hidden;
        }
      `}</style>
      </div>
    </ConfigProvider>
  );
};

export default LessonsPage;