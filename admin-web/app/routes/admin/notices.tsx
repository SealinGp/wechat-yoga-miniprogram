import { useState, useEffect } from 'react';
import { 
  Table, 
  Card, 
  Button, 
  Space, 
  Modal, 
  Form, 
  Input, 
  InputNumber, 
  Switch,
  message,
  Popconfirm,
  Tag
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, EyeOutlined } from '@ant-design/icons';
import { api, mockApi } from '../../utils/auth';

const apiClient = api;

const { TextArea } = Input;

interface Notice {
  id: number;
  title: string;
  content: string;
  author?: string;
  priority?: number;
  is_active?: boolean;
  created_at?: string;
}

export default function Notices() {
  const [notices, setNotices] = useState<Notice[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [viewModalVisible, setViewModalVisible] = useState(false);
  const [editingNotice, setEditingNotice] = useState<Notice | null>(null);
  const [viewingNotice, setViewingNotice] = useState<Notice | null>(null);
  const [form] = Form.useForm();

  const fetchNotices = async () => {
    try {
      setLoading(true);
      const response = await apiClient.get('/admin/notices');
      setNotices(response.data);
    } catch (error) {
      message.error('获取公告列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchNotices();
  }, []);

  const handleCreate = () => {
    setEditingNotice(null);
    form.resetFields();
    form.setFieldsValue({
      priority: 0,
      is_active: true,
      author: '管理员',
    });
    setModalVisible(true);
  };

  const handleEdit = (record: Notice) => {
    setEditingNotice(record);
    form.setFieldsValue(record);
    setModalVisible(true);
  };

  const handleView = (record: Notice) => {
    setViewingNotice(record);
    setViewModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await apiClient.delete(`/admin/notices/${id}`);
      message.success('删除成功');
      fetchNotices();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      
      if (editingNotice) {
        await apiClient.put(`/admin/notices/${editingNotice.id}`, values);
        message.success('更新成功');
      } else {
        await apiClient.post('/admin/notices', values);
        message.success('创建成功');
      }
      
      setModalVisible(false);
      fetchNotices();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const getPriorityColor = (priority: number) => {
    if (priority >= 8) return 'red';
    if (priority >= 5) return 'orange';
    if (priority >= 2) return 'blue';
    return 'default';
  };

  const getPriorityText = (priority: number) => {
    if (priority >= 8) return '高';
    if (priority >= 5) return '中';
    if (priority >= 2) return '低';
    return '普通';
  };

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '标题',
      dataIndex: 'title',
      key: 'title',
      ellipsis: true,
    },
    {
      title: '内容预览',
      dataIndex: 'content',
      key: 'content',
      ellipsis: true,
      render: (content: string) => (
        <span className="text-gray-600">
          {content?.substring(0, 50)}{content?.length > 50 ? '...' : ''}
        </span>
      ),
    },
    {
      title: '作者',
      dataIndex: 'author',
      key: 'author',
      width: 100,
    },
    {
      title: '优先级',
      dataIndex: 'priority',
      key: 'priority',
      render: (priority: number) => (
        <Tag color={getPriorityColor(priority)}>
          {getPriorityText(priority)} ({priority})
        </Tag>
      ),
      width: 100,
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (active: boolean) => (
        <Switch checked={active} disabled />
      ),
      width: 80,
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (date: string) => {
        if (!date) return '-';
        return new Date(date).toLocaleString();
      },
      width: 160,
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record: Notice) => (
        <Space>
          <Button
            size="small"
            icon={<EyeOutlined />}
            onClick={() => handleView(record)}
          >
            查看
          </Button>
          <Button
            type="primary"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确定要删除这条公告吗？"
            onConfirm={() => handleDelete(record.id)}
            okText="确定"
            cancelText="取消"
          >
            <Button
              type="primary"
              danger
              size="small"
              icon={<DeleteOutlined />}
            >
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <>
      <Card 
        title="公告管理"
        extra={
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={handleCreate}
          >
            发布公告
          </Button>
        }
      >
        <Table
          columns={columns}
          dataSource={notices}
          rowKey="id"
          loading={loading}
          pagination={{ pageSize: 10 }}
        />
      </Card>

      {/* 编辑/新建公告模态框 */}
      <Modal
        title={editingNotice ? '编辑公告' : '发布公告'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            label="公告标题"
            name="title"
            rules={[{ required: true, message: '请输入公告标题' }]}
          >
            <Input placeholder="请输入公告标题" />
          </Form.Item>

          <Form.Item
            label="公告内容"
            name="content"
            rules={[{ required: true, message: '请输入公告内容' }]}
          >
            <TextArea 
              rows={6} 
              placeholder="请输入公告内容"
              showCount
              maxLength={1000}
            />
          </Form.Item>

          <Form.Item
            label="发布者"
            name="author"
          >
            <Input placeholder="请输入发布者姓名" />
          </Form.Item>

          <Form.Item
            label="优先级"
            name="priority"
            help="数值越高优先级越高，显示顺序越靠前。推荐：紧急(8-10)，重要(5-7)，一般(2-4)，普通(0-1)"
          >
            <InputNumber 
              min={0} 
              max={10} 
              placeholder="请输入优先级(0-10)" 
              className="w-full" 
            />
          </Form.Item>

          <Form.Item
            label="是否发布"
            name="is_active"
            valuePropName="checked"
          >
            <Switch />
          </Form.Item>
        </Form>
      </Modal>

      {/* 查看公告模态框 */}
      <Modal
        title="查看公告"
        open={viewModalVisible}
        onCancel={() => setViewModalVisible(false)}
        footer={[
          <Button key="close" onClick={() => setViewModalVisible(false)}>
            关闭
          </Button>,
          <Button 
            key="edit" 
            type="primary" 
            onClick={() => {
              setViewModalVisible(false);
              if (viewingNotice) {
                handleEdit(viewingNotice);
              }
            }}
          >
            编辑
          </Button>,
        ]}
        width={800}
      >
        {viewingNotice && (
          <div className="space-y-4">
            <div>
              <h3 className="text-lg font-semibold mb-2">{viewingNotice.title}</h3>
              <div className="flex items-center space-x-4 text-sm text-gray-500 mb-4">
                <span>作者：{viewingNotice.author || '未知'}</span>
                <span>优先级：
                  <Tag color={getPriorityColor(viewingNotice.priority || 0)}>
                    {getPriorityText(viewingNotice.priority || 0)}
                  </Tag>
                </span>
                <span>状态：{viewingNotice.is_active ? '已发布' : '未发布'}</span>
                {viewingNotice.created_at && (
                  <span>创建时间：{new Date(viewingNotice.created_at).toLocaleString()}</span>
                )}
              </div>
            </div>
            <div className="bg-gray-50 p-4 rounded">
              <pre className="whitespace-pre-wrap font-sans">{viewingNotice.content}</pre>
            </div>
          </div>
        )}
      </Modal>
    </>
  );
}