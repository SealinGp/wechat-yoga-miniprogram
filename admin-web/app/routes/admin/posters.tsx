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
  DatePicker,
  message,
  Popconfirm,
  Image
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import dayjs from 'dayjs';
import { api, mockApi } from '../../utils/auth';

const apiClient = api;

const { TextArea } = Input;
const { RangePicker } = DatePicker;

interface Poster {
  id: number;
  title?: string;
  image: string;
  link_url?: string;
  sort_order: number;
  is_active?: boolean;
  start_date?: string;
  end_date?: string;
  created_at?: string;
}

export default function Posters() {
  const [posters, setPosters] = useState<Poster[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingPoster, setEditingPoster] = useState<Poster | null>(null);
  const [form] = Form.useForm();

  const fetchPosters = async () => {
    try {
      setLoading(true);
      const response = await apiClient.get('/admin/posters');
      setPosters(response.data);
    } catch (error) {
      message.error('获取轮播图列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPosters();
  }, []);

  const handleCreate = () => {
    setEditingPoster(null);
    form.resetFields();
    form.setFieldsValue({
      sort_order: posters.length + 1,
      is_active: true,
    });
    setModalVisible(true);
  };

  const handleEdit = (record: Poster) => {
    setEditingPoster(record);
    const formData = {
      ...record,
      date_range: record.start_date && record.end_date 
        ? [dayjs(record.start_date), dayjs(record.end_date)]
        : undefined,
    };
    form.setFieldsValue(formData);
    setModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await apiClient.delete(`/admin/posters/${id}`);
      message.success('删除成功');
      fetchPosters();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      
      // Process date range
      const processedValues = {
        ...values,
        start_date: values.date_range?.[0]?.format('YYYY-MM-DD HH:mm:ss'),
        end_date: values.date_range?.[1]?.format('YYYY-MM-DD HH:mm:ss'),
      };
      
      delete processedValues.date_range;
      
      if (editingPoster) {
        await apiClient.put(`/admin/posters/${editingPoster.id}`, processedValues);
        message.success('更新成功');
      } else {
        await apiClient.post('/admin/posters', processedValues);
        message.success('创建成功');
      }
      
      setModalVisible(false);
      fetchPosters();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '预览',
      dataIndex: 'image',
      key: 'image',
      render: (image: string) => (
        <Image
          src={`http://localhost:8002/images/${image}`}
          alt="轮播图"
          width={80}
          height={60}
          style={{ objectFit: 'cover' }}
          fallback="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMIAAADDCAYAAADQvc6UAAABRWlDQ1BJQ0MgUHJvZmlsZQAAKJFjYGASSSwoyGFhYGDIzSspCnJ3UoiIjFJgf8LAwSDCIMogwMCcmFxc4BgQ4ANUwgCjUcG3awyMIPqyLsis7PPOq3QdDFcvjV3jOD1boQVTPQrgSkktTgbSf4A4LbmgqISBgTEFyFYuLykAsTuAbJEioKOA7DkgdjqEvQHEToKwj4DVhAQ5A9k3gGyB5IxEoBmML4BsnSQk8XQkNtReEOBxcfXxUQg1Mjc0dyHgXNJBSWpFCYh2zi+oLMpMzyhRcASGUqqCZ16yno6CkYGRAQMDKMwhqj/fAIcloxgHQqxAjIHBEugw5sUIsSQpBobtQPdLciLEVJYzMPBHMDBsayhILEqEO4DxG0txmrERhM29nYGBddr//5/DGRjYNRkY/l7////39v///y4Dmn+LgeHANwDrkl1AuO+pmgAAADhlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAAqACAAQAAAABAAAAwqADAAQAAAABAAAAwwAAAAD9b/HnAAAHlklEQVR4Ae3dP3Ik1RnG4W+FgYxN"
        />
      ),
    },
    {
      title: '标题',
      dataIndex: 'title',
      key: 'title',
    },
    {
      title: '图片文件',
      dataIndex: 'image',
      key: 'image',
      ellipsis: true,
    },
    {
      title: '跳转链接',
      dataIndex: 'link_url',
      key: 'link_url',
      ellipsis: true,
    },
    {
      title: '排序',
      dataIndex: 'sort_order',
      key: 'sort_order',
      width: 80,
    },
    {
      title: '有效期',
      key: 'validity',
      render: (_, record: Poster) => {
        if (!record.start_date && !record.end_date) {
          return '永久有效';
        }
        const start = record.start_date ? dayjs(record.start_date).format('MM-DD') : '';
        const end = record.end_date ? dayjs(record.end_date).format('MM-DD') : '';
        return `${start} ~ ${end}`;
      },
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (active: boolean) => (
        <Switch checked={active} disabled />
      ),
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record: Poster) => (
        <Space>
          <Button
            type="primary"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确定要删除这张轮播图吗？"
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
        title="轮播图管理"
        extra={
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={handleCreate}
          >
            新增轮播图
          </Button>
        }
      >
        <Table
          columns={columns}
          dataSource={posters}
          rowKey="id"
          loading={loading}
          pagination={{ pageSize: 10 }}
        />
      </Card>

      <Modal
        title={editingPoster ? '编辑轮播图' : '新增轮播图'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={800}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            label="标题"
            name="title"
          >
            <Input placeholder="请输入轮播图标题（可选）" />
          </Form.Item>

          <Form.Item
            label="图片文件名"
            name="image"
            rules={[{ required: true, message: '请输入图片文件名' }]}
            help="请输入存储在服务器 images 目录下的文件名，例如：banner1.jpg"
          >
            <Input placeholder="例如：banner1.jpg" />
          </Form.Item>

          <Form.Item
            label="跳转链接"
            name="link_url"
            help="用户点击轮播图时的跳转地址"
          >
            <Input placeholder="例如：/pages/booking/booking" />
          </Form.Item>

          <Form.Item
            label="排序顺序"
            name="sort_order"
            rules={[{ required: true, message: '请输入排序顺序' }]}
          >
            <InputNumber min={1} max={100} placeholder="数字越小排序越前" className="w-full" />
          </Form.Item>

          <Form.Item
            label="显示时间范围"
            name="date_range"
            help="不设置时间范围则永久有效"
          >
            <RangePicker 
              showTime 
              format="YYYY-MM-DD HH:mm:ss"
              placeholder={['开始时间', '结束时间']}
              className="w-full"
            />
          </Form.Item>

          <Form.Item
            label="是否启用"
            name="is_active"
            valuePropName="checked"
          >
            <Switch />
          </Form.Item>
        </Form>
      </Modal>
    </>
  );
}